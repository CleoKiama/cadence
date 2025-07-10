use anyhow::Context;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Settings, glib};

mod current_week;

const APP_ID: &str = "org.gtk_rs.habitron";

pub fn ui_main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    if let Err(err) = detect_and_apply_theme() {
        eprintln!("something went wrong failed to load the css {}", err)
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

fn detect_and_apply_theme() -> Result<(), anyhow::Error> {
    let settings =
        Settings::default().ok_or_else(|| anyhow::anyhow!("Could not get default settings"))?;

    let is_dark = settings.is_gtk_application_prefer_dark_theme();
    if is_dark {
        load_dark_theme_css()?
    } else {
        load_light_theme_css()?
    }

    Ok(())
}

fn load_dark_theme_css() -> Result<(), anyhow::Error> {
    load_css_from_path("./style-dark.css")
}

fn load_light_theme_css() -> Result<(), anyhow::Error> {
    load_css_from_path("./style-light.css")
}

fn load_css_from_path(path: &str) -> Result<(), anyhow::Error> {
    let provider = gtk::CssProvider::new();
    let css_file_path = std::path::Path::new(path);
    provider.load_from_path(css_file_path);

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().with_context(|| "failed to load the css")?,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Ok(())
}
