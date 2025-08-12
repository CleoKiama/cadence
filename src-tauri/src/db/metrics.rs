use chrono::{Datelike, NaiveDate};
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct RowMetric {
    pub name: String,
    pub value: i64,
    pub date: String,
}

pub fn get_metrics_by_date_range(
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

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use chrono::{Days, Local, Datelike};
    use std::sync::{Arc, Mutex};

    use crate::{
        core::read_journal::{Metric, DB_DATE_FORMAT},
        db::Db,
    };
    use rusqlite::{params, Connection};

    use super::*;

    const METRIC_NAME_1: &str = "pushups";
    const METRIC_NAME_2: &str = "reading";

    fn setup_test_db() -> Result<Db, anyhow::Error> {
        let conn = Connection::open_in_memory()
            .with_context(|| "Failed to open in-memory database with error".to_string())?;

        let db = Db {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_db()?;
        Ok(db)
    }

    fn seed_metrics_database(conn: Arc<Mutex<Connection>>) -> Result<(), anyhow::Error> {
        let conn = conn.lock().unwrap();
        let current_date = Local::now().date_naive();
        
        // Add metrics for the last 10 days
        for i in 0..10 {
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            
            // Add pushups metric
            let pushups_metric = Metric {
                file_path: format!("file_{}.md", i),
                name: METRIC_NAME_1.to_string(),
                value: 10 + i,
                date,
            };
            conn.execute(
                "INSERT INTO metrics(file_path,name,value,date) values (?1, ?2, ?3, ?4)",
                params![
                    pushups_metric.file_path,
                    pushups_metric.name,
                    pushups_metric.value,
                    pushups_metric.date.format(DB_DATE_FORMAT).to_string()
                ],
            )?;

            // Add reading metric every other day
            if i % 2 == 0 {
                let reading_metric = Metric {
                    file_path: format!("reading_file_{}.md", i),
                    name: METRIC_NAME_2.to_string(),
                    value: 30 + i,
                    date,
                };
                conn.execute(
                    "INSERT INTO metrics(file_path,name,value,date) values (?1, ?2, ?3, ?4)",
                    params![
                        reading_metric.file_path,
                        reading_metric.name,
                        reading_metric.value,
                        reading_metric.date.format(DB_DATE_FORMAT).to_string()
                    ],
                )?;
            }
        }
        Ok(())
    }

    #[test]
    fn test_get_metrics_in_range() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();
        seed_metrics_database(conn.clone()).expect("Failed to seed database");
        
        let current_date = Local::now().date_naive();
        let from_date = current_date.checked_sub_days(Days::new(5)).unwrap();
        let to_date = current_date.checked_sub_days(Days::new(2)).unwrap();
        
        let conn_guard = conn.lock().unwrap();
        let metrics = get_metrics_by_date_range(
            &conn_guard,
            (from_date.year(), from_date.month(), from_date.day()),
            (to_date.year(), to_date.month(), to_date.day()),
        ).expect("Failed to get metrics by date range");
        
        // Should get metrics for 4 days (5, 4, 3, 2 days ago)
        // Each day has pushups, every other day has reading
        // So we expect: 4 pushups + 2 reading = 6 metrics
        assert_eq!(metrics.len(), 6);
        
        // Verify we have both metric types
        let pushups_count = metrics.iter().filter(|m| m.name == METRIC_NAME_1).count();
        let reading_count = metrics.iter().filter(|m| m.name == METRIC_NAME_2).count();
        
        assert_eq!(pushups_count, 4);
        assert_eq!(reading_count, 2);
    }

    #[test]
    fn test_get_metrics_for_empty_range() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();
        seed_metrics_database(conn.clone()).expect("Failed to seed database");
        
        // Query a future date range where no metrics exist
        let future_date = Local::now().date_naive().checked_add_days(Days::new(10)).unwrap();
        let future_date_end = future_date.checked_add_days(Days::new(5)).unwrap();
        
        let conn_guard = conn.lock().unwrap();
        let metrics = get_metrics_by_date_range(
            &conn_guard,
            (future_date.year(), future_date.month(), future_date.day()),
            (future_date_end.year(), future_date_end.month(), future_date_end.day()),
        ).expect("Failed to get metrics by date range");
        
        assert_eq!(metrics.len(), 0);
    }

    #[test]
    fn test_get_metrics_with_invalid_date() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();
        
        let conn_guard = conn.lock().unwrap();
        
        // Test with invalid month (13)
        let result = get_metrics_by_date_range(
            &conn_guard,
            (2024, 13, 1), // Invalid month
            (2024, 12, 31),
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid date provided"));
        
        // Test with invalid day (32)
        let result = get_metrics_by_date_range(
            &conn_guard,
            (2024, 1, 1),
            (2024, 1, 32), // Invalid day
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid date provided"));
    }
}