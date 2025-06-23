use rusqlite::{Connection, Result};
use std::path::Path;

pub mod queries;

pub fn init_db(path: &str) -> Result<Connection> {
    let db_exists = Path::new(path).exists();
    let conn = Connection::open(path)?;

    if !db_exists {
        println!("Creating new SQLite database...");
    }

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS file_meta (
            file_path TEXT PRIMARY KEY,
            last_modified TEXT
        );

        CREATE TABLE IF NOT EXISTS metrics (
            file_path TEXT PRIMARY KEY,
            name TEXT,
            value INTEGER,
            date TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_date ON metrics(date);
        CREATE INDEX IF NOT EXISTS idx_name ON metrics(name);
        CREATE INDEX IF NOT EXISTS idx_file ON metrics(file_path);
        ",
    )?;

    Ok(conn)
}
