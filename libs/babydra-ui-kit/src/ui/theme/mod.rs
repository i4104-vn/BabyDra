//! Theme and styling system coordinator.
//!
//! Loads the shared structural stylesheet (`styles/shared/**`) embedded in
//! this crate, then resolves the active theme package (`themes/<id>`) through
//! the `babydra-theme` engine and concatenates its color layers on top.
//!
//! Theme selection comes from `~/.babydra/babydra.conf` (`theme.selection`);
//! an empty `id` selects the engine default theme (`babydra-default`).

pub mod colors;

/// Shared structural stylesheet — layout, spacing, component structure.
/// Color values live in theme packages (`themes/<id>/`), not here.
const SHARED_CSS: &str = concat!(
    include_str!("../../styles/shared/panel/panel.css"),
    "\n",
    include_str!("../../styles/shared/panel/workspaces.css"),
    "\n",
    include_str!("../../styles/shared/panel/clock.css"),
    "\n",
    include_str!("../../styles/shared/panel/status.css"),
    "\n",
    include_str!("../../styles/shared/panel/system_monitor.css"),
    "\n",
    include_str!("../../styles/shared/panel/tray.css"),
    "\n",
    include_str!("../../styles/shared/panel/taskbar.css"),
    "\n",
    include_str!("../../styles/shared/control_center/control_center.css"),
    "\n",
    include_str!("../../styles/shared/control_center/power.css"),
    "\n",
    include_str!("../../styles/shared/island/system_island.css"),
    "\n",
    include_str!("../../styles/shared/island/notification.css"),
    "\n",
    include_str!("../../styles/shared/launcher/launcher.css"),
    "\n",
    include_str!("../../styles/shared/calendar/calendar.css"),
    "\n",
    include_str!("../../styles/shared/shared/button.css"),
    "\n",
    include_str!("../../styles/shared/shared/sidebar.css"),
    "\n",
    include_str!("../../styles/shared/apps/screenshot.css"),
    "\n",
    include_str!("../../styles/shared/apps/lock.css"),
    "\n",
    include_str!("../../styles/shared/apps/preview.css"),
    "\n",
    include_str!("../../styles/shared/apps/settings.css"),
    "\n",
    include_str!("../../styles/shared/apps/switcher.css"),
    "\n",
    include_str!("../../styles/shared/apps/desktop.css"),
    "\n",
    include_str!("../../styles/shared/explore/window.css"),
    "\n",
    include_str!("../../styles/shared/explore/header_bar.css"),
    "\n",
    include_str!("../../styles/shared/explore/content_view.css"),
    "\n",
    include_str!("../../styles/shared/explore/info_panel.css"),
    "\n",
    include_str!("../../styles/shared/explore/status_bar.css"),
    "\n",
    include_str!("../../styles/shared/explore/context_menu.css"),
    "\n",
    include_str!("../../styles/shared/explore/dialogs.css"),
    "\n",
    include_str!("../../styles/shared/shared/scrollbar.css")
);

thread_local! {
    static CSS_PROVIDER: gtk4::CssProvider = gtk4::CssProvider::new();
}

use gio::prelude::*;

/// Resolves the color layers for the active theme + mode.
///
/// Returns `(dark_css, light_css, extra_layer)` — the caller picks the
/// dark/light side based on the current mode and appends the extra layer.
fn resolve_theme_layers() -> (String, String, String) {
    let selection = babydra_core::config::load_babydra_config().theme.selection;
    let id = if selection.id.is_empty() {
        "babydra-default".to_string()
    } else {
        selection.id.clone()
    };

    let theme = match babydra_theme::resolve_theme(&id) {
        Ok(t) => t,
        Err(err) => {
            tracing::warn!(
                target: "babydra-ui-kit",
                "theme '{id}' failed to resolve ({err}); falling back to babydra-default"
            );
            match babydra_theme::resolve_theme("babydra-default") {
                Ok(t) => t,
                Err(err2) => {
                    tracing::error!(
                        target: "babydra-ui-kit",
                        "no theme package available ({err2}); loading structural CSS only"
                    );
                    return (String::new(), String::new(), String::new());
                }
            }
        }
    };

    (theme.dark_css, theme.light_css, theme.css_layer)
}

/// Builds the full CSS string for the current mode.
///
/// Dark mode preference precedence: `theme.selection.dark` from config wins
/// when set (`Some`), otherwise falls back to the GSettings system scheme.
fn build_css() -> String {
    let selection = babydra_core::config::load_babydra_config().theme.selection;
    let is_dark = selection.dark.unwrap_or_else(is_dark_mode);
    let (dark_css, light_css, extra_layer) = resolve_theme_layers();
    let color_layer = if is_dark { dark_css } else { light_css };
    let css = format!("{SHARED_CSS}\n{color_layer}\n{extra_layer}");
    css.replace("\r", "")
}

/// Initializes the GtkCssProvider, registers it with the GdkDisplay,
/// and loads the shared structural CSS + the active theme package layers.
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
            let css = build_css();
            provider.load_from_data(&css);

            let provider_clone = provider.clone();
            settings.connect_gtk_application_prefer_dark_theme_notify(move |_s| {
                let css = build_css();
                provider_clone.load_from_data(&css);
            });
        } else {
            let css = build_css();
            provider.load_from_data(&css);
        }
    });
}

/// Helper stub for backward compatibility.
pub fn apply_theme_class(_window: &gtk4::ApplicationWindow) {
    // stub — theme is applied globally via init_theme()
}

/// Checks if dark mode is preferred in GSettings.
pub fn is_dark_mode() -> bool {
    gtk4::Settings::default()
        .map(|s| s.is_gtk_application_prefer_dark_theme())
        .unwrap_or(true)
}

/// Sets the color scheme preference in GSettings.
pub fn set_dark_mode(dark: bool) {
    std::thread::spawn(move || {
        let _ = babydra_core::services::system::set_color_scheme(dark);

        gtk4::glib::idle_add_local_once(move || {
            if let Some(settings) = gtk4::Settings::default() {
                settings.set_gtk_application_prefer_dark_theme(dark);
            }
            init_theme();
        });
    });
}
