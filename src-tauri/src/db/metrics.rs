use anyhow::Context;
use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use rusqlite::params;

use crate::{core::read_journal::DB_DATE_FORMAT, DbConnection};

pub fn get_weekly_metric_avg(db: &DbConnection, habit_name: &str) -> Result<i32, anyhow::Error> {
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
        SELECT round(AVG(value))
        FROM metrics
        WHERE name = ?1
        AND date between ?2 AND  ?3
        ",
        )
        .with_context(|| "Failed to prepare SQL statement")?;
    query_stmt
        .query_one(params![habit_name, start_date, end_date], |row| {
            let avg: Option<f32> = row.get(0)?;
            Ok(avg.unwrap_or(0.0) as i32)
        })
        .map_err(|e| anyhow::anyhow!(e))
}

pub fn get_monthly_metric_total(db: &DbConnection, habit_name: &str) -> Result<u32, anyhow::Error> {
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
