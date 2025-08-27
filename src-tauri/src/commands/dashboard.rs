use serde::Serialize;
use tauri::State;

use crate::{
    db::metrics,
    db::streaks::{get_habit_streak, get_longest_habit_streak},
    db::utils::get_all_habits,
    DbConnection,
};

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Trend {
    Stable,
    Up,
    Down,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardMetrics {
    name: String,
    current_streak: i64,
    longest_streak: i64,
    weekly_average: f32,
    display_name: String,
    last_updated: String,
    monthly_total: u32,
    trend: Trend,
}

#[tauri::command]
pub fn get_dashboard_metrics(db: State<'_, DbConnection>) -> Result<Option<Vec<DashboardMetrics>>, String> {
    let habit_metrics = get_habit_metrics(&db)?;

    if habit_metrics.is_empty() {
        Ok(None)
    } else {
        Ok(Some(habit_metrics))
    }
}

fn get_habit_metrics(db: &DbConnection) -> Result<Vec<DashboardMetrics>, String> {
    let habits = get_all_habits(db).map_err(|e| e.to_string())?;
    let mut metrics_list = Vec::with_capacity(habits.len());
    for habit in habits {
        let summary_metric = get_summary_metric(db, &habit)?;
        metrics_list.push(summary_metric);
    }

    Ok(metrics_list)
}

fn get_summary_metric(db: &DbConnection, habit_name: &str) -> Result<DashboardMetrics, String> {
    let current_streak = get_habit_streak(db, habit_name).map_err(|e| e.to_string())?;
    let longest_streak = get_longest_habit_streak(db, habit_name).map_err(|e| e.to_string())?;
    let weekly_avg = metrics::get_weekly_metric_avg(db, habit_name).map_err(|e| e.to_string())?;
    let monthly_total =
        metrics::get_monthly_metric_total(db, habit_name).map_err(|e| e.to_string())?;

    let display_name = habit_name.to_string(); //HACK: Placeholder for display name logic
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT updated_at FROM metrics WHERE name = ?1 ORDER BY updated_at DESC LIMIT 1")
        .map_err(|e| e.to_string())?;
    let last_updated: String = stmt
        .query_one([habit_name], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let trend = Trend::Stable; //HACK: Placeholder for trend logic

    Ok(DashboardMetrics {
        name: habit_name.to_string(),
        last_updated,
        display_name,
        current_streak,
        longest_streak,
        weekly_average: weekly_avg,
        monthly_total,
        trend,
    })
}
