use crate::DbConnection;

pub fn get_all_habits(db: &DbConnection) -> Result<Vec<String>, rusqlite::Error> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT DISTINCT name FROM metrics")?;
    let habit_iter = stmt.query_map([], |row| row.get(0))?;

    let mut habits = Vec::new();
    for habit in habit_iter {
        habits.push(habit?);
    }

    Ok(habits)
}

pub fn get_journal_files_path(
    db: &DbConnection,
) -> Result<Option<String>, anyhow::Error> {
    use rusqlite::params;

    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock connection: {}", e))?;

    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;

    match stmt.query_row(params!["journal_files_path"], |row| row.get(0)) {
        Ok(path) => Ok(Some(path)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(anyhow::anyhow!("Failed to get journal files path: {}", e)),
    }
}
