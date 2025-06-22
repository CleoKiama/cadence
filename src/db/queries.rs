use chrono::NaiveDate;
use rusqlite::{Connection, params};

pub struct RowMetric {
    pub name: String,
    pub value: i64,
    pub date: String,
}

pub fn get_metrics_by_date(
    conn: &Connection,
    from: (i32, u32, u32), // (year, month, day)
    to: (i32, u32, u32),   // (year, month, day)
) -> Result<Vec<RowMetric>, anyhow::Error> {
    let from_query = NaiveDate::from_ymd_opt(from.0, from.1, from.2);
    let to_query = NaiveDate::from_ymd_opt(to.0, to.1, to.2);
    let from_query = match from_query {
        Some(date) => date.format("%Y-%m-%d").to_string(),
        None => return Err(anyhow::anyhow!("Invalid date provided")),
    };
    let to_query = match to_query {
        Some(date) => date.format("%Y-%m-%d").to_string(),
        None => return Err(anyhow::anyhow!("Invalid date provided")),
    };
    let mut stmt =
        conn.prepare("SELECT name, value, date FROM metrics WHERE date >= ?1 AND date <= ?2")?;
    let metrics_iter = stmt.query_map(params![from_query, to_query], |row| {
        Ok(RowMetric {
            name: row.get(0)?,
            value: row.get(1)?,
            date: row.get(2)?,
        })
    })?;

    let mut metrics: Vec<RowMetric> = Vec::new();
    for metric in metrics_iter {
        metrics.push(metric?);
    }
    Ok(metrics)
}
