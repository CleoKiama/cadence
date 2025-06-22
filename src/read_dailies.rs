use anyhow::Context;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};

pub fn read_dailies_dir(dir_path: &str, conn: &Connection) -> Result<Vec<String>, anyhow::Error> {
    let mut paths = Vec::new();
    let dir_entries = std::fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read the directory: {}", dir_path))?;
    for entry in dir_entries {
        let entry = entry.with_context(|| format!("failed to read directory {}", dir_path))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.starts_with("202") {
                    let metadata = entry.metadata().with_context(|| {
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

                    conn.execute(
                        "INSERT OR REPLACE INTO file_meta (file_path, last_modified) VALUES (?1, ?2)",
                        params![path.to_string_lossy().to_string(),formatted_time],
                    ).with_context(|| {
                        format!(
                            "Failed to insert or update file metadata for {}",
                            path.display()
                        )
                    })?;

                    paths.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(paths)
}
