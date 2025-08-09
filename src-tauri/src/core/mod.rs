use anyhow::Context;

pub mod file_watcher;
pub mod needed_metrics;
pub mod read_dailies;
pub mod read_journal;

use read_dailies::read_dailies_dir;

use crate::db::init_db;

pub fn core() -> Result<(), anyhow::Error> {
    let conn = init_db("/tmp/tracker_1.db").with_context(|| "Failed to initialize the database")?;
    let root_dir = "/media/Obsidian_Vaults/10xGoals/Journal/Dailies";
    let needed_metrics =
        needed_metrics::NeededMetrics::new(vec!["did_journal", "dsa_problems_solved"]);
    let dailies = read_dailies_dir(root_dir, &conn).unwrap();
    for daily in dailies {
        let result = read_journal::read_front_matter(&daily, &needed_metrics.metrics, &conn);
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
        }
    }

    println!("Watching for file changes in {}", root_dir);
    file_watcher::watch_files(&conn, root_dir, &needed_metrics.metrics).unwrap();
    Ok(())
}
