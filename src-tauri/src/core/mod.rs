use serde::Serialize;
use std::{sync::mpsc, time::Duration};
use tauri::{AppHandle, Emitter, Manager};

pub mod file_watcher;
pub mod read_dailies;
pub mod read_journal;
pub mod sync_worker;

use read_dailies::read_dailies_dir;

use crate::{core::file_watcher::WatchCommand, DbConnection};

fn get_tracked_metrics_from_db(db: &DbConnection) -> Result<Vec<String>, anyhow::Error> {
    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock connection: {}", e))?;

    let mut stmt = conn.prepare("SELECT value FROM tracked_metrics")?;
    let metric_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;

    let mut metrics = Vec::new();
    for metric in metric_iter {
        metrics.push(metric?);
    }

    Ok(metrics)
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SyncProgress {
    sync_progress: f32,
}

pub async fn resync_database(
    app_handle: AppHandle,
    journal_path: &str,
) -> Result<(), anyhow::Error> {
    let db = app_handle.state::<DbConnection>();
    let tx = app_handle.state::<mpsc::SyncSender<String>>().clone();
    let tracked_metrics = get_tracked_metrics_from_db(&db)?;

    if tracked_metrics.is_empty() {
        return Ok(());
    }

    match app_handle.emit("sync-start", "") {
        Ok(_) => (),
        Err(e) => println!("Failed to emit sync-start event: {}", e),
    }

    let (progress_tx, mut progress_rx) = tokio::sync::mpsc::channel::<f32>(100);
    let app_handle_clone = app_handle.clone();

    tauri::async_runtime::spawn(async move {
        let mut latest: Option<f32> = None;
        let mut ticker = tokio::time::interval(Duration::from_millis(500));
        loop {
            tokio::select! {
                progress = progress_rx.recv() => {
                    match progress {
                        Some(p) => {
                            latest = Some(p);
                        }
                        None => {
                            // Channel closed - send final progress and break
                            println!("Progress channel closed, ending progress emitter");
                            let _ = app_handle_clone.emit("sync-complete", ());
                            break;
                        }
                    }
                }
                _ = ticker.tick() => {
                    if let Some(progress) = latest.take() {
                        match app_handle_clone.emit("sync-progress", SyncProgress { sync_progress: progress }) {
                            Ok(_) => println!("Emitted progress: {:.2}%", progress),
                            Err(e) => eprintln!("Failed to emit progress event: {}", e),
                        }
                    }
                }
            }
        }
    });

    let app_handle_clone = app_handle.clone();
    let jouornal_path_clone = journal_path.to_string();
    let handle = tauri::async_runtime::spawn_blocking(move || {
        read_dailies_dir(jouornal_path_clone, app_handle_clone)
    });

    let file_paths = handle.await??;
    let len = file_paths.len();
    for (i, path) in file_paths.into_iter().enumerate() {
        let progress = ((i + 1) as f32 / len as f32) * 100.0;
        if tx.send(path).is_ok() {
            let _ = progress_tx.send(progress.round()).await;
        }
    }

    drop(progress_tx);

    Ok(())
}

pub fn init(
    app_handle: AppHandle,
    journal_path: Option<String>,
) -> Result<mpsc::Sender<WatchCommand>, anyhow::Error> {
    let db = app_handle.state::<DbConnection>();

    let tracked_metrics = get_tracked_metrics_from_db(&db)?;
    let tx = app_handle.state::<mpsc::SyncSender<String>>().clone();

    if let Some(root_dir) = &journal_path {
        if !tracked_metrics.is_empty() {
            let file_paths = read_dailies_dir(root_dir.clone(), app_handle.clone())?;
            for path in file_paths {
                match tx.clone().send(path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to send file path to sync worker: {}", e);
                    }
                }
            }
        }
    };

    let watcher = file_watcher::start_watcher(app_handle, journal_path)?;
    Ok(watcher)
}
