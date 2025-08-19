use tauri::{async_runtime::Sender, AppHandle, Manager};

pub mod file_watcher;
pub mod needed_metrics;
pub mod read_dailies;
pub mod read_journal;

use read_dailies::read_dailies_dir;

use crate::{core::file_watcher::WatchCommand, DbConnection};

pub async fn init(
    app_handle: AppHandle,
    journal_path: Option<String>,
) -> Result<Sender<WatchCommand>, anyhow::Error> {
    let needed_metrics = vec!["dsa_problems_solved", "exercise"];
    let db = app_handle.state::<DbConnection>();

    if let Some(root_dir) = &journal_path {
        let dailies = read_dailies_dir(root_dir, &*db).await?;
        for daily in dailies {
            let result = read_journal::read_front_matter(&daily, &needed_metrics, &*db).await;
            match result {
                Ok(_) => (),
                Err(e) => eprintln!("Error reading front matter for {}: {}", daily, e),
            }
        }
    };

    let watcher = file_watcher::start_watcher(app_handle, journal_path).await?;
    Ok(watcher)
}
