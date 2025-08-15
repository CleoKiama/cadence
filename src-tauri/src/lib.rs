use std::env;

use tauri::Manager;

mod commands;
mod core;
mod db;

use commands::dashboard::*;
use commands::streaks::*;
use commands::test::*;
use db::Db;
use dotenvy::dotenv;

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
        .setup(move |app| {
            println!("initializing the database");
            //TODO: update the database path as needed later
            let db = Db::new("/tmp/habitron.db")?;
            if let Err(err) = db.init_db() {
                eprintln!("Failed to initialize the database: {}", err);
            }
            app.manage(db.get_connection());

            tauri::async_runtime::spawn({
                let db_clone = db.get_connection();
                async move {
                    if let Err(e) = core::init(db_clone.clone()).await {
                        eprintln!("Error during core initialization: {}", e);
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
            test_command,
            get_dashboard_metrics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
