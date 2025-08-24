use anyhow::Context;
use chrono::{DateTime, Utc};
use rusqlite::params;
use std::fs;
use tauri::{AppHandle, Manager};

use crate::DbConnection;

pub fn read_dailies_dir(
    dir_path: String,
    app_handle: AppHandle,
) -> Result<Vec<String>, anyhow::Error> {
    let dir_entries = fs::read_dir(dir_path).with_context(|| "Failed to read the directory")?;

    let mut file_paths = Vec::new();
    let db = app_handle.state::<DbConnection>();

    for entry in dir_entries {
        let entry = entry?;
        if let Some(path) = read_entry(entry, &db)? {
            file_paths.push(path);
        }
    }

    Ok(file_paths)
}

fn read_entry(dir_entry: fs::DirEntry, db: &DbConnection) -> Result<Option<String>, anyhow::Error> {
    let path = dir_entry.path();
    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if file_name.starts_with("202") {
                let metadata = dir_entry.metadata().with_context(|| {
                    format!("Failed to get metadata for file: {}", path.display())
                })?;
                let metadata = metadata.modified().with_context(|| {
                    format!(
                        "Failed to get last modified time for file: {}",
                        path.display()
                    )
                })?;
                let datetime: DateTime<Utc> = metadata.into();
                let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

                db.lock().unwrap().execute(
                    "INSERT OR REPLACE INTO file_meta (file_path, last_modified) VALUES (?1, ?2)",
                    params![path.to_string_lossy().to_string(), formatted_time],
                )
                .with_context(|| {
                    format!(
                        "Failed to insert or update file metadata for {}",
                        path.display()
                    )
                })?;

                return Ok(Some(path.to_string_lossy().to_string()));
            }
        }
    }
    Ok(None)
}
