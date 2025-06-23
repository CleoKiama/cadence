use crate::db::init_db;
use crate::file_watcher::watch_files;
use crate::needed_metrics::NeededMetrics;
use crate::read_dailies::read_dailies_dir;
use crate::read_journal::read_front_matter;

use anyhow::Context;

mod db;
mod file_watcher;
mod needed_metrics;
mod read_dailies;
mod read_journal;

fn main() {
    let conn = init_db("/tmp/tracker_1.db")
        .with_context(|| "Failed to initialize the database")
        .unwrap();
    let root_dir = "/home/cleo2/Documents/Obsidian_Vaults/10xGoals/Journal/Dailies";
    let needed_metrics = NeededMetrics::new(vec!["did_journal", "dsa_problems_solved"]);
    let dailies = read_dailies_dir(root_dir, &conn).unwrap();
    for daily in dailies {
        let result = read_front_matter(&daily, &needed_metrics.metrics, &conn);
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
        }
    }

    println!("Watching for file changes in {}", root_dir);
    watch_files(&conn, root_dir, &needed_metrics.metrics).unwrap();
}
