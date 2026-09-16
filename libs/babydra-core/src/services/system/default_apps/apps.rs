//! System default web browser, terminal, and file manager detection and settings.

use crate::models::desktop::app::AppChoice;
use crate::services::utils::{gsettings, xdg};

/// Returns the desktop ID of the current default web browser.
pub fn get_default_browser() -> String {
    xdg::get_setting("default-web-browser")
        .or_else(|| xdg::query_default("x-scheme-handler/http"))
        .unwrap_or_default()
}

/// Returns list of detected installed browsers matching standard XDG WebBrowser category and web schemes.
pub fn get_available_browsers() -> Vec<AppChoice> {
    xdg::find_apps_by_category_or_mime(
        &["WebBrowser"],
        &[
            "x-scheme-handler/http",
            "x-scheme-handler/https",
            "text/html",
        ],
        &get_default_browser(),
    )
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

/// Returns list of detected installed file managers matching standard XDG FileManager category and directory MIME.
pub fn get_available_file_managers() -> Vec<AppChoice> {
    xdg::find_apps_by_category_or_mime(
        &["FileManager"],
        &["inode/directory"],
        &get_default_file_manager(),
    )
}

/// Sets the system default file manager to the specified desktop ID.
pub fn set_default_file_manager(desktop_id: &str) -> bool {
    xdg::set_default(desktop_id, &["inode/directory"])
}

/// Returns the current default terminal command/desktop ID.
pub fn get_default_terminal() -> String {
    if let Some(cleaned) = gsettings::get("org.gnome.desktop.default-applications.terminal", "exec")
    {
        if !cleaned.is_empty() {
            return format!("{}.desktop", cleaned);
        }
    }
    let terminals = xdg::find_apps_by_category_or_mime(&["TerminalEmulator"], &[], "");
    terminals
        .first()
        .map(|t| t.desktop_id.clone())
        .unwrap_or_default()
}

/// Returns list of detected installed terminals matching standard XDG TerminalEmulator category.
pub fn get_available_terminals() -> Vec<AppChoice> {
    xdg::find_apps_by_category_or_mime(&["TerminalEmulator"], &[], &get_default_terminal())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_default_apps_query_no_panic() {
        let browsers = get_available_browsers();
        let file_managers = get_available_file_managers();
        let terminals = get_available_terminals();

        // Must run cleanly and return choices based on system state
        for b in &browsers {
            assert!(!b.desktop_id.is_empty());
        }
        for fm in &file_managers {
            assert!(!fm.desktop_id.is_empty());
        }
        for t in &terminals {
            assert!(!t.desktop_id.is_empty());
        }
    }
}
