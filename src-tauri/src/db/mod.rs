use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub mod queries;

pub struct Db {
    pub conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Db {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }

    pub fn init_db(&self) -> Result<()> {
        self.conn.lock().unwrap().execute_batch(
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
        Ok(())
    }
}
