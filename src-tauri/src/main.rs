// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod db;

fn main() {
    habitron_lib::run();
    let conn = db::Db::new("/tmp/habitron.db");

    match conn {
        Ok(db) => db.init_db(),
        Err(e) => eprintln!("Failed to initialize database: {}", e),
    }
}
