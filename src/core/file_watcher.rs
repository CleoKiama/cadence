use notify::{Event, RecursiveMode, Result, Watcher, recommended_watcher};
use std::{collections::HashSet, path::Path, sync::mpsc::channel};

pub fn watch_files(
    conn: &rusqlite::Connection,
    dir: &str,
    needed_metrics: &Vec<String>,
) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new(dir), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = sync_data(event, conn, needed_metrics) {
                    eprintln!("Error syncing data: {}", e);
                }
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
    Ok(())
}

pub fn sync_data(
    event: Event,
    conn: &rusqlite::Connection,
    needed_metrics: &Vec<String>,
) -> Result<()> {
    let mut visited_paths: HashSet<String> = HashSet::new();
    for path in event.paths {
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let file_path = path.to_str();
            if let Some(file_path) = file_path {
                if visited_paths.contains(file_path) {
                    println!("Skipping already visited file: {}", file_path);
                    continue; // Skip already visited paths
                }
                match super::read_journal::read_front_matter(file_path, needed_metrics, conn) {
                    Ok(_) => println!("Successfully processed: {}", file_path),
                    Err(e) => eprintln!("Error processing {}: {}", file_path, e),
                }

                visited_paths.insert(file_path.to_string());
            }
        }
    }
    Ok(())
}
