use gtk::prelude::*;
use gtk::{Application, glib};

mod core;
mod db;

const APP_ID: &str = "org.gtk_rs.habitron";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.run()
}
