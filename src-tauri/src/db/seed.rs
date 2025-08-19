use chrono::{Days, Local};
use rand::{rng, Rng};
use rusqlite::params;

use crate::{core::read_journal::{Metric, DB_DATE_FORMAT, DB_DATE_TIME_FORMAT}, DbConnection};

pub fn seed_development_data(db: &DbConnection) -> Result<(), anyhow::Error> {
    if is_db_populated(db) {
        println!("Development data already seeded. Skipping...");
        return Ok(());
    }
    println!("Seeding development data...");

    // Seed multiple realistic habits with different patterns
    seed_consistent_habit(db, "exercise", 30, 0.85)?;
    seed_sporadic_habit(db, "meditation", 45, vec![2, 3, 5, 8, 13])?;
    seed_learning_habit(db, "dsa_solved", 60, 0.7)?;
    seed_reading_habit(db, "pages_read", 90, 0.6)?;
    seed_habit_with_breaks(
        db,
        "journal_words",
        120,
        vec![(10, 15), (30, 35), (50, 52)],
    )?;
    seed_recent_habit(db, "water_glasses", 14, 0.9)?;

    println!("Development data seeded successfully!");
    Ok(())
}

fn is_db_populated(db: &DbConnection) -> bool {
    let conn = db.lock().unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM metrics", [], |row| row.get(0))
        .unwrap_or(0);
    count > 0
}

fn seed_consistent_habit(
    db: &DbConnection,
    name: &str,
    days: i32,
    consistency: f64,
) -> Result<(), anyhow::Error> {
    let current_date = Local::now().date_naive();
    for i in 0..days {
        let should_log = rng().random::<f64>() < consistency;
        if should_log {
            let value = match name {
                "exercise" => rng().random_range(20..=90), // minutes
                _ => rng().random_range(1..=5),
            };
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("journal/{}.md", date.format("%Y-%m-%d")),
                name: name.to_string(),
                value,
                date,
            };
            insert_metric(db, &metric)?;
        }
    }
    Ok(())
}

fn seed_sporadic_habit(
    db: &DbConnection,
    name: &str,
    days: i32,
    skip_days: Vec<i32>,
) -> Result<(), anyhow::Error> {
    let current_date = Local::now().date_naive();
    for i in 0..days {
        if !skip_days.contains(&i) {
            let value = match name {
                "meditation" => rng().random_range(5..=30), // minutes
                _ => rng().random_range(1..=3),
            };
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("journal/{}.md", date.format("%Y-%m-%d")),
                name: name.to_string(),
                value,
                date,
            };
            insert_metric(db, &metric)?;
        }
    }
    Ok(())
}

fn seed_learning_habit(
    db: &DbConnection,
    name: &str,
    days: i32,
    consistency: f64,
) -> Result<(), anyhow::Error> {
    let current_date = Local::now().date_naive();
    for i in 0..days {
        let should_log = rng().random::<f64>() < consistency;
        if should_log {
            let value = match name {
                "dsa_solved" => rng().random_range(1..=8), // problems solved
                _ => rng().random_range(1..=5),
            };
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("journal/{}.md", date.format("%Y-%m-%d")),
                name: name.to_string(),
                value,
                date,
            };
            insert_metric(db, &metric)?;
        }
    }
    Ok(())
}

fn seed_reading_habit(
    db: &DbConnection,
    name: &str,
    days: i32,
    consistency: f64,
) -> Result<(), anyhow::Error> {
    let current_date = Local::now().date_naive();
    for i in 0..days {
        let should_log = rng().random::<f64>() < consistency;
        if should_log {
            let value = match name {
                "pages_read" => rng().random_range(5..=50), // pages
                _ => rng().random_range(1..=10),
            };
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("journal/{}.md", date.format("%Y-%m-%d")),
                name: name.to_string(),
                value,
                date,
            };
            insert_metric(db, &metric)?;
        }
    }
    Ok(())
}

fn seed_habit_with_breaks(
    db: &DbConnection,
    name: &str,
    days: i32,
    break_periods: Vec<(i32, i32)>,
) -> Result<(), anyhow::Error> {
    let current_date = Local::now().date_naive();
    for i in 0..days {
        let in_break = break_periods
            .iter()
            .any(|(start, end)| i >= *start && i <= *end);
        if !in_break {
            let value = match name {
                "journal_words" => rng().random_range(100..=1000), // words written
                _ => rng().random_range(1..=10),
            };
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("journal/{}.md", date.format("%Y-%m-%d")),
                name: name.to_string(),
                value,
                date,
            };
            insert_metric(db, &metric)?;
        }
    }
    Ok(())
}

fn seed_recent_habit(
    db: &DbConnection,
    name: &str,
    days: i32,
    consistency: f64,
) -> Result<(), anyhow::Error> {
    let current_date = Local::now().date_naive();
    for i in 0..days {
        let should_log = rng().random::<f64>() < consistency;
        if should_log {
            let value = match name {
                "water_glasses" => rng().random_range(4..=12), // glasses of water
                _ => rng().random_range(1..=8),
            };
            let days_back = i as u64;
            let date = current_date
                .checked_sub_days(Days::new(days_back))
                .ok_or_else(|| anyhow::anyhow!("Invalid date for seeding"))?;
            let metric = Metric {
                file_path: format!("journal/{}.md", date.format("%Y-%m-%d")),
                name: name.to_string(),
                value,
                date,
            };
            insert_metric(db, &metric)?;
        }
    }
    Ok(())
}

pub fn insert_metric(db: &DbConnection, metric: &Metric) -> Result<(), anyhow::Error> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO metrics(file_path, name, value, date,updated_at) VALUES (?1, ?2, ?3, ?4,?5)",
        params![
            metric.file_path,
            metric.name,
            metric.value,
            metric.date.format(DB_DATE_FORMAT).to_string(),
            Local::now().format(DB_DATE_TIME_FORMAT).to_string()
        ],
    )?;
    Ok(())
}
