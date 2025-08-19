use anyhow::Context;
use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use rusqlite::params;

use crate::{
    commands::analytics::{ChartDataPoint, HeatmapPoint},
    core::read_journal::DB_DATE_FORMAT,
    db::utils::get_all_habits,
    DbConnection,
};

pub fn get_weekly_metric_avg(
    db: &DbConnection,
    habit_name: &str,
) -> Result<f32, anyhow::Error> {
    let conn = db.lock().unwrap();
    let now = Local::now();
    let today = now.date_naive();
    let week = today.week(Weekday::Sun);
    let (start, end) = week.days().into_inner();
    let start_date = start.format(DB_DATE_FORMAT).to_string();
    let end_date = end.format(DB_DATE_FORMAT).to_string();
    let mut query_stmt = conn
        .prepare(
            " 
        SELECT AVG(value)
        FROM metrics
        WHERE name = ?1
        AND date >= ?2
        AND date <= ?3
        ",
        )
        .with_context(|| "Failed to prepare SQL statement")?;
    query_stmt
        .query_one(params![habit_name, start_date, end_date], |row| {
            let avg: Option<f32> = row.get(0)?;
            Ok(avg.unwrap_or(0.0))
        })
        .map_err(|e| anyhow::anyhow!(e))
}

pub fn get_monthly_metric_total(
    db: &DbConnection,
    habit_name: &str,
) -> Result<u32, anyhow::Error> {
    let conn = db.lock().unwrap();
    let today = Local::now().date_naive();

    let start_of_month = today
        .with_day(1)
        .ok_or_else(|| anyhow::anyhow!("Failed to calculate start of month"))?;

    let first_of_next_month = if today.month() == 12 {
        NaiveDate::from_ymd_opt(today.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1)
    }
    .ok_or_else(|| anyhow::anyhow!("Failed to calculate first day of next month"))?;

    let end_of_month = first_of_next_month
        .checked_sub_days(Days::new(1))
        .ok_or_else(|| anyhow::anyhow!("Failed to calculate end of month"))?;

    let mut query_stmt = conn
        .prepare(
            "
        SELECT SUM(value)
        FROM metrics
        WHERE name = ?1
        AND date >= ?2
        AND date <= ?3
        ",
        )
        .with_context(|| "Failed to prepare SQL statement")?;

    query_stmt
        .query_one(
            params![
                habit_name,
                start_of_month.format(DB_DATE_FORMAT).to_string(),
                end_of_month.format(DB_DATE_FORMAT).to_string()
            ],
            |row| {
                let total: Option<u32> = row.get(0)?;
                Ok(total.unwrap_or(0))
            },
        )
        .map_err(|e| anyhow::anyhow!(e))
}

fn get_habit_trend_data(
    db: &DbConnection,
    habit_name: &str,
    days: u32,
) -> Result<Vec<ChartDataPoint>, anyhow::Error> {
    let conn = db.lock().unwrap();
    let today = Local::now().date_naive();
    let start_date = today
        .checked_sub_days(Days::new(days as u64))
        .ok_or_else(|| anyhow::anyhow!("Failed to calculate start date"))?;

    let mut stmt = conn.prepare(
        "
        SELECT date, value 
        FROM metrics 
        WHERE name = ?1 
        AND date >= ?2 
        AND date <= ?3 
        ORDER BY date ASC
        ",
    )?;

    let start_date_str = start_date.format(DB_DATE_FORMAT).to_string();
    let end_date_str = today.format(DB_DATE_FORMAT).to_string();

    let mut existing_data: std::collections::HashMap<String, i64> =
        std::collections::HashMap::new();

    let rows = stmt.query_map(params![habit_name, start_date_str, end_date_str], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;

    for row in rows {
        let (date, value) = row?;
        existing_data.insert(date, value);
    }

    // Generate data points for all days in the range
    let mut result = Vec::new();
    let mut current_date = start_date;

    while current_date <= today {
        let date_str = current_date.format(DB_DATE_FORMAT).to_string();
        let value = existing_data.get(&date_str).copied().unwrap_or(0);

        result.push(ChartDataPoint {
            date: date_str,
            value,
        });

        current_date = current_date
            .succ_opt()
            .ok_or_else(|| anyhow::anyhow!("Date overflow while iterating"))?;
    }

    Ok(result)
}

pub fn get_habit_heatmap_data(
    db: &DbConnection,
    habit_name: &str,
    days: u32,
) -> Result<Vec<HeatmapPoint>, anyhow::Error> {
    let trend_data = get_habit_trend_data(db, habit_name, days)?;

    // Find max value to calculate intensity levels
    let max_value = trend_data.iter().map(|d| d.value).max().unwrap_or(1);

    let result = trend_data
        .into_iter()
        .map(|point| {
            let level = if point.value == 0 {
                0
            } else {
                // Calculate level from 1-4 based on value relative to max
                let ratio = point.value as f64 / max_value as f64;
                ((ratio * 3.0).ceil() as u8 + 1).min(4)
            };

            HeatmapPoint {
                date: point.date,
                count: point.value,
                level,
            }
        })
        .collect();

    Ok(result)
}

pub fn get_all_habits_analytics(
    db: &DbConnection,
    days: u32,
) -> Result<std::collections::HashMap<String, Vec<ChartDataPoint>>, anyhow::Error> {
    // Get all unique habit names
    let habit_names = get_all_habits(db)?;
    let mut result = std::collections::HashMap::new();

    for habit_name in habit_names {
        let trend_data = get_habit_trend_data(db, &habit_name, days)?;
        result.insert(habit_name, trend_data);
    }

    Ok(result)
}
