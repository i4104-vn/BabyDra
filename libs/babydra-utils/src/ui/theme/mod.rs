//! Theme and styling system coordinator.
//! Concatenates component stylesheets and registers them with the GDK Display context.

const DARK_CSS: &str = concat!(
    include_str!("../../styles/dark/panel.css"), "\n",
    include_str!("../../styles/dark/workspaces.css"), "\n",
    include_str!("../../styles/dark/clock.css"), "\n",
    include_str!("../../styles/dark/status.css"), "\n",
    include_str!("../../styles/dark/system_island.css"), "\n",
    include_str!("../../styles/dark/sys_monitor.css"), "\n",
    include_str!("../../styles/dark/tray.css"), "\n",
    include_str!("../../styles/dark/taskbar.css"), "\n",
    include_str!("../../styles/dark/button.css"), "\n",
    include_str!("../../styles/dark/control_center.css"), "\n",
    include_str!("../../styles/dark/launcher.css"), "\n",
    include_str!("../../styles/dark/notification.css"), "\n",
    include_str!("../../styles/dark/calendar.css"), "\n",
    include_str!("../../styles/dark/power.css"), "\n",
    include_str!("../../styles/dark/switcher.css"), "\n",
    include_str!("../../styles/dark/screenshot.css"), "\n",
    include_str!("../../styles/dark/lock.css"), "\n",
    include_str!("../../styles/dark/preview.css"), "\n",
    include_str!("../../styles/dark/settings.css"), "\n",
    include_str!("../../styles/dark/explore.css")
);

const LIGHT_CSS: &str = concat!(
    include_str!("../../styles/light/panel.css"), "\n",
    include_str!("../../styles/light/workspaces.css"), "\n",
    include_str!("../../styles/light/clock.css"), "\n",
    include_str!("../../styles/light/status.css"), "\n",
    include_str!("../../styles/light/system_island.css"), "\n",
    include_str!("../../styles/light/sys_monitor.css"), "\n",
    include_str!("../../styles/light/tray.css"), "\n",
    include_str!("../../styles/light/taskbar.css"), "\n",
    include_str!("../../styles/light/button.css"), "\n",
    include_str!("../../styles/light/control_center.css"), "\n",
    include_str!("../../styles/light/launcher.css"), "\n",
    include_str!("../../styles/light/notification.css"), "\n",
    include_str!("../../styles/light/calendar.css"), "\n",
    include_str!("../../styles/light/power.css"), "\n",
    include_str!("../../styles/light/switcher.css"), "\n",
    include_str!("../../styles/light/screenshot.css"), "\n",
    include_str!("../../styles/light/lock.css"), "\n",
    include_str!("../../styles/light/preview.css"), "\n",
    include_str!("../../styles/light/settings.css"), "\n",
    include_str!("../../styles/light/explore.css")
);

thread_local! {
    static CSS_PROVIDER: gtk4::CssProvider = gtk4::CssProvider::new();
}

use gio::prelude::*;

/// Initializes the GtkCssProvider, registers it with the GdkDisplay,
/// and dynamically loads either the dark or light stylesheet folder.
pub fn init_theme() {
    if let Some(settings) = gtk4::Settings::default() {
        let gsettings = gio::Settings::new("org.gnome.desktop.interface");
        let value = gsettings.string("color-scheme");
        if value == "prefer-dark" {
            settings.set_gtk_application_prefer_dark_theme(true);
        } else if value == "prefer-light" {
            settings.set_gtk_application_prefer_dark_theme(false);
        }

        let user_icon_theme = gsettings.string("icon-theme");
        let user_icon_theme = user_icon_theme.trim();
        settings.set_gtk_icon_theme_name(Some(user_icon_theme));

        if let Some(display) = gtk4::gdk::Display::default() {
            let icon_theme = gtk4::IconTheme::for_display(&display);
            let home = glib::home_dir();
            let local_path = home.join(".local/share/icons");
            if local_path.exists() {
                icon_theme.add_search_path(local_path);
            }
            icon_theme.add_search_path("/usr/share/icons");
            icon_theme.add_search_path("/usr/share/pixmaps");
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
            }
        }

        if let Some(settings) = gtk4::Settings::default() {
            let is_dark = super::icon::is_dark_mode();
            let css = if is_dark { DARK_CSS } else { LIGHT_CSS };
            let cleaned_css = css.replace("\r", "");
            provider.load_from_data(&cleaned_css);

            let provider_clone = provider.clone();
            settings.connect_gtk_application_prefer_dark_theme_notify(move |_s| {
                let is_dark = super::icon::is_dark_mode();
                let css = if is_dark { DARK_CSS } else { LIGHT_CSS };
                let cleaned_css = css.replace("\r", "");
                provider_clone.load_from_data(&cleaned_css);
            });
        } else {
            let cleaned_css = DARK_CSS.replace("\r", "");
            provider.load_from_data(&cleaned_css);
        }
    });
}

/// Helper stub for backward compatibility.
pub fn apply_theme_class(_window: &gtk4::ApplicationWindow) { }

/// Checks if dark mode is preferred in GSettings.
pub fn is_dark_mode() -> bool {
    gtk4::Settings::default()
        .map(|s| s.is_gtk_application_prefer_dark_theme())
        .unwrap_or(true)
}

/// Sets the color scheme preference in GSettings.
pub fn set_dark_mode(dark: bool) {
    let scheme = if dark { "prefer-dark" } else { "prefer-light" };
    let _ = std::process::Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "color-scheme", scheme])
        .output();

    if let Some(settings) = gtk4::Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(dark);
    }

    init_theme();
}

pub fn apply_explore_theme() {
    init_theme();
}
