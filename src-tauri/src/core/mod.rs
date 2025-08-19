use tauri::{async_runtime::Sender, AppHandle, Manager};

pub mod file_watcher;
pub mod needed_metrics;
pub mod read_dailies;
pub mod read_journal;

use read_dailies::read_dailies_dir;

use crate::{core::file_watcher::WatchCommand, DbConnection};

fn get_tracked_metrics_from_db(db: &DbConnection) -> Result<Vec<String>, anyhow::Error> {
    let conn = db
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to lock connection: {}", e))?;

    let mut stmt = conn.prepare("SELECT value FROM tracked_metrics")?;
    let metric_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;

    let mut metrics = Vec::new();
    for metric in metric_iter {
        metrics.push(metric?);
    }

    Ok(metrics)
}

pub async fn resync_database(
    db: &DbConnection,
    journal_path: &str,
) -> Result<(), anyhow::Error> {
    let tracked_metrics = get_tracked_metrics_from_db(db)?;
    
    if tracked_metrics.is_empty() {
        return Ok(());
    }

    let needed_metrics: Vec<&str> = tracked_metrics.iter().map(|s| s.as_str()).collect();
    
    let dailies = read_dailies_dir(journal_path, db).await?;
    for daily in dailies {
        let result = read_journal::read_front_matter(&daily, &needed_metrics, db).await;
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
        }
    }
    
    Ok(())
}

pub async fn init(
    app_handle: AppHandle,
    journal_path: Option<String>,
) -> Result<Sender<WatchCommand>, anyhow::Error> {
    let db = app_handle.state::<DbConnection>();
    
    let tracked_metrics = get_tracked_metrics_from_db(&db)?;
    
    if let Some(root_dir) = &journal_path {
        if !tracked_metrics.is_empty() {
            let needed_metrics: Vec<&str> = tracked_metrics.iter().map(|s| s.as_str()).collect();
            let dailies = read_dailies_dir(root_dir, &db).await?;
            for daily in dailies {
                let result = read_journal::read_front_matter(&daily, &needed_metrics, &db).await;
                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
                }
            }
        }
    };

    let watcher = file_watcher::start_watcher(app_handle, journal_path).await?;
    Ok(watcher)
}
