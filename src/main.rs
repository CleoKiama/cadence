use crate::db::init_db;
use crate::db::queries::get_metrics_by_date;
use crate::read_dailies::read_dailies_dir;
use crate::read_journal::read_front_matter;
use anyhow::Context;

mod db;
mod read_dailies;
mod read_journal;

fn main() {
    let conn = init_db("/tmp/tracker_1.db")
        .with_context(|| "Failed to initialize the database")
        .unwrap();
    let needed_metrics = vec!["did_journal", "dsa_problems_solved"];
    let dailies = read_dailies_dir("/home/cleo2/vault/10xGoals/Journal/Dailies", &conn).unwrap();
    for daily in dailies {
        let result = read_front_matter(&daily, &needed_metrics, &conn);
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
        }
    }
    println!("Done reading dailies");

    let metrics_count = get_metrics_by_date(&conn, (2025, 2, 10), (2025, 2, 20)).unwrap();
    for metric in metrics_count {
        println!(
            "Metric: {}, Value: {}, Date: {}",
            metric.name, metric.value, metric.date
        );
    }
}
