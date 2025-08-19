use anyhow::Context;
use chrono::{DateTime, Utc};
use rusqlite::params;
use tokio;

use crate::DbConnection;

pub async fn read_dailies_dir(
    dir_path: &str,
    db: &DbConnection,
) -> Result<Vec<String>, anyhow::Error> {
    let mut paths = Vec::new();
    let mut dir_entries = tokio::fs::read_dir(dir_path)
        .await
        .with_context(|| format!("Failed to read the directory: {}", dir_path))?;
    while let Some(entry) = dir_entries.next_entry().await? {
        read_entry(entry, db, &mut paths).await?;
    }
    Ok(paths)
}

async fn read_entry(
    dir_entry: tokio::fs::DirEntry,
    db: &DbConnection,
    paths: &mut Vec<String>,
) -> Result<(), anyhow::Error> {
    let path = dir_entry.path();
    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if file_name.starts_with("202") {
                let metadata = dir_entry.metadata().await.with_context(|| {
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

                paths.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}
