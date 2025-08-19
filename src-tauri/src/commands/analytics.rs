use serde::Serialize;
use tauri::State;

use crate::{db::metrics::{get_all_habits_analytics, get_habit_heatmap_data}, DbConnection};

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
    pub level: u8, // 0-4 for intensity levels
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
    db: State<'_, DbConnection>,
    habit_name: String,
    days: u32,
) -> Result<AnalyticsHeatmapData, String> {
    let heatmap_data =
        get_habit_heatmap_data(&db, &habit_name, days).map_err(|e| e.to_string())?;

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
    db: State<'_, DbConnection>,
    days: u32,
) -> Result<Vec<AnalyticsTrendData>, String> {
    let all_data = get_all_habits_analytics(&db, days).map_err(|e| e.to_string())?;

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
