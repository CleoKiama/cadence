use rusqlite::{Connection, Result};

pub mod metrics;
pub mod seed;
pub mod streaks;
pub mod utils;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Db { conn })
    }

    pub fn into_connection(self) -> Connection {
        self.conn
    }

    pub fn init_db(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS file_meta (
                file_path TEXT PRIMARY KEY,
                last_modified TEXT
            );

            CREATE TABLE IF NOT EXISTS metrics (
                file_path TEXT NOT NULL,
                name TEXT NOT NULL,
                value INTEGER,
                date TEXT NOT NULL,
                updated_at TEXT,
                PRIMARY KEY (file_path, name, date)
            );

            CREATE TABLE IF NOT EXISTS tracked_metrics (
                value TEXT PRIMARY KEY
            );

            CREATE TABLE IF NOT EXISTS journals_files_path (
                value TEXT PRIMARY KEY
            );

            CREATE INDEX IF NOT EXISTS idx_file ON metrics(file_path);
            ",
        )?;
        Ok(())
    }
}
