use std::path::Path;

use chrono::{Local, NaiveDate};
use rusqlite::{fallible_iterator::FallibleIterator, Batch, Result};
use serde::Serialize;

use crate::{
    core::file_watcher::WatchCommand, core::read_journal::DB_DATE_TIME_FORMAT,
    core::resync_database, db::utils::get_journal_files_path, DbConnection, WatcherState,
};

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
pub fn get_settings(db: tauri::State<'_, DbConnection>) -> Result<Settings, String> {
    let tracked_metrics = get_tracked_metrics(&db).map_err(|e| e.to_string())?;
    let journal_files_path = get_journal_files_path(&db).map_err(|e| e.to_string())?;

    Ok(Settings {
        tracked_metrics,
        journal_files_path,
    })
}

#[tauri::command]
pub async fn set_journal_files_path(
    db: tauri::State<'_, DbConnection>,
    watcher: tauri::State<'_, WatcherState>,
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

    // Get previous journal path before updating
    let previous_path = get_journal_files_path(&db).map_err(|e| e.to_string())?;

    // Save to database
    {
        let conn = db
            .lock()
            .map_err(|e| format!("Failed to lock connection: {}", e))?;
        let mut stmt = conn
            .prepare("INSERT OR REPLACE INTO journals_files_path (value) VALUES (?1)")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        stmt.execute([path])
            .map_err(|e| format!("Failed to set journal files path: {}", e))?;
    }

    // Resync database with new path
    resync_database(&db, path)
        .await
        .map_err(|e| format!("Failed to resync database: {}", e))?;

    // Unwatch previous path if it exists
    if let Some(prev_path) = previous_path {
        let sender_opt = {
            let sender_guard = watcher
                .lock()
                .map_err(|e| format!("Failed to lock watcher sender: {}", e))?;
            sender_guard.clone()
        };
        if let Some(sender) = sender_opt {
            sender
                .send(WatchCommand::Unwatch(prev_path))
                .await
                .map_err(|e| format!("Failed to send unwatch command: {}", e))?;
        }
    }

    // Watch new path
    let sender_opt = {
        let sender_guard = watcher
            .lock()
            .map_err(|e| format!("Failed to lock watcher sender: {}", e))?;
        sender_guard.clone()
    };
    if let Some(sender) = sender_opt {
        sender
            .send(WatchCommand::Watch(path.to_string()))
            .await
            .map_err(|e| format!("Failed to send watch command: {}", e))?;
    }

    println!("path: {:#?}", path);

    Ok(())
}

fn get_tracked_metrics(db: &DbConnection) -> Result<Option<Vec<TrackedMetric>>, anyhow::Error> {
    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT name, updated_at, COUNT(*) as entries 
         FROM metrics 
         left join tracked_metrics on tracked_metrics.value = metrics.name
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

#[tauri::command]
pub fn delete_metric(
    db: tauri::State<'_, DbConnection>,
    metric_name: String,
) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Failed to lock connection: {}", e))?;
    let sql = r"
        DELETE FROM metrics WHERE name = ?1
        DELETE FROM tracked_metrics WHERE name = ?1
    ";
    let mut batch = Batch::new(&conn, sql);
    while let Some(mut stmt) = batch
        .next()
        .map_err(|err| format!("failed to delete metric {}", err))?
    {
        stmt.execute([metric_name.clone()])
            .map_err(|e| format!("failed to delete metric {}", e))?;
    }
    println!("Deleted metric: {}", metric_name);
    Ok(())
}

#[tauri::command]
pub fn add_metric(db: tauri::State<'_, DbConnection>, metric_name: String) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Failed to lock connection: {}", e))?;

    // Insert into tracked_metrics table
    let mut stmt = conn
        .prepare("INSERT OR REPLACE INTO tracked_metrics (value) VALUES (?1)")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    stmt.execute([metric_name.clone()])
        .map_err(|e| format!("Failed to add metric: {}", e))?;

    println!("Added metric: {}", metric_name);
    //TODO: Call re sync database after adding metric
    Ok(())
}

#[tauri::command]
pub fn udpate_metric(
    db: tauri::State<'_, DbConnection>,
    prev_name: String,
    new_name: String,
) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|e| format!("Failed to lock connection: {}", e))?;
    let sql = r"
        UPDATE  metrics
        SET name = ?1
        WHERE name = ?2;

        UPDATE tracked_metrics 
        SET value = ?1
        WHERE value = ?2
    ";
    let mut batch = Batch::new(&conn, sql);
    while let Some(mut stmt) = batch
        .next()
        .map_err(|err| format!("failed to delete metric {}", err))?
    {
        stmt.execute([new_name.clone(), prev_name.clone()])
            .map_err(|e| format!("failed to delete metric {}", e))?;
    }
    println!("updated the metric from {} to {}", prev_name, new_name);
    //TODO:: re sync database
    Ok(())
}
