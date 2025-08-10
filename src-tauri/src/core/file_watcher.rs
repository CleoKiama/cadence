use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
use rusqlite::Connection;
use std::{
    collections::HashSet,
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

pub async fn start_watcher(
    db: Arc<Mutex<Connection>>,
    dir: String,
    needed_metrics: &Vec<String>,
) -> notify::Result<()> {
    let (tx, mut rx) = mpsc::channel(100);

    // Blocking watcher task
    tauri::async_runtime::spawn_blocking(move || {
        move || -> notify::Result<()> {
            let mut watcher = recommended_watcher(move |res| {
                let _ = tx.blocking_send(res);
            })?;
            watcher.watch(Path::new(&dir), RecursiveMode::Recursive)?;
            loop {
                std::thread::park(); // Keep it alive
            }
        }
    });

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                if let Err(e) = sync_data(event, db.clone(), needed_metrics).await {
                    eprintln!("Error syncing data: {}", e);
                }
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }

    Ok(())
}

pub async fn sync_data(
    event: Event,
    db: Arc<Mutex<Connection>>,
    needed_metrics: &Vec<String>,
) -> Result<()> {
    let mut visited_paths: HashSet<String> = HashSet::new();
    for path in event.paths {
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let file_path = path.to_str();
            if let Some(file_path) = file_path {
                if visited_paths.contains(file_path) {
                    println!("Skipping already visited file: {}", file_path);
                    continue;
                }
                match super::read_journal::read_front_matter(file_path, needed_metrics, db.clone())
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
