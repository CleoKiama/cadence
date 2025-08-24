use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::{
    collections::HashSet,
    path::Path,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Manager};

#[derive(Debug)]
pub enum WatchCommand {
    Watch(String),
    Unwatch(String),
    Stop,
}

pub fn start_watcher(
    app_handle: AppHandle,
    dir: Option<String>,
) -> notify::Result<mpsc::Sender<WatchCommand>> {
    let (event_tx, event_rx) = mpsc::channel();
    let (cmd_tx, cmd_rx) = mpsc::channel::<WatchCommand>();

    let mut watcher = recommended_watcher(move |res| {
        let _ = event_tx.send(res);
    })
    .map_err(|e| {
        eprintln!("Failed to create watcher: {:?}", e);
        e
    })?;

    // Watch the initial directory if provided
    if let Some(dir) = &dir {
        let path = Path::new(dir);
        if let Err(e) = watcher.watch(path, RecursiveMode::NonRecursive) {
            eprintln!("Failed to watch initial directory {}: {:?}", dir, e);
            return Err(e);
        }
    }

    // Watcher command handler thread
    thread::spawn(move || -> anyhow::Result<()> {
        while let Ok(cmd) = cmd_rx.recv() {
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

    // Event processor thread with buffering
    thread::spawn(move || {
        let max_buffer_size = 20;
        let mut event_buffer: HashSet<String> = HashSet::with_capacity(max_buffer_size);
        let mut last_flush = Instant::now();
        let flush_interval = Duration::from_millis(1000);
        let tx = app_handle.state::<mpsc::SyncSender<String>>().clone();

        while let Ok(event_result) = event_rx.recv() {
            match event_result {
                Ok(event) => {
                    match event.kind {
                        // TODO:: handle remove event too
                        notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                            for path in &event.paths {
                                if path.is_file()
                                    && path.extension().and_then(|s| s.to_str()) == Some("md")
                                {
                                    if let Some(path_str) = path.to_str() {
                                        event_buffer.insert(path_str.to_string());
                                    }
                                }
                            }

                            // Flush buffer if it's time or if buffer is full
                            let should_flush = last_flush.elapsed() >= flush_interval
                                || event_buffer.len() >= max_buffer_size;

                            if should_flush && !event_buffer.is_empty() {
                                for path in event_buffer.drain() {
                                    match tx.send(path) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            eprintln!(
                                                "Failed to send file path to sync worker: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                                last_flush = Instant::now();
                            }
                        }
                        _ => continue, // Ignore other event kinds
                    }
                }
                Err(err) => eprintln!("Watcher error: {err}"),
            }
        }

        // Process any remaining events
        if !event_buffer.is_empty() {
            for path in event_buffer.drain() {
                match tx.send(path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to send file path to sync worker: {}", e);
                    }
                }
            }
        }
    });

    Ok(cmd_tx)
}
