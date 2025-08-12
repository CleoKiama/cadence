use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use chrono::{Local, NaiveDate};
use rusqlite::{params, Connection};

use crate::core::read_journal::DB_DATE_FORMAT;

pub struct RowMetric {
    name: String,
    value: i64,
    date: String,
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

pub fn get_habit_streak(conn: &Arc<Mutex<Connection>>, name: &str) -> Result<i64, anyhow::Error> {
    let conn = conn.lock().unwrap();
    let today = Local::now().date_naive();
    let mut current_date = today
        .checked_sub_days(chrono::Days::new(1))
        .ok_or_else(|| anyhow::anyhow!("Failed to compute yesterday"))?; // Start from yesterday

    // Query all relevant dates where this habit has value > 0
    let mut stmt = conn.prepare("SELECT date FROM metrics WHERE name = ?1 AND value > 0")?;
    let date_iter = stmt.query_map(params![name], |row| row.get::<_, String>(0))?;

    let mut logged_dates = HashSet::new();
    for date_result in date_iter {
        match date_result {
            Ok(date_str) => {
                if let Ok(date) = NaiveDate::parse_from_str(&date_str, DB_DATE_FORMAT) {
                    logged_dates.insert(date);
                } else {
                    return Err(anyhow::anyhow!(
                        "Invalid date format in database: {}",
                        date_str
                    ));
                }
            }
            Err(e) => return Err(anyhow::anyhow!("Failed to read date from database: {}", e)),
        }
    }

    let mut streak = 0;

    // Walk backward from yesterday while we see valid entries
    while logged_dates.contains(&current_date) {
        streak += 1;

        match current_date.checked_sub_days(chrono::Days::new(1)) {
            Some(date) => current_date = date,
            None => break, // Reached Unix epoch or invalid date
        }
    }

    Ok(streak)
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use chrono::{Days, Local};
    use rand::{rng, Rng};

    use crate::{
        core::read_journal::{Metric, DB_DATE_FORMAT},
        db::Db,
    };

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const MAX_DATE: i32 = 20;
    const METRIC_NAME: &str = "dsa_solved";

    fn setup_test_db() -> Result<Db, anyhow::Error> {
        let conn = Connection::open_in_memory()
            .with_context(|| "Failed to open in-memory database with error".to_string())?;

        let db = Db {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_db()?;
        Ok(db)
    }

    fn seed_database(conn: Arc<Mutex<Connection>>) -> Result<(), anyhow::Error> {
        let conn = conn.lock().unwrap();
        let current_date = Local::now().date_naive();
        for i in 0..=MAX_DATE {
            let value: u32 = rng().random_range(1..=10);
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("file_{}.md", i),
                name: METRIC_NAME.to_string(),
                value,
                date,
            };
            conn.execute(
                "INSERT INTO metrics(file_path,name,value,date) values (?1, ?2, ?3, ?4)",
                params![
                    metric.file_path,
                    metric.name,
                    metric.value,
                    metric.date.format("%Y-%m-%d").to_string()
                ],
            )?;
        }
        Ok(())
    }

    #[test]
    fn test_get_habit_streak() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();
        seed_database(conn.clone()).expect("Failed to seed database");
        let streak = get_habit_streak(&conn, METRIC_NAME).expect("Failed to get habit streak");
        assert_eq!(streak, 20);
    }
    #[test]
    fn test_broken_habit_streak() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();
        seed_database(conn.clone()).expect("Failed to seed database");

        let conn = conn.lock().unwrap();
        let two_days_ago = Local::now()
            .date_naive()
            .checked_sub_days(Days::new(2))
            .expect("Invalid date");

        // Break 2 days ago — set to zero
        conn.execute(
            "UPDATE metrics SET value = 0 WHERE name = ?1 AND date = ?2",
            params![METRIC_NAME, two_days_ago.format(DB_DATE_FORMAT).to_string()],
        )
        .expect("Failed to update");

        drop(conn); // unlock

        let streak =
            get_habit_streak(&db.get_connection(), METRIC_NAME).expect("Failed to get streak");

        assert_eq!(streak, 1);
    }
    #[test]
    fn test_missed_yesterday_resets_streak() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();
        seed_database(conn.clone()).expect("Failed to seed database");

        let conn = conn.lock().unwrap();
        let yesterday = Local::now()
            .date_naive()
            .checked_sub_days(Days::new(1))
            .unwrap();

        conn.execute(
            "UPDATE metrics SET value = 0 WHERE name = ?1 AND date = ?2",
            params![METRIC_NAME, yesterday.format(DB_DATE_FORMAT).to_string()],
        )
        .expect("Failed to zero yesterday");

        drop(conn);

        let streak = get_habit_streak(&db.get_connection(), METRIC_NAME).unwrap();
        assert_eq!(streak, 0);
    }
}
