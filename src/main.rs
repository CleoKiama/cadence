use ui::ui_main;

mod core;
mod db;
mod ui;

const APP_ID: &str = "org.gtk_rs.habitron";

fn main() {
    ui_main();
}
