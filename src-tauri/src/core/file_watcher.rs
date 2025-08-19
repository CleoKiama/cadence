use anyhow::Result;
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use rusqlite::Connection;
use std::{
    collections::HashSet,
    path::Path,
    sync::{Arc, Mutex},
};
use tauri::async_runtime::{channel, Sender};

#[derive(Debug)]
pub enum WatchCommand {
    Watch(String),
    Unwatch(String),
    Stop,
}

pub async fn start_watcher(
    db: Arc<Mutex<Connection>>,
    dir: Option<String>,
) -> notify::Result<Sender<WatchCommand>> {
    let (event_tx, mut event_rx) = channel(100);
    let (cmd_tx, mut cmd_rx) = channel::<WatchCommand>(10);

    let mut watcher = recommended_watcher(move |res| {
        let _ = event_tx.blocking_send(res);
    })
    .map_err(|e| {
        eprintln!("Failed to create watcher: {:?}", e);
        e
    })?;

    // Watch the initial directory if provided
    if let Some(dir) = &dir {
        let path = Path::new(dir);
        if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
            eprintln!("Failed to watch initial directory {}: {:?}", dir, e);
            return Err(e); // Fail early: don't proceed if initial watch fails
        }
    }

    tauri::async_runtime::spawn_blocking(move || -> anyhow::Result<()> {
        while let Some(cmd) = cmd_rx.blocking_recv() {
            match cmd {
                WatchCommand::Watch(path) => {
                    println!("Watching {}", path);
                    if let Err(e) = watcher.watch(Path::new(&path), RecursiveMode::NonRecursive) {
                        eprintln!("Failed to watch {}: {}", path, e);
                    }
                }
                WatchCommand::Unwatch(path) => {
                    println!("Unwatching {}", path);
                    if let Err(e) = watcher.unwatch(Path::new(&path)) {
                        eprintln!("Failed to unwatch {}: {}", path, e);
                    }
                }
                WatchCommand::Stop => {
                    println!("Stopping watcher");
                    break;
                }
            }
        }
        Ok(())
    });

    // Event handler task
    let db_clone = db.clone();
    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            match event {
                Ok(event) => {
                    if let Err(e) = sync_data(event, db_clone.clone()).await {
                        eprintln!("Error syncing data: {e}");
                    }
                }
                Err(err) => eprintln!("Watcher error: {err}"),
            }
        }
    });

    Ok(cmd_tx)
}

pub async fn sync_data(event: Event, db: Arc<Mutex<Connection>>) -> Result<()> {
    let mut visited_paths: HashSet<String> = HashSet::new();
    let needed_metrics = Vec::from([
        "dsa_problems_solved",
        "exercise",
        "reading",
        "study",
        "workout",
    ]);
    for path in event.paths {
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let file_path = path.to_str();
            if let Some(file_path) = file_path {
                if visited_paths.contains(file_path) {
                    println!("Skipping already visited file: {}", file_path);
                    continue;
                }
                match super::read_journal::read_front_matter(file_path, &needed_metrics, db.clone())
                    .await
                {
                    Ok(_) => println!("Successfully processed: {}", file_path),
                    Err(e) => eprintln!("Error processing {}: {}", file_path, e),
                }

                visited_paths.insert(file_path.to_string());
            }
        }
    }
    Ok(())
}
