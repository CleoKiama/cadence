use crate::db::streaks::{get_habit_streak, get_longest_habit_streak};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tauri::State;

#[tauri::command]
pub fn get_longest_streak(
    state: State<'_, Arc<Mutex<Connection>>>,
    habit_name: String,
) -> Result<i64, String> {
    let result = get_longest_habit_streak(&state, &habit_name);
    match result {
        Ok(streak) => Ok(streak),
        Err(err) => Err(format!("Failed to get longest streak: {}", err)),
    }
}

#[tauri::command]
pub fn get_current_streak(
    state: State<'_, Arc<Mutex<Connection>>>,
    habit_name: String,
) -> Result<i64, String> {
    let result = get_habit_streak(&state, &habit_name);
    match result {
        Ok(streak) => Ok(streak),
        Err(err) => Err(format!("Failed to get longest streak: {}", err)),
    }
}
