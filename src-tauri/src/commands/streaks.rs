use anyhow::{anyhow, Result};

use crate::{
    commands::utils::activity_server::{get_acitivity_data, HabitData},
    db::{
        streaks::{get_habit_streak, get_longest_habit_streak},
        utils::get_all_habits,
    },
    DbConnection,
};

use chrono::{Days, Months, NaiveDate};
use tauri::State;

#[tauri::command]
pub fn get_longest_streak(
    db: State<'_, DbConnection>,
    habit_name: String,
) -> Result<Option<i64>, String> {
    // Check if any habits are tracked first
    let habits = get_all_habits(&db).map_err(|e| e.to_string())?;
    if habits.is_empty() {
        return Ok(None);
    }

    let result = get_longest_habit_streak(&db, &habit_name);
    match result {
        Ok(streak) => Ok(Some(streak)),
        Err(err) => Err(format!("Failed to get longest streak: {}", err)),
    }
}

#[tauri::command]
pub fn get_current_streak(
    db: State<'_, DbConnection>,
    habit_name: String,
) -> Result<Option<i64>, String> {
    // Check if any habits are tracked first
    let habits = get_all_habits(&db).map_err(|e| e.to_string())?;
    if habits.is_empty() {
        return Ok(None);
    }

    let result = get_habit_streak(&db, &habit_name);
    match result {
        Ok(streak) => Ok(Some(streak)),
        Err(err) => Err(format!("Failed to get current streak: {}", err)),
    }
}

#[tauri::command]
pub fn get_current_streak_data(
    db: State<'_, DbConnection>,
    year: i32,
    month: u32,
) -> Result<Vec<HabitData>, String> {
    let current_date = NaiveDate::from_ymd_opt(year, month, 1).ok_or("failed to parse the date")?; //default to the first date of the month
    let data = get_month_range_data(current_date, &db)
        .map_err(|err| format!("Error getting streak data {:?}", err))?;
    Ok(data)
}

fn get_month_range_data(
    date: NaiveDate,
    db: &DbConnection,
) -> Result<Vec<HabitData>, anyhow::Error> {
    let last_day_prev_month = date
        .checked_sub_days(Days::new(1))
        .ok_or_else(|| anyhow!("Error subtracting a day"))?;

    let last_week_prev_month_start = last_day_prev_month - Days::new(6);
    println!(
        "last_week_prev_month_start: {:#?}",
        last_week_prev_month_start.format("%Y-%m-%d").to_string()
    );

    let first_day_next_month = date
        .checked_add_months(Months::new(1))
        .ok_or_else(|| anyhow!("Error in get_month_range checked_add_months"))?;

    let first_week_end_next_month = first_day_next_month + chrono::Duration::days(6);
    println!(
        "first_day_next_month: {:#?}",
        first_week_end_next_month.format("%Y-%m-%d").to_string()
    );

    get_acitivity_data(db, last_week_prev_month_start, first_week_end_next_month)
}
