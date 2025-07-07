use chrono::{Days, Local};
use gtk::Button;

pub fn current_week() -> Vec<Button> {
    let days = get_days();
    days.into_iter()
        .map(|day| Button::builder().label(format!("{} ✅", day)).build())
        .collect()
}

fn get_days() -> Vec<String> {
    let i = 7;
    let mut past_week: Vec<String> = Vec::with_capacity(7);
    for i in (0..i).rev() {
        let now = Local::now().checked_sub_days(Days::new(i)).unwrap();
        let formatted = format!("{}", now.format("%a"));
        past_week.push(formatted);
    }
    past_week
}
