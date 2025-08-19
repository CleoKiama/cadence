use std::env;

use tauri::Manager;

mod commands;
mod core;
mod db;

use commands::analytics::*;
use commands::dashboard::*;
use commands::settings::*;
use commands::streaks::*;
use db::Db;
use dotenvy::dotenv;

use crate::commands::recent_activity::get_recent_activity;
use crate::core::file_watcher::WatchCommand;
use crate::db::utils::get_journal_files_path;
use std::sync::{Arc, Mutex};
use tauri::async_runtime::Sender;

pub type Watcher = Arc<Mutex<Option<Sender<WatchCommand>>>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(e) = dotenv() {
        eprintln!("Warning: Could not load .env file: {}", e);
    }
    let mut seed_database = false;
    if let Ok(seed_env) = env::var("DB_SEED") {
        if seed_env == "true" {
            println!("Seeding database as per environment variable DB_SEED=true");
            seed_database = true;
        }
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            println!("initializing the database");
            //TODO: update the database path as needed later
            let db = Db::new("/tmp/habitron.db")?;
            if let Err(err) = db.init_db() {
                eprintln!("Failed to initialize the database: {}", err);
            }

            let watcher: Watcher = Arc::new(Mutex::new(None));
            app.manage(db.get_connection());
            app.manage(watcher.clone());

            tauri::async_runtime::spawn({
                let db_clone = db.get_connection();
                let watcher_clone = watcher.clone();

                async move {
                    // Get journal path from settings
                    let journal_path = match get_journal_files_path(&db_clone) {
                        Ok(path) => path,
                        Err(e) => {
                            eprintln!("Error getting journal path from settings: {}", e);
                            None
                        }
                    };

                    // Initialize core with optional journal path
                    match core::init(db_clone.clone(), journal_path).await {
                        Ok(handle_opt) => {
                            watcher_clone.lock().unwrap().replace(handle_opt);
                        }
                        Err(e) => eprintln!("Error during core initialization: {}", e),
                    }

                    if seed_database {
                        println!("Seeding database here...");
                        if let Err(e) = db::seed::seed_development_data(db_clone) {
                            eprintln!("Error seeding database: {}", e);
                        } else {
                            println!("Database seeded successfully!");
                        }
                    }
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_current_streak,
            get_longest_streak,
            get_dashboard_metrics,
            get_recent_activity,
            get_analytics_heatmap_data,
            get_all_analytics_data,
            get_settings,
            set_journal_files_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
