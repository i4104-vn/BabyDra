//! Theme and styling system coordinator.
//! Concatenates component stylesheets and registers them with the GDK Display context.

const DARK_CSS: &str = concat!(
    include_str!("../../styles/dark/panel/panel.css"), "\n",
    include_str!("../../styles/dark/panel/workspaces.css"), "\n",
    include_str!("../../styles/dark/panel/clock.css"), "\n",
    include_str!("../../styles/dark/panel/status.css"), "\n",
    include_str!("../../styles/dark/panel/sys_monitor.css"), "\n",
    include_str!("../../styles/dark/panel/tray.css"), "\n",
    include_str!("../../styles/dark/panel/taskbar.css"), "\n",
    include_str!("../../styles/dark/control_center/control_center.css"), "\n",
    include_str!("../../styles/dark/control_center/power.css"), "\n",
    include_str!("../../styles/dark/island/system_island.css"), "\n",
    include_str!("../../styles/dark/island/notification.css"), "\n",
    include_str!("../../styles/dark/launcher/launcher.css"), "\n",
    include_str!("../../styles/dark/calendar/calendar.css"), "\n",
    include_str!("../../styles/dark/shared/button.css"), "\n",
    include_str!("../../styles/dark/shared/switch.css"), "\n",
    include_str!("../../styles/dark/apps/screenshot.css"), "\n",
    include_str!("../../styles/dark/apps/lock.css"), "\n",
    include_str!("../../styles/dark/apps/preview.css"), "\n",
    include_str!("../../styles/dark/apps/settings.css"), "\n",
    include_str!("../../styles/dark/apps/switcher.css"), "\n",
    include_str!("../../styles/dark/explore/window.css"), "\n",
    include_str!("../../styles/dark/explore/header_bar.css"), "\n",
    include_str!("../../styles/dark/explore/sidebar.css"), "\n",
    include_str!("../../styles/dark/explore/content_view.css"), "\n",
    include_str!("../../styles/dark/explore/info_panel.css"), "\n",
    include_str!("../../styles/dark/explore/status_bar.css"), "\n",
    include_str!("../../styles/dark/explore/context_menu.css"), "\n",
    include_str!("../../styles/dark/explore/dialogs.css"), "\n",
    include_str!("../../styles/dark/shared/scrollbar.css")
);

const LIGHT_CSS: &str = concat!(
    include_str!("../../styles/light/panel/panel.css"), "\n",
    include_str!("../../styles/light/panel/workspaces.css"), "\n",
    include_str!("../../styles/light/panel/clock.css"), "\n",
    include_str!("../../styles/light/panel/status.css"), "\n",
    include_str!("../../styles/light/panel/sys_monitor.css"), "\n",
    include_str!("../../styles/light/panel/tray.css"), "\n",
    include_str!("../../styles/light/panel/taskbar.css"), "\n",
    include_str!("../../styles/light/control_center/control_center.css"), "\n",
    include_str!("../../styles/light/control_center/power.css"), "\n",
    include_str!("../../styles/light/island/system_island.css"), "\n",
    include_str!("../../styles/light/island/notification.css"), "\n",
    include_str!("../../styles/light/launcher/launcher.css"), "\n",
    include_str!("../../styles/light/calendar/calendar.css"), "\n",
    include_str!("../../styles/light/shared/button.css"), "\n",
    include_str!("../../styles/light/shared/switch.css"), "\n",
    include_str!("../../styles/light/apps/screenshot.css"), "\n",
    include_str!("../../styles/light/apps/lock.css"), "\n",
    include_str!("../../styles/light/apps/preview.css"), "\n",
    include_str!("../../styles/light/apps/settings.css"), "\n",
    include_str!("../../styles/light/apps/switcher.css"), "\n",
    include_str!("../../styles/light/explore/window.css"), "\n",
    include_str!("../../styles/light/explore/header_bar.css"), "\n",
    include_str!("../../styles/light/explore/sidebar.css"), "\n",
    include_str!("../../styles/light/explore/content_view.css"), "\n",
    include_str!("../../styles/light/explore/info_panel.css"), "\n",
    include_str!("../../styles/light/explore/status_bar.css"), "\n",
    include_str!("../../styles/light/explore/context_menu.css"), "\n",
    include_str!("../../styles/light/explore/dialogs.css"), "\n",
    include_str!("../../styles/light/shared/scrollbar.css")
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
        let is_dark = value != "prefer-light";
        settings.set_gtk_application_prefer_dark_theme(is_dark);

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

            let gsettings = gio::Settings::new("org.gnome.desktop.interface");
            
            let gsettings_c = gsettings.clone();
            gsettings.connect_changed(Some("color-scheme"), move |_, _| {
                let value = gsettings_c.string("color-scheme");
                if let Some(settings) = gtk4::Settings::default() {
                    if value == "prefer-dark" {
                        settings.set_gtk_application_prefer_dark_theme(true);
                    } else {
                        settings.set_gtk_application_prefer_dark_theme(false);
                    }
                }
            });

            let gsettings_c2 = gsettings.clone();
            gsettings.connect_changed(Some("icon-theme"), move |_, _| {
                let user_icon_theme = gsettings_c2.string("icon-theme");
                let user_icon_theme = user_icon_theme.trim();
                if let Some(settings) = gtk4::Settings::default() {
                    settings.set_gtk_icon_theme_name(Some(user_icon_theme));
                }
            });
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
    let _ = babydra_common::services::system::set_gsettings_color_scheme(dark);

    if let Some(settings) = gtk4::Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(dark);
    }

    init_theme();
}

pub fn apply_explore_theme() {
    init_theme();
}
