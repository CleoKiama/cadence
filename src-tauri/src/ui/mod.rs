use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

use crate::ui::ui_theme::{detect_and_apply_theme, on_theme_change};

mod current_week;
mod ui_theme;

const APP_ID: &str = "org.gtk_rs.habitron";

pub fn ui_main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    if let Ok(settings) = detect_and_apply_theme() {
        settings.connect_gtk_theme_name_notify(move |settings| {
            let is_dark = settings.is_gtk_application_prefer_dark_theme();
            on_theme_change(is_dark);
        });
    } else {
        eprintln!("something went wrong failed to load the css")
    }

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .css_classes(["day-container"])
        .build();

    let day_buttons = current_week::current_week();
    for button in day_buttons {
        button.add_css_class("day-button");
        gtk_box.append(&button);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.present();
}
