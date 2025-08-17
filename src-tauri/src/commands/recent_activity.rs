use anyhow::anyhow;
use chrono::{Days, Local, Months};
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::State;

use crate::{core::read_journal::DB_DATE_FORMAT, db::utils::get_all_habits};

#[derive(Serialize)]
pub struct DataPoint {
    value: i32,
    date: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitData {
    habit_name: String,
    data: Vec<DataPoint>,
}

#[tauri::command]
pub fn get_recent_activity(
    state: State<'_, Arc<Mutex<Connection>>>,
) -> Result<Vec<HabitData>, String> {
    let data = get_recent_activity_date(&state).map_err(|e| e.to_string())?;
    Ok(data)
}

fn get_recent_activity_date(
    conn: &Arc<Mutex<Connection>>,
) -> Result<Vec<HabitData>, anyhow::Error> {
    let current_habits = get_all_habits(conn)?;
    let mut habit_data: Vec<HabitData> = Vec::new();

    let today = Local::now().date_naive();
    let target_date = today
        .checked_sub_months(Months::new(1))
        .ok_or_else(|| anyhow!("Failed to calculate target date"))?;

    let conn = conn
        .lock()
        .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT name, date, value 
         FROM metrics 
         WHERE date BETWEEN ?1 AND ?2",
    )?;

    let mut rows = stmt.query(rusqlite::params![
        target_date.format(DB_DATE_FORMAT).to_string(),
        today.format(DB_DATE_FORMAT).to_string(),
    ])?;

    let mut values: HashMap<(String, String), i32> = HashMap::new();

    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let date: String = row.get(1)?;
        let value: i32 = row.get(2)?;
        values.insert((name, date), value);
    }

    for habit in current_habits {
        let mut data_points: Vec<DataPoint> = Vec::new();
        let mut day = target_date;

        while day <= today {
            let date_str = day.format(DB_DATE_FORMAT).to_string();
            let value = values
                .get(&(habit.clone(), date_str.clone()))
                .copied()
                .unwrap_or(0);

            data_points.push(DataPoint {
                value,
                date: date_str,
            });

            day = day
                .checked_add_days(Days::new(1))
                .ok_or_else(|| anyhow!("Failed to increment date"))?;
        }

        habit_data.push(HabitData {
            habit_name: habit,
            data: data_points,
        });
    }

    Ok(habit_data)
}
