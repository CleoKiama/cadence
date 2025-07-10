use anyhow::Context;
use gtk::Settings;

const LIGHT_THEME_CSS: &str = "./style-light.css";
const DARK_THEME_CSS: &str = "./style-dark.css";

pub fn detect_and_apply_theme() -> Result<Settings, anyhow::Error> {
    let settings =
        Settings::default().ok_or_else(|| anyhow::anyhow!("Could not get default settings"))?;

    let is_dark = settings.is_gtk_application_prefer_dark_theme();
    on_theme_change(is_dark);

    Ok(settings)
}

pub fn on_theme_change(is_dark: bool) {
    if is_dark {
        if let Err(err) = load_css_from_path(DARK_THEME_CSS) {
            eprintln!("Error loading dark theme CSS: {}", err);
        }
    } else if let Err(err) = load_css_from_path(LIGHT_THEME_CSS) {
        eprintln!("Error loading light theme CSS: {}", err);
    }
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
