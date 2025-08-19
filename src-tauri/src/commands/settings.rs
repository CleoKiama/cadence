use std::path::Path;
use std::sync::{Arc, Mutex};

use chrono::{Local, NaiveDate};
use rusqlite::Connection;
use serde::Serialize;

use crate::core::read_journal::DB_DATE_TIME_FORMAT;
use crate::db::utils::get_journal_files_path;
use crate::Watcher;

const JOURNAL_FILES_PATH_KEY: &str = "journal_files_path";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedMetric {
    name: String,
    active: bool,
    last_updated: String,
    entries: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    tracked_metrics: Option<Vec<TrackedMetric>>,
    journal_files_path: Option<String>,
}

#[tauri::command]
pub fn get_settings(
    state: tauri::State<'_, Arc<Mutex<rusqlite::Connection>>>,
) -> Result<Settings, String> {
    let tracked_metrics = get_tracked_metrics(&state).map_err(|e| e.to_string())?;
    let journal_files_path = get_journal_files_path(&state).map_err(|e| e.to_string())?;

    Ok(Settings {
        tracked_metrics,
        journal_files_path,
    })
}

#[tauri::command]
pub async fn set_journal_files_path(
    state: tauri::State<'_, Arc<Mutex<rusqlite::Connection>>>,
    watch_command: tauri::State<'_, Watcher>,
    path: &str,
) -> Result<(), String> {
    // Validate path exists and is accessible
    if path.is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    if !Path::new(path).exists() {
        return Err("Path does not exist".to_string());
    }

    if !Path::new(path).is_dir() {
        return Err("Path must be a directory".to_string());
    }

    // Save to database
    {
        let conn = state
            .lock()
            .map_err(|e| format!("Failed to lock connection: {}", e))?;
        let mut stmt = conn
            .prepare("INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        stmt.execute([JOURNAL_FILES_PATH_KEY, path])
            .map_err(|e| format!("Failed to set journal files path: {}", e))?;
    }

    println!("path: {:#?}", path);

    Ok(())
}

fn get_tracked_metrics(
    conn: &Arc<Mutex<Connection>>,
) -> Result<Option<Vec<TrackedMetric>>, anyhow::Error> {
    let conn = conn
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT name, updated_at, COUNT(*) as entries 
         FROM metrics 
         GROUP BY name 
         ORDER BY updated_at DESC",
    )?;

    let tracked_metrics: Vec<TrackedMetric> = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let updated_at: String = row.get(1)?;
            let entries: i32 = row.get(2)?;

            Ok((name, updated_at, entries))
        })?
        .map(|res| {
            let (name, updated_at, entries) = res?;
            let last_updated = NaiveDate::parse_from_str(&updated_at, DB_DATE_TIME_FORMAT)
                .map_err(|e| anyhow::anyhow!("Failed to parse date {}: {}", updated_at, e))?;

            let today = Local::now().date_naive();
            let last_seven_days = today
                .checked_sub_signed(chrono::Duration::days(7))
                .ok_or_else(|| anyhow::anyhow!("Failed to calculate last seven days"))?;

            let active = last_updated >= last_seven_days;

            Ok(TrackedMetric {
                name,
                active,
                last_updated: updated_at,
                entries,
            })
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    if tracked_metrics.is_empty() {
        Ok(None)
    } else {
        Ok(Some(tracked_metrics))
    }
}
