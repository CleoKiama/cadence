use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use chrono::{Local, NaiveDate};
use rusqlite::{params, Connection};

use crate::core::read_journal::DB_DATE_FORMAT;

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

pub fn get_longest_habit_streak(
    conn: &Arc<Mutex<Connection>>,
    name: &str,
) -> Result<i64, anyhow::Error> {
    let conn = conn.lock().unwrap();

    // Query all relevant dates where this habit has value > 0
    let mut stmt =
        conn.prepare("SELECT date FROM metrics WHERE name = ?1 AND value > 0 ORDER BY date ASC")?;
    let date_iter = stmt.query_map(params![name], |row| row.get::<_, String>(0))?;

    let mut dates = Vec::new();
    for date_result in date_iter {
        match date_result {
            Ok(date_str) => {
                if let Ok(date) = NaiveDate::parse_from_str(&date_str, DB_DATE_FORMAT) {
                    dates.push(date);
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

    if dates.is_empty() {
        return Ok(0); // No entries found, return 0
    }

    let mut longest_streak = 1;
    let mut current_streak = 1;

    let dates_iter = dates.windows(2);

    for window in dates_iter {
        let [prev_date, current_date] = window else {
            continue;
        };
        if let Some(expected_date) = prev_date.checked_add_days(chrono::Days::new(1)) {
            if *current_date == expected_date {
                current_streak += 1;
                longest_streak = longest_streak.max(current_streak);
            } else {
                current_streak = 1;
            }
        }
    }

    Ok(longest_streak)
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use chrono::{Days, Local};
    use rand::{rng, Rng};

    use crate::{
        core::read_journal::{Metric, DB_DATE_FORMAT},
        db::{seed::insert_metric, Db},
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

    fn seed_test_data(
        conn: Arc<Mutex<Connection>>,
        metric_name: &str,
        days: i32,
    ) -> Result<(), anyhow::Error> {
        let current_date = Local::now().date_naive();
        for i in 0..=days {
            let value: u32 = rng().random_range(1..=10);
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("test_file_{}.md", date.format("%Y-%m-%d")),
                name: metric_name.to_string(),
                value,
                date,
            };
            insert_metric(&conn, &metric)?;
        }
        Ok(())
    }

    fn seed_database(conn: Arc<Mutex<Connection>>) -> Result<(), anyhow::Error> {
        seed_test_data(conn, METRIC_NAME, MAX_DATE)
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

    #[test]
    fn test_get_longest_habit_streak_continuous() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();
        seed_database(conn.clone()).expect("Failed to seed database");

        let longest_streak = get_longest_habit_streak(&conn, METRIC_NAME)
            .expect("Failed to get longest habit streak");
        assert_eq!(longest_streak, 21); // MAX_DATE + 1 (0 to 20 inclusive)
    }

    #[test]
    fn test_get_longest_habit_streak_with_gaps() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Create a pattern: 3 days, gap, 5 days, gap, 2 days
        let conn_guard = conn.lock().unwrap();
        let current_date = Local::now().date_naive();

        // First streak: 3 days (days 0, 1, 2)
        for i in 0..3 {
            let date = current_date.checked_sub_days(Days::new(i)).unwrap();
            conn_guard
                .execute(
                    "INSERT INTO metrics(file_path,name,value,date) values (?1, ?2, ?3, ?4)",
                    params![
                        format!("file_{}.md", i),
                        METRIC_NAME,
                        1,
                        date.format(DB_DATE_FORMAT).to_string()
                    ],
                )
                .unwrap();
        }

        // Second streak: 5 days (days 5, 6, 7, 8, 9)
        for i in 5..10 {
            let date = current_date.checked_sub_days(Days::new(i)).unwrap();
            conn_guard
                .execute(
                    "INSERT INTO metrics(file_path,name,value,date) values (?1, ?2, ?3, ?4)",
                    params![
                        format!("file_{}.md", i),
                        METRIC_NAME,
                        1,
                        date.format(DB_DATE_FORMAT).to_string()
                    ],
                )
                .unwrap();
        }

        // Third streak: 2 days (days 12, 13)
        for i in 12..14 {
            let date = current_date.checked_sub_days(Days::new(i)).unwrap();
            conn_guard
                .execute(
                    "INSERT INTO metrics(file_path,name,value,date) values (?1, ?2, ?3, ?4)",
                    params![
                        format!("file_{}.md", i),
                        METRIC_NAME,
                        1,
                        date.format(DB_DATE_FORMAT).to_string()
                    ],
                )
                .unwrap();
        }

        drop(conn_guard);

        let longest_streak = get_longest_habit_streak(&conn, METRIC_NAME)
            .expect("Failed to get longest habit streak");
        assert_eq!(longest_streak, 5); // Should find the longest streak of 5 days
    }

    #[test]
    fn test_get_longest_habit_streak_no_entries() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        let longest_streak = get_longest_habit_streak(&conn, "nonexistent_habit")
            .expect("Failed to get longest habit streak");
        assert_eq!(longest_streak, 0);
    }

    #[test]
    fn test_get_longest_habit_streak_single_entry() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        let conn_guard = conn.lock().unwrap();
        let current_date = Local::now().date_naive();

        conn_guard
            .execute(
                "INSERT INTO metrics(file_path,name,value,date) values (?1, ?2, ?3, ?4)",
                params![
                    "single_file.md",
                    METRIC_NAME,
                    1,
                    current_date.format(DB_DATE_FORMAT).to_string()
                ],
            )
            .unwrap();

        drop(conn_guard);

        let longest_streak = get_longest_habit_streak(&conn, METRIC_NAME)
            .expect("Failed to get longest habit streak");
        assert_eq!(longest_streak, 1);
    }
}
