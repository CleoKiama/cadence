use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio::fs::metadata;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

pub const DB_DATE_FORMAT: &str = "%Y-%m-%d";
pub const DB_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub value: u32,
    pub date: NaiveDate,
    pub file_path: String,
}

pub async fn read_front_matter(
    path: &str,
    needed_metrics: &Vec<String>,
    db: Arc<Mutex<Connection>>,
) -> Result<(), anyhow::Error> {
    if !should_read_file(path, db.clone()).await? {
        return Ok(());
    }

    let file = File::open(path).await?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut in_front_matter = false;
    while let Some(line) = lines.next_line().await? {
        if line.trim().contains("---") {
            if in_front_matter {
                break;
            } else {
                in_front_matter = true;
                continue;
            }
        }
        for metric in needed_metrics {
            if line.starts_with(metric) {
                let metric = extract_metric(&line, path)?;
                write_metric_to_db(metric, db.clone())?;
            }
        }
    }
    update_file_metadata(path, db)?;
    Ok(())
}

fn extract_metric(line: &str, path: &str) -> Result<Metric> {
    let raw_date_string = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get file name from path"))?
        .to_string();
    let date = NaiveDate::parse_from_str(&raw_date_string, DB_DATE_FORMAT).with_context(|| {
        format!(
            "Failed to parse the date from the file name {}",
            raw_date_string,
        )
    })?;

    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() == 2 {
        let name = parts[0].trim().to_string();
        let value = parts[1].trim().parse::<u32>().unwrap_or(0);
        Ok(Metric {
            name,
            value,
            date,
            file_path: path.to_string(),
        })
    } else {
        Err(anyhow::anyhow!("Invalid metric format in line {}", line))
    }
}

async fn should_read_file(path: &str, db: Arc<Mutex<Connection>>) -> Result<bool> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Ok(false);
    }

    let last_modified: SystemTime = metadata(file_path)
        .await
        .and_then(|m| m.modified())
        .map_err(|e| anyhow::anyhow!("Failed to get metadata for {}: {}", path, e))?;

    let last_modified_time: DateTime<Utc> = last_modified.into();
    let last_modified_str = last_modified_time.format(DB_DATE_TIME_FORMAT).to_string();

    // Check if file_meta entry exists and matches
    let meta_matches: i64 = db.lock().unwrap().query_row(
        "SELECT COUNT(*) FROM file_meta WHERE file_path = ?1 AND last_modified = ?2",
        params![path, last_modified_str],
        |row| row.get(0),
    )?;

    // Check if file has any metrics stored
    let metrics_exist: i64 = db.lock().unwrap().query_row(
        "SELECT COUNT(*) FROM metrics WHERE file_path = ?1",
        params![path],
        |row| row.get(0),
    )?;

    Ok(meta_matches == 0 || metrics_exist == 0)
}

pub fn write_metric_to_db(metrics: Metric, db: Arc<Mutex<Connection>>) -> Result<()> {
    let _ = db.lock().unwrap().execute(
        "INSERT OR REPLACE INTO metrics (file_path, name, value, date) VALUES (?1, ?2, ?3, ?4)",
        params![
            metrics.file_path,
            metrics.name,
            metrics.value,
            metrics.date.format(DB_DATE_FORMAT).to_string()
        ],
    );
    Ok(())
}

fn update_file_metadata(path: &str, db: Arc<Mutex<Connection>>) -> Result<()> {
    let file_path = Path::new(path);
    let last_modified: SystemTime = file_path
        .metadata()
        .and_then(|m| m.modified())
        .map_err(|e| anyhow::anyhow!("Failed to get metadata for {}: {}", path, e))?;

    let last_modified_time: DateTime<Utc> = last_modified.into();
    let last_modified_str = last_modified_time.format(DB_DATE_TIME_FORMAT).to_string();

    db.lock().unwrap().execute(
        "INSERT OR REPLACE INTO file_meta (file_path, last_modified) VALUES (?1, ?2)",
        params![path, last_modified_str],
    )?;

    Ok(())
}
