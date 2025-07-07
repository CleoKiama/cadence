use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider, glib};

mod current_week;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

pub fn ui_main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    load_css();

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .width_request(12)
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

    // Present window
    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("./style.css");
}
