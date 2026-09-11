//! System default web browser, terminal, and file manager detection and settings.

use crate::models::desktop::app::AppChoice;
use crate::services::utils::{gsettings, xdg};

/// Returns the desktop ID of the current default web browser.
pub fn get_default_browser() -> String {
    xdg::get_setting("default-web-browser")
        .or_else(|| xdg::query_default("x-scheme-handler/http"))
        .unwrap_or_default()
}

/// Returns list of detected installed browsers.
pub fn get_available_browsers() -> Vec<AppChoice> {
    const KEYWORDS: &[&str] = &[
        "browser", "chrome", "firefox", "opera", "brave", "zen", "vivaldi", "chromium",
    ];
    xdg::find_matching_apps(KEYWORDS, &get_default_browser())
}

/// Sets the system default web browser to the specified desktop ID.
pub fn set_default_browser(desktop_id: &str) -> bool {
    let _ = xdg::set_setting("default-web-browser", desktop_id);
    xdg::set_default(
        desktop_id,
        &[
            "x-scheme-handler/http",
            "x-scheme-handler/https",
            "text/html",
        ],
    )
}

/// Returns the desktop ID of the current default file manager.
pub fn get_default_file_manager() -> String {
    xdg::query_default("inode/directory").unwrap_or_default()
}

/// Returns list of detected installed file managers.
pub fn get_available_file_managers() -> Vec<AppChoice> {
    const KEYWORDS: &[&str] = &[
        "explore", "file", "nautilus", "thunar", "dolphin", "pcmanfm", "nemo", "caja",
    ];
    xdg::find_matching_apps(KEYWORDS, &get_default_file_manager())
}

/// Sets the system default file manager to the specified desktop ID.
pub fn set_default_file_manager(desktop_id: &str) -> bool {
    xdg::set_default(desktop_id, &["inode/directory"])
}

/// Returns the current default terminal command/desktop ID.
pub fn get_default_terminal() -> String {
    if let Some(cleaned) = gsettings::get("org.gnome.desktop.default-applications.terminal", "exec") {
        if !cleaned.is_empty() {
            return format!("{}.desktop", cleaned);
        }
    }
    "kitty.desktop".to_string()
}

/// Returns list of detected installed terminals.
pub fn get_available_terminals() -> Vec<AppChoice> {
    const KEYWORDS: &[&str] = &[
        "term", "kitty", "alacritty", "foot", "ghostty", "konsole", "wezterm",
    ];
    xdg::find_matching_apps(KEYWORDS, &get_default_terminal())
}

/// Sets the system default terminal.
pub fn set_default_terminal(desktop_id: &str) -> bool {
    let bin_name = desktop_id.trim_end_matches(".desktop");
    gsettings::set(
        "org.gnome.desktop.default-applications.terminal",
        "exec",
        bin_name,
    )
}
