use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tauri::async_runtime::Sender;

pub mod file_watcher;
pub mod needed_metrics;
pub mod read_dailies;
pub mod read_journal;

use read_dailies::read_dailies_dir;

use crate::core::file_watcher::WatchCommand;

pub async fn init(
    db: Arc<Mutex<Connection>>,
    journal_path: Option<String>,
) -> Result<Sender<WatchCommand>, anyhow::Error> {
    let needed_metrics = vec!["dsa_problems_solved", "exercise"];

    if let Some(root_dir) = &journal_path {
        let dailies = read_dailies_dir(root_dir, db.clone()).await?;
        for daily in dailies {
            let result = read_journal::read_front_matter(&daily, &needed_metrics, db.clone()).await;
            match result {
                Ok(_) => (),
                Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
            }
        }
    };

    let watcher = file_watcher::start_watcher(db.clone(), journal_path).await?;
    Ok(watcher)
}
