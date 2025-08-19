use crate::{
    db::streaks::{get_habit_streak, get_longest_habit_streak},
    DbConnection,
};
use tauri::State;

#[tauri::command]
pub fn get_longest_streak(db: State<'_, DbConnection>, habit_name: String) -> Result<i64, String> {
    let result = get_longest_habit_streak(&db, &habit_name);
    match result {
        Ok(streak) => Ok(streak),
        Err(err) => Err(format!("Failed to get longest streak: {}", err)),
    }
}

#[tauri::command]
pub fn get_current_streak(db: State<'_, DbConnection>, habit_name: String) -> Result<i64, String> {
    let result = get_habit_streak(&db, &habit_name);
    match result {
        Ok(streak) => Ok(streak),
        Err(err) => Err(format!("Failed to get longest streak: {}", err)),
    }
}
