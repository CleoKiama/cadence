use chrono::NaiveDate;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct HeatmapEntry {
    pub value: i64,
    pub date: String,
    pub habit_name: String,
}

pub fn get_monthly_heatmap_data(
    conn: &Arc<Mutex<Connection>>,
    habit_name: &str,
    year: i32,
    month: u32,
) -> Result<Vec<HeatmapEntry>, anyhow::Error> {
    // Validate input date
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| anyhow::anyhow!("Invalid year/month: {}/{}", year, month))?;

    // Get the last day of the month
    let last_day = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .and_then(|d| d.pred_opt())
    .ok_or_else(|| anyhow::anyhow!("Failed to calculate last day of month"))?;

    // Query existing data for this habit in the month
    let conn_guard = conn.lock().unwrap();
    let mut stmt = conn_guard.prepare(
        "SELECT value, date FROM metrics WHERE name = ?1 AND date >= ?2 AND date <= ?3 ORDER BY date"
    )?;

    let from_date = first_day.format("%Y-%m-%d").to_string();
    let to_date = last_day.format("%Y-%m-%d").to_string();

    let existing_data: std::collections::HashMap<String, i64> = stmt
        .query_map(params![habit_name, from_date, to_date], |row| {
            Ok((row.get::<_, String>(1)?, row.get::<_, i64>(0)?))
        })?
        .collect::<Result<std::collections::HashMap<_, _>, _>>()?;

    drop(stmt);
    drop(conn_guard);

    // Generate entries for all days in the month
    let mut result = Vec::new();
    let mut current_date = first_day;

    while current_date <= last_day {
        let date_str = current_date.format("%Y-%m-%d").to_string();
        let value = existing_data.get(&date_str).copied().unwrap_or(0);

        result.push(HeatmapEntry {
            value,
            date: date_str,
            habit_name: habit_name.to_string(),
        });

        current_date = current_date
            .succ_opt()
            .ok_or_else(|| anyhow::anyhow!("Date overflow while iterating through month"))?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::read_journal::DB_DATE_FORMAT, db::Db};
    use anyhow::Context;
    use rusqlite::{params, Connection};
    use std::sync::{Arc, Mutex};

    const TEST_HABIT: &str = "pushups";

    fn setup_test_db() -> Result<Db, anyhow::Error> {
        let conn =
            Connection::open_in_memory().with_context(|| "Failed to open in-memory database")?;

        let db = Db {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_db()?;
        Ok(db)
    }

    fn seed_partial_month_data(
        conn: &Connection,
        year: i32,
        month: u32,
    ) -> Result<(), anyhow::Error> {
        // Add data for specific days: 1st, 3rd, 5th, 15th, and last day of month
        let days_to_seed = vec![1, 3, 5, 15];

        for day in days_to_seed {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                conn.execute(
                    "INSERT INTO metrics(file_path,name,value,date) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        format!("file_{}.md", day),
                        TEST_HABIT,
                        day as i64 * 10, // Different values for each day
                        date.format(DB_DATE_FORMAT).to_string()
                    ],
                )?;
            }
        }

        // Add data for last day of month
        let last_day = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .and_then(|d| d.pred_opt())
        .unwrap();

        conn.execute(
            "INSERT INTO metrics(file_path,name,value,date) VALUES (?1, ?2, ?3, ?4)",
            params![
                "file_last.md",
                TEST_HABIT,
                999,
                last_day.format(DB_DATE_FORMAT).to_string()
            ],
        )?;

        Ok(())
    }

    #[test]
    fn test_get_monthly_heatmap_with_partial_data() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Test January 2024 (31 days)
        {
            let conn_guard = conn.lock().unwrap();
            seed_partial_month_data(&conn_guard, 2024, 1).expect("Failed to seed data");
        }

        let heatmap_data = get_monthly_heatmap_data(&conn, TEST_HABIT, 2024, 1)
            .expect("Failed to get heatmap data");

        // Should have 31 entries for January
        assert_eq!(heatmap_data.len(), 31);

        // Check specific days with data
        assert_eq!(heatmap_data[0].value, 10); // 1st day
        assert_eq!(heatmap_data[0].date, "2024-01-01");

        assert_eq!(heatmap_data[2].value, 30); // 3rd day
        assert_eq!(heatmap_data[2].date, "2024-01-03");

        assert_eq!(heatmap_data[4].value, 50); // 5th day
        assert_eq!(heatmap_data[14].value, 150); // 15th day
        assert_eq!(heatmap_data[30].value, 999); // Last day (31st)

        // Check days without data should be 0
        assert_eq!(heatmap_data[1].value, 0); // 2nd day
        assert_eq!(heatmap_data[1].date, "2024-01-02");

        assert_eq!(heatmap_data[6].value, 0); // 7th day
        assert_eq!(heatmap_data[10].value, 0); // 11th day
    }

    #[test]
    fn test_get_monthly_heatmap_february_leap_year() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Test February 2024 (leap year - 29 days)
        let heatmap_data = get_monthly_heatmap_data(&conn, TEST_HABIT, 2024, 2)
            .expect("Failed to get heatmap data");

        assert_eq!(heatmap_data.len(), 29);
        assert_eq!(heatmap_data[0].date, "2024-02-01");
        assert_eq!(heatmap_data[28].date, "2024-02-29");

        // All values should be 0 (no data)
        for entry in &heatmap_data {
            assert_eq!(entry.value, 0);
        }
    }

    #[test]
    fn test_get_monthly_heatmap_february_non_leap_year() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Test February 2023 (non-leap year - 28 days)
        let heatmap_data = get_monthly_heatmap_data(&conn, TEST_HABIT, 2023, 2)
            .expect("Failed to get heatmap data");

        assert_eq!(heatmap_data.len(), 28);
        assert_eq!(heatmap_data[0].date, "2023-02-01");
        assert_eq!(heatmap_data[27].date, "2023-02-28");
    }

    #[test]
    fn test_get_monthly_heatmap_april_30_days() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Test April 2024 (30 days)
        let heatmap_data = get_monthly_heatmap_data(&conn, TEST_HABIT, 2024, 4)
            .expect("Failed to get heatmap data");

        assert_eq!(heatmap_data.len(), 30);
        assert_eq!(heatmap_data[0].date, "2024-04-01");
        assert_eq!(heatmap_data[29].date, "2024-04-30");
    }

    #[test]
    fn test_get_monthly_heatmap_no_data() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Test month with no data
        let heatmap_data = get_monthly_heatmap_data(&conn, "nonexistent_habit", 2024, 6)
            .expect("Failed to get heatmap data");

        assert_eq!(heatmap_data.len(), 30); // June has 30 days

        // All values should be 0
        for entry in &heatmap_data {
            assert_eq!(entry.value, 0);
        }
    }

    #[test]
    fn test_get_monthly_heatmap_invalid_date() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Test invalid month
        let result = get_monthly_heatmap_data(&conn, TEST_HABIT, 2024, 13);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid year/month"));

        // Test invalid month 0
        let result = get_monthly_heatmap_data(&conn, TEST_HABIT, 2024, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_monthly_heatmap_full_month_data() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        // Seed data for every day of March 2024
        {
            let conn_guard = conn.lock().unwrap();
            for day in 1..=31 {
                let date = NaiveDate::from_ymd_opt(2024, 3, day).unwrap();
                conn_guard
                    .execute(
                        "INSERT INTO metrics(file_path,name,value,date) VALUES (?1, ?2, ?3, ?4)",
                        params![
                            format!("file_{}.md", day),
                            TEST_HABIT,
                            day as i64,
                            date.format(DB_DATE_FORMAT).to_string()
                        ],
                    )
                    .expect("Failed to insert test data");
            }
        }

        let heatmap_data = get_monthly_heatmap_data(&conn, TEST_HABIT, 2024, 3)
            .expect("Failed to get heatmap data");

        assert_eq!(heatmap_data.len(), 31);

        // Check that all days have correct values
        for (index, entry) in heatmap_data.iter().enumerate() {
            let expected_day = index + 1;
            assert_eq!(entry.value, expected_day as i64);
            assert_eq!(entry.date, format!("2024-03-{:02}", expected_day));
        }
    }

    #[test]
    fn test_get_monthly_heatmap_different_habits() {
        let db = setup_test_db().expect("Failed to setup test database");
        let conn = db.get_connection();

        let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();

        // Add data for different habits on same day
        {
            let conn_guard = conn.lock().unwrap();
            conn_guard
                .execute(
                    "INSERT INTO metrics(file_path,name,value,date) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        "file1.md",
                        "pushups",
                        100,
                        date.format(DB_DATE_FORMAT).to_string()
                    ],
                )
                .expect("Failed to insert pushups data");

            conn_guard
                .execute(
                    "INSERT INTO metrics(file_path,name,value,date) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        "file2.md",
                        "reading",
                        200,
                        date.format(DB_DATE_FORMAT).to_string()
                    ],
                )
                .expect("Failed to insert reading data");
        }

        // Test pushups habit
        let pushups_data = get_monthly_heatmap_data(&conn, "pushups", 2024, 5)
            .expect("Failed to get pushups heatmap");
        assert_eq!(pushups_data[14].value, 100); // 15th day (0-indexed)
        assert_eq!(pushups_data[0].value, 0); // Other days should be 0

        // Test reading habit
        let reading_data = get_monthly_heatmap_data(&conn, "reading", 2024, 5)
            .expect("Failed to get reading heatmap");
        assert_eq!(reading_data[14].value, 200); // 15th day (0-indexed)
        assert_eq!(reading_data[0].value, 0); // Other days should be 0
    }
}
