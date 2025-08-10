// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{sleep, Duration};

mod core;
mod db;

use db::Db;

#[derive(Clone, Serialize)]
struct DownloadProgress {
    url: String,
    progress: u8,
}

async fn download(app: AppHandle) {
    let url = "https://example.com/file.zip".to_string();
    for i in 0..=100 {
        app.emit(
            "download-progress",
            DownloadProgress {
                url: url.clone(),
                progress: i as u8,
            },
        )
        .unwrap();
        sleep(Duration::from_millis(700)).await;
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn start_download(app: AppHandle) {
    println!("Starting download");
    download(app).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            println!("initializing the database");
            let db = Db::new("/tmp/habitron.db")?;
            if let Err(err) = db.init_db() {
                eprintln!("Failed to initialize the database: {}", err);
            }
            app.manage(db.get_connection());

            tauri::async_runtime::spawn({
                let db_clone = db.get_connection();
                async move {
                    if let Err(e) = core::init(db_clone).await {
                        eprintln!("Error during core initialization: {}", e);
                    }
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_download,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
