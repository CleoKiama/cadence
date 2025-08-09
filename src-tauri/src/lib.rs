// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::time::{sleep, Duration};

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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
