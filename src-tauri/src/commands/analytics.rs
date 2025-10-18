use chrono::NaiveDate;
use tauri::State;

use crate::{
    core::read_journal::DB_DATE_FORMAT, db::streaks::compute_longest_streak, DbConnection,
};

#[tauri::command]
pub fn get_all_habits_longest_streak(db: State<'_, DbConnection>) -> Result<i64, String> {
    let conn = db.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT DISTINCT date FROM metrics WHERE value > 0 ORDER BY date ASC")
        .map_err(|e| format!("Database error preparing streak query: {}", e))?;

    let date_iter = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| format!("Database error executing streak query: {}", e))?;

    // Call the calculation function and map the final anyhow::Error to String
    let result = compute_longest_streak(date_iter).map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub fn count_all_habits(db: State<'_, DbConnection>) -> Result<Option<u32>, String> {
    let conn = db.lock().unwrap();
    let total = conn
        .query_one(
            "select count(distinct name) from metrics where value > 0",
            [],
            |row| row.get::<_, Option<u32>>(0),
        )
        .map_err(|e| format!("error getting total habits count: {}", e))?;
    Ok(total)
}

#[tauri::command]
pub fn get_completion_rate(db: State<'_, DbConnection>) -> Result<f64, String> {
    let conn = db.lock().unwrap();

    // 1. Get Date Range (MIN and MAX logged dates)
    let (min_date_str, max_date_str) = conn
        .query_row("SELECT MIN(date), MAX(date) FROM metrics", [], |row| {
            Ok((row.get::<_, String>(0), row.get::<_, String>(1)))
        })
        .map_err(|e| format!("Database error fetching date range: {}", e))?;

    if min_date_str.is_err() || max_date_str.is_err() {
        return Ok(0.0);
    }

    let min_date_str = min_date_str.unwrap();
    let max_date_str = max_date_str.unwrap();

    // Parse dates and calculate total days logged (Denominator component)
    let start_date = NaiveDate::parse_from_str(&min_date_str, DB_DATE_FORMAT)
        .map_err(|e| format!("Date parse error (min date {}): {}", min_date_str, e))?;
    let end_date = NaiveDate::parse_from_str(&max_date_str, DB_DATE_FORMAT)
        .map_err(|e| format!("Date parse error (max date {}): {}", max_date_str, e))?;

    let total_days_logged = (end_date - start_date).num_days() + 1;

    let total_successes = conn
        .query_row("SELECT COUNT(*) FROM metrics WHERE value > 0", [], |row| {
            row.get::<_, i64>(0)
        })
        .map_err(|e| format!("Database error fetching total successes: {}", e))?;

    let total_unique_habits = conn
        .query_row("SELECT COUNT(DISTINCT name) FROM metrics", [], |row| {
            row.get::<_, i64>(0)
        })
        .map_err(|e| format!("Database error fetching total unique habits: {}", e))?;

    if total_days_logged <= 0 || total_unique_habits <= 0 {
        return Ok(0.0);
    }

    let total_opportunities = total_days_logged as f64 * total_unique_habits as f64;

    let completion_rate = (total_successes as f64 / total_opportunities) * 100.0;

    Ok(completion_rate.min(100.0))
}

#[tauri::command]
pub fn count_active_days(db: State<'_, DbConnection>) -> Result<i64, String> {
    let conn = db.lock().unwrap();

    // Use COUNT(DISTINCT date) to find the number of unique days with activity (value > 0).
    let total_active_days = conn
        .query_one(
            "SELECT COUNT(DISTINCT date) FROM metrics WHERE value > 0",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|e| format!("Error getting total active days count: {}", e))?;

    Ok(total_active_days)
}
