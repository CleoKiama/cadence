use std::collections::HashMap;

use anyhow::anyhow;
use chrono::{self, Days, Local, NaiveDate};
use serde::Serialize;
use tauri::State;

use crate::{
    core::read_journal::DB_DATE_FORMAT,
    db::{
        metrics,
        streaks::{get_habit_streak, get_longest_habit_streak},
        utils::get_all_habits,
    },
    DbConnection,
};

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Trend {
    Stable,
    Up, //TODO: updae to use this two variants
    Down,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardMetrics {
    name: String,
    current_streak: i64,
    longest_streak: i64,
    weekly_average: i32,
    display_name: String,
    last_updated: String,
    monthly_total: u32,
    trend: Trend,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct WeeklyMetricStat {
    date: String,
    value: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyMetrics {
    prev_week: Vec<WeeklyMetricStat>,
    current_week: Vec<WeeklyMetricStat>,
}

#[tauri::command]
pub fn get_dashboard_metrics(
    db: State<'_, DbConnection>,
) -> Result<Option<Vec<DashboardMetrics>>, String> {
    let habit_metrics = get_habit_metrics(&db)?;

    if habit_metrics.is_empty() {
        Ok(None)
    } else {
        Ok(Some(habit_metrics))
    }
}

#[tauri::command]
pub fn get_weekly_metric_stats(
    db: State<'_, DbConnection>,
    habit_name: &str,
    week_starts_on: &str,
) -> Result<WeeklyMetrics, String> {
    let result = weekly_metric_stats(&db, habit_name, week_starts_on)
        .map_err(|e| format!("error getting weekly_metrics, {}", e))?;
    Ok(result)
}

fn weekly_metric_stats(
    db: &DbConnection,
    habit_name: &str,
    week_starts_on: &str,
) -> Result<WeeklyMetrics, anyhow::Error> {
    let today = Local::now().date_naive();
    let first_day_of_week = week_starts_on.parse::<chrono::Weekday>()?;
    let first_day_current_week = today.week(first_day_of_week).first_day();
    // get the days diff from first day of the week to today
    let diff = today - first_day_current_week;
    let last_day_current_week = first_day_current_week
        .checked_add_signed(diff)
        .ok_or_else(|| anyhow!("Error in weekly_metric_stats adding duration days"))?;

    let prev_week = today
        .checked_sub_days(Days::new(7))
        .ok_or_else(|| anyhow!("could get previous week in weekly_metrics"))?
        .week(first_day_of_week);

    let prev_week_data = get_weekly_data(
        db,
        habit_name,
        prev_week.first_day(),
        prev_week.last_day(),
        7,
    )?;

    let current_week_data = get_weekly_data(
        db,
        habit_name,
        first_day_current_week,
        last_day_current_week,
        (diff.num_days() + 1) as usize, // today included in the diff
    )?;

    let result = WeeklyMetrics {
        prev_week: prev_week_data,
        current_week: current_week_data,
    };

    Ok(result)
}

fn get_weekly_data(
    db: &DbConnection,
    habit_name: &str,
    start: NaiveDate,
    end: NaiveDate,
    num_of_days: usize,
) -> Result<Vec<WeeklyMetricStat>, anyhow::Error> {
    let conn = db.lock().unwrap();
    let mut stmt =
        conn.prepare("SELECT date,value from metrics where name=?1 and date between ?2 and ?3")?;
    let rows = stmt.query_map(
        [
            habit_name,
            &start.format(DB_DATE_FORMAT).to_string(),
            &end.format(DB_DATE_FORMAT).to_string(),
        ],
        |row| Ok((row.get::<_, String>(0), row.get::<_, i32>(1))),
    )?;

    let mut data: HashMap<String, i32> = HashMap::with_capacity(7);
    for row in rows {
        let row = row.map_err(|e| {
            anyhow!(format!(
                "Error parsing the row getting weekly metric: {}",
                e
            ))
        })?;
        let (date_result, value_result) = row;
        let date = date_result
            .map_err(|e| anyhow!(format!("error parsing the date from the db ,{}", e)))?;
        let value = value_result
            .map_err(|e| anyhow!(format!("error parsing the date from the db ,{}", e)))?;

        data.insert(date, value);
    }

    let mut result: Vec<WeeklyMetricStat> = Vec::with_capacity(7);
    for date in start.iter_days().take(num_of_days) {
        let date_string = date.format(DB_DATE_FORMAT).to_string();
        // default to 0 if there was not a value from the db
        let value = data.get(&date_string).unwrap_or(&0);
        let entry = WeeklyMetricStat {
            date: date_string,
            value: *value,
        };
        result.push(entry)
    }

    Ok(result)
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use anyhow::Context;

    use super::*;
    // use crate::db::seed;
    use crate::db::metrics;
    use crate::db::Db;

    fn init_db() -> Result<DbConnection, anyhow::Error> {
        let db_path = "/tmp/habitron.db";
        let db = Db::new(db_path)
            .with_context(|| "something went wrong initializing the database".to_string())?;

        db.init_db().with_context(|| "Error seeding the database")?;
        let db_conn = Mutex::new(db.into_connection());
        // seed::seed_development_data(&db_conn)?;
        Ok(db_conn)
    }

    #[test]
    fn test_weekly_metrics() -> Result<(), anyhow::Error> {
        let db_con = init_db()?;
        let res = weekly_metric_stats(&db_con, "pages_read", "Sun")?;
        println!("prev week: {:#?}", res.prev_week);
        println!("current week: {:#?}", res.current_week);
        Ok(())
    }
    #[test]
    fn get_weekly_average() -> Result<(), anyhow::Error> {
        let db_con = init_db()?;
        let result = metrics::get_weekly_metric_avg(&db_con, "dsa_problems_solved")?;
        println!("result: {:#?}", result);
        Ok(())
    }
}
