use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub value: u32,
    pub date: NaiveDate,
}

fn extract_metric(line: &str, raw_date_string: &str) -> Result<Metric> {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    let date = NaiveDate::parse_from_str(raw_date_string, "%Y-%m-%d").with_context(|| {
        format!(
            "Failed to parse the date from the file name {}",
            raw_date_string,
        )
    })?;
    if parts.len() == 2 {
        let name = parts[0].trim().to_string();
        let value = parts[1].trim().parse::<u32>().unwrap_or(0);
        Ok(Metric { name, value, date })
    } else {
        Err(anyhow::anyhow!("Invalid metric format in line {}", line))
    }
}

pub fn read_front_matter(
    path: &str,
    needed_metrics: &Vec<&str>,
    accumulator: &mut Vec<Metric>,
) -> Result<(), anyhow::Error> {
    let file = File::open(path).with_context(|| format!("Failed to open file: {}", path))?;
    let raw_date_string = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get file name from path"))?
        .to_string();
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
                let metric = extract_metric(&line, &raw_date_string)?;
                accumulator.push(metric);
            }
        }
    }
    Ok(())
}
