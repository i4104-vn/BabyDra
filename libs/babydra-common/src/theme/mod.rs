
//! Theme and styling system coordinator.
//! Concatenates component stylesheets and registers them with the GDK Display context.

const DARK_CSS: &str = concat!(
    include_str!("styles/dark/panel.css"), "\n",
    include_str!("styles/dark/workspaces.css"), "\n",
    include_str!("styles/dark/clock.css"), "\n",
    include_str!("styles/dark/status.css"), "\n",
    include_str!("styles/dark/system_island.css"), "\n",
    include_str!("styles/dark/sys_monitor.css"), "\n",
    include_str!("styles/dark/tray.css"), "\n",
    include_str!("styles/dark/taskbar.css"), "\n",
    include_str!("styles/dark/button.css"), "\n",
    include_str!("styles/dark/control_center.css"), "\n",
    include_str!("styles/dark/launcher.css"), "\n",
    include_str!("styles/dark/notification.css"), "\n",
    include_str!("styles/dark/calendar.css"), "\n",
    include_str!("styles/dark/power.css"), "\n",
    include_str!("styles/dark/menu.css"), "\n",
    include_str!("styles/dark/switcher.css"), "\n",
    include_str!("styles/dark/screenshot.css"), "\n",
    include_str!("styles/dark/lock.css")
);

const LIGHT_CSS: &str = concat!(
    include_str!("styles/light/panel.css"), "\n",
    include_str!("styles/light/workspaces.css"), "\n",
    include_str!("styles/light/clock.css"), "\n",
    include_str!("styles/light/status.css"), "\n",
    include_str!("styles/light/system_island.css"), "\n",
    include_str!("styles/light/sys_monitor.css"), "\n",
    include_str!("styles/light/tray.css"), "\n",
    include_str!("styles/light/taskbar.css"), "\n",
    include_str!("styles/light/button.css"), "\n",
    include_str!("styles/light/control_center.css"), "\n",
    include_str!("styles/light/launcher.css"), "\n",
    include_str!("styles/light/notification.css"), "\n",
    include_str!("styles/light/calendar.css"), "\n",
    include_str!("styles/light/power.css"), "\n",
    include_str!("styles/light/menu.css"), "\n",
    include_str!("styles/light/switcher.css"), "\n",
    include_str!("styles/light/screenshot.css"), "\n",
    include_str!("styles/light/lock.css")
);

thread_local! {
    static CSS_PROVIDER: gtk4::CssProvider = gtk4::CssProvider::new();
}

/// Initializes the GtkCssProvider, registers it with the GdkDisplay,
/// and dynamically loads either the dark or light stylesheet folder.
pub fn init_theme() {
    if let Some(settings) = gtk4::Settings::default() {
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.interface", "color-scheme"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let value = stdout.trim().trim_matches('\'');
            if value == "prefer-dark" {
                settings.set_gtk_application_prefer_dark_theme(true);
            } else if value == "prefer-light" {
                settings.set_gtk_application_prefer_dark_theme(false);
            }
        }
    }

    CSS_PROVIDER.with(|provider| {
        thread_local! {
            static REGISTERED: std::cell::Cell<bool> = std::cell::Cell::new(false);
        }

        let is_registered = REGISTERED.with(|r| r.get());
        if !is_registered {
            if let Some(display) = gtk4::gdk::Display::default() {
                gtk4::style_context_add_provider_for_display(
                    &display,
                    provider,
                    gtk4::STYLE_PROVIDER_PRIORITY_USER,
                );
                REGISTERED.with(|r| r.set(true));
                println!("Successfully registered glassmorphism stylesheet with GTK Display.");
            }
        }

        if let Some(settings) = gtk4::Settings::default() {
            let is_dark = crate::icon::is_dark_mode();
            let css = if is_dark { DARK_CSS } else { LIGHT_CSS };
            let cleaned_css = css.replace("\r", "");
            provider.load_from_data(&cleaned_css);

            let provider_clone = provider.clone();
            settings.connect_gtk_application_prefer_dark_theme_notify(move |_s| {
                let is_dark = crate::icon::is_dark_mode();
                let css = if is_dark { DARK_CSS } else { LIGHT_CSS };
                let cleaned_css = css.replace("\r", "");
                provider_clone.load_from_data(&cleaned_css);
                println!("Dynamic theme re-loaded (is_dark = {}).", is_dark);
            });
        } else {
            let cleaned_css = DARK_CSS.replace("\r", "");
            provider.load_from_data(&cleaned_css);
        }
    });
}

/// Helper stub for backward compatibility.
pub fn apply_theme_class(_window: &gtk4::ApplicationWindow) { }

