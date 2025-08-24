use std::sync::{mpsc, Arc, Mutex};

use tauri::{AppHandle, Manager};

use crate::{
    core::{get_tracked_metrics_from_db, read_journal::read_front_matter},
    DbConnection,
};

const WORKER_CHANNEL_CAPACITY: usize = 4;

pub fn setup_sync_worker(app_handle: AppHandle) -> mpsc::SyncSender<String> {
    let (tx, rx) = mpsc::sync_channel::<String>(WORKER_CHANNEL_CAPACITY);
    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..WORKER_CHANNEL_CAPACITY {
        let rx = Arc::clone(&rx);
        let app = app_handle.clone();
        std::thread::spawn(move || loop {
            let msg = {
                let lock = rx.lock().unwrap();
                lock.recv()
            };
            let db = app.state::<DbConnection>();
            match msg {
                Ok(file_path) => {
                    let tracked_metrics = get_tracked_metrics_from_db(&db);
                    let needed_metrics = match tracked_metrics {
                        Ok(metrics) => metrics,
                        Err(e) => {
                            eprintln!("Error fetching tracked metrics: {}", e);
                            continue;
                        }
                    };
                    if needed_metrics.is_empty() {
                        continue;
                    }
                    match read_front_matter(&file_path, &needed_metrics, &db) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("Error processing {}: {}", file_path, e);
                        }
                    };
                }
                Err(e) => {
                    eprintln!("Worker thread error: {}", e);
                    break;
                }
            }
        });
    }
    tx
}
