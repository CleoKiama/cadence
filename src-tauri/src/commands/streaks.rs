use crate::{
    db::streaks::{get_habit_streak, get_longest_habit_streak},
    db::utils::get_all_habits,
    DbConnection,
};
use tauri::State;

#[tauri::command]
pub fn get_longest_streak(db: State<'_, DbConnection>, habit_name: String) -> Result<Option<i64>, String> {
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
pub fn get_current_streak(db: State<'_, DbConnection>, habit_name: String) -> Result<Option<i64>, String> {
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
