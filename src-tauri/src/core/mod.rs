use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub mod file_watcher;
pub mod needed_metrics;
pub mod read_dailies;
pub mod read_journal;

use read_dailies::read_dailies_dir;

pub async fn init(db: Arc<Mutex<Connection>>) -> Result<(), anyhow::Error> {
    let root_dir = "/media/Obsidian_Vaults/10xGoals/Journal/Dailies"; //TODO: This should come from the UI
    let needed_metrics =
        needed_metrics::NeededMetrics::new(vec!["did_journal", "dsa_problems_solved"]); //TODO: This should come from the UI
    let dailies = read_dailies_dir(root_dir, db.clone()).await?;
    for daily in dailies {
        let result =
            read_journal::read_front_matter(&daily, &needed_metrics.metrics, db.clone()).await;
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
        }
    }

    println!("Watching for file changes in {}", root_dir);
    file_watcher::watch_files(db.clone(), root_dir, &needed_metrics.metrics)
        .await
        .unwrap();
    Ok(())
}
