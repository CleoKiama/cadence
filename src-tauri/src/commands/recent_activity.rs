use chrono::{Local, Months};
use tauri::State;

use crate::commands::utils::activity_server::{get_acitivity_data, HabitData};
use crate::DbConnection;

#[tauri::command]
pub fn get_recent_activity(db: State<'_, DbConnection>) -> Result<Option<Vec<HabitData>>, String> {
    let today = Local::now().date_naive();
    let end_date = today
        .checked_sub_months(Months::new(1))
        .ok_or_else(|| "Failed to calculate target date".to_string())?;

    let data = get_acitivity_data(&db, end_date, today).map_err(|e| e.to_string())?; // get data for the past 30 days

    if data.is_empty() {
        Ok(None)
    } else {
        Ok(Some(data))
    }
}
