use std::env;
use std::sync::Mutex;

use tauri::Manager;
use rusqlite::Connection;

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
use tauri::async_runtime::Sender;

// Type aliases to prevent runtime panics
pub type DbConnection = Mutex<Connection>;
pub type WatcherState = Mutex<Option<Sender<WatchCommand>>>;

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

            let watcher: WatcherState = Mutex::new(None);
            app.manage(DbConnection::new(db.into_connection()));
            app.manage(watcher);

            tauri::async_runtime::spawn({
                let app_handle = app.handle().clone();

                async move {
                    // Get journal path from settings
                    let db_state = app_handle.state::<DbConnection>();
                    let journal_path = match get_journal_files_path(&*db_state) {
                        Ok(path) => path,
                        Err(e) => {
                            eprintln!("Error getting journal path from settings: {}", e);
                            None
                        }
                    };

                    // Initialize core with optional journal path
                    match core::init(app_handle.clone(), journal_path).await {
                        Ok(handle_opt) => {
                            let watcher_state = app_handle.state::<WatcherState>();
                            watcher_state.lock().unwrap().replace(handle_opt);
                        }
                        Err(e) => eprintln!("Error during core initialization: {}", e),
                    }

                    if seed_database {
                        println!("Seeding database here...");
                        let db_state = app_handle.state::<DbConnection>();
                        if let Err(e) = db::seed::seed_development_data(&*db_state) {
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
