use std::env;
use std::sync::{mpsc, Mutex};

use rusqlite::Connection;
use tauri::Manager;

mod commands;
mod core;
mod db;

use commands::analytics::*;
use commands::dashboard::*;
use commands::settings::*;
use commands::streaks::*;
use dotenvy::dotenv;

use crate::commands::recent_activity::get_recent_activity;
use crate::core::file_watcher::WatchCommand;
use crate::core::sync_worker::setup_sync_worker;
use crate::db::utils::get_journal_files_path;
use crate::db::Db;

// Type aliases to prevent runtime panics
pub type DbConnection = Mutex<Connection>;
pub type WatcherState = Mutex<Option<mpsc::Sender<WatchCommand>>>;

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
            let db_path = app.path().cache_dir();
            if db_path.is_err() {
                panic!("Could not determine cache directory for the application");
            }
            let db_path = db_path.unwrap().join("cadance.db");
            let db_path = db_path.to_str().unwrap();

            let db = Db::new(db_path)?;
            if let Err(err) = db.init_db() {
                eprintln!("Failed to initialize the database: {}", err);
            }

            let watcher: WatcherState = Mutex::new(None);
            let tx = setup_sync_worker(app.handle().clone());

            app.manage(DbConnection::new(db.into_connection()));
            app.manage(watcher);
            app.manage(tx);

            tauri::async_runtime::spawn({
                let app_handle = app.handle().clone();

                async move {
                    // Get journal path from settings
                    let db_state = app_handle.state::<DbConnection>();
                    let journal_path = match get_journal_files_path(&db_state) {
                        Ok(path) => path,
                        Err(e) => {
                            eprintln!("Error getting journal path from settings: {}", e);
                            None
                        }
                    };

                    // Initialize core with optional journal path
                    tauri::async_runtime::spawn_blocking({
                        let app_handle_clone = app_handle.clone();
                        move || match core::init(app_handle_clone.clone(), journal_path) {
                            Ok(handle_opt) => {
                                let watcher_state = app_handle_clone.state::<WatcherState>();
                                watcher_state.lock().unwrap().replace(handle_opt);
                            }
                            Err(e) => eprintln!("Error during core initialization: {}", e),
                        }
                    });

                    if seed_database {
                        println!("Seeding database here...");
                        let db_state = app_handle.state::<DbConnection>();
                        if let Err(e) = db::seed::seed_development_data(&db_state) {
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
            //dashboard
            get_current_streak,
            get_longest_streak,
            get_dashboard_metrics,
            get_weekly_metric_stats,
            //streak grid
            get_current_streak_data,
            // analytics
            get_recent_activity,
            // new analytic ones
            get_analytics_summary,
            get_weekly_activity,
            //settings
            get_settings,
            is_journal_path_configured,
            set_journal_files_path,
            add_metric,
            delete_metric,
            udpate_metric,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
