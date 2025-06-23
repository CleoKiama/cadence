use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::Connection;
use std::fs::File;
use std::fs::metadata;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub value: u32,
    pub date: NaiveDate,
    pub file_path: String,
}

pub fn read_front_matter(
    path: &str,
    needed_metrics: &Vec<String>,
    conn: &Connection,
) -> Result<(), anyhow::Error> {
    if !should_read_file(path, conn)? {
        return Ok(());
    }

    let file = File::open(path).with_context(|| format!("Failed to open file: {}", path))?;
    let reader = BufReader::new(file);
    let mut in_front_matter = false;
    for (i, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Failed to read line {}", i + 1))?;
        if line.trim().contains("---") {
            if in_front_matter {
                // End of front matter
                break;
            } else {
                // Start of front matter
                in_front_matter = true;
                continue;
            }
        }

        for metric in needed_metrics {
            if line.starts_with(metric) {
                let metric = extract_metric(&line, path)?;
                write_metric_to_db(metric, conn)?;
            }
        }
    }

    update_file_metadata(path, conn)?;
    Ok(())
}

fn extract_metric(line: &str, path: &str) -> Result<Metric> {
    let raw_date_string = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get file name from path"))?
        .to_string();
    let date = NaiveDate::parse_from_str(&raw_date_string, "%Y-%m-%d").with_context(|| {
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

fn should_read_file(path: &str, conn: &Connection) -> Result<bool> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Ok(false);
    }

    let last_modified: SystemTime = metadata(file_path)
        .and_then(|m| m.modified())
        .map_err(|e| anyhow::anyhow!("Failed to get metadata for {}: {}", path, e))?;

    let last_modified_time: DateTime<Utc> = last_modified.into();
    let last_modified_str = last_modified_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Check if file_meta entry exists and matches
    let meta_matches: i64 = conn.query_row(
        "SELECT COUNT(*) FROM file_meta WHERE file_path = ?1 AND last_modified = ?2",
        rusqlite::params![path, last_modified_str],
        |row| row.get(0),
    )?;

    // Check if file has any metrics stored
    let metrics_exist: i64 = conn.query_row(
        "SELECT COUNT(*) FROM metrics WHERE file_path = ?1",
        rusqlite::params![path],
        |row| row.get(0),
    )?;

    // We want to re-read the file if:
    // - meta does not match
    // - OR no metrics exist for this file
    Ok(meta_matches == 0 || metrics_exist == 0)
}

pub fn write_metric_to_db(metrics: Metric, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO metrics (file_path, name, value, date) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            metrics.file_path,
            metrics.name,
            metrics.value,
            metrics.date.format("%Y-%m-%d").to_string()
        ],
    )
    .with_context(|| format!("Failed to insert metric: {:?}", metrics))?;
    Ok(())
}

fn update_file_metadata(path: &str, conn: &Connection) -> Result<()> {
    let file_path = Path::new(path);
    let last_modified: SystemTime = file_path
        .metadata()
        .and_then(|m| m.modified())
        .map_err(|e| anyhow::anyhow!("Failed to get metadata for {}: {}", path, e))?;

    let last_modified_time: DateTime<Utc> = last_modified.into();
    let last_modified_str = last_modified_time.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT OR REPLACE INTO file_meta (file_path, last_modified) VALUES (?1, ?2)",
        rusqlite::params![path, last_modified_str],
    )?;

    Ok(())
}
