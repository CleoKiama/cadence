use rusqlite::Connection;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::State;

use crate::db::metrics::{get_all_habits_analytics, get_habit_heatmap_data};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsTrendData {
    pub habit_name: String,
    pub data: Vec<ChartDataPoint>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartDataPoint {
    pub date: String,
    pub value: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsHeatmapData {
    pub habit_name: String,
    pub data: Vec<HeatmapPoint>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatmapPoint {
    pub date: String,
    pub count: i64,
    pub level: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsSummary {
    pub total_habits: u32,
    pub active_habits: u32,
    pub total_entries: u32,
    pub time_range: String,
}

#[tauri::command]
pub fn get_analytics_heatmap_data(
    state: State<'_, Arc<Mutex<Connection>>>,
    habit_name: String,
    days: u32,
) -> Result<AnalyticsHeatmapData, String> {
    let heatmap_data =
        get_habit_heatmap_data(&state, &habit_name, days).map_err(|e| e.to_string())?;

    let chart_data = heatmap_data
        .into_iter()
        .map(|point| HeatmapPoint {
            date: point.date,
            count: point.count,
            level: point.level,
        })
        .collect();

    Ok(AnalyticsHeatmapData {
        habit_name,
        data: chart_data,
    })
}

#[tauri::command]
pub fn get_all_analytics_data(
    state: State<'_, Arc<Mutex<Connection>>>,
    days: u32,
) -> Result<Vec<AnalyticsTrendData>, String> {
    let all_data = get_all_habits_analytics(&state, days).map_err(|e| e.to_string())?;

    let result = all_data
        .into_iter()
        .map(|(habit_name, trend_data)| {
            let chart_data = trend_data
                .into_iter()
                .map(|point| ChartDataPoint {
                    date: point.date,
                    value: point.value,
                })
                .collect();

            AnalyticsTrendData {
                habit_name,
                data: chart_data,
            }
        })
        .collect();

    Ok(result)
}

#[tauri::command]
pub fn get_analytics_summary(
    state: State<'_, Arc<Mutex<Connection>>>,
    days: u32,
) -> Result<AnalyticsSummary, String> {
    let all_data = get_all_habits_analytics(&state, days).map_err(|e| e.to_string())?;
    let conn = state.lock().unwrap();
    let total_habits: u32 = conn
        .query_row("SELECT COUNT(DISTINCT name) FROM metrics", [], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;

    let active_habits = all_data
        .values()
        .filter(|data| data.iter().any(|point| point.value > 0))
        .count() as u32;

    let total_entries: u32 = all_data
        .values()
        .map(|data| data.iter().map(|point| point.value as u32).sum::<u32>())
        .sum();

    let time_range = format!("Last {} days", days);

    Ok(AnalyticsSummary {
        total_habits,
        active_habits,
        total_entries,
        time_range,
    })
}

