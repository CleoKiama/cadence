use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub fn get_all_habits(conn: &Arc<Mutex<Connection>>) -> Result<Vec<String>, rusqlite::Error> {
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT DISTINCT name FROM metrics")?;
    let habit_iter = stmt.query_map([], |row| row.get(0))?;

    let mut habits = Vec::new();
    for habit in habit_iter {
        habits.push(habit?);
    }

    Ok(habits)
}
