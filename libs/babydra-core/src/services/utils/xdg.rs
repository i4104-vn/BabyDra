use crate::models::desktop::app::AppChoice;
use std::path::Path;
use std::process::Command;

/// Queries the default application desktop ID registered for a MIME type via `xdg-mime`.
pub fn query_default(mime_type: &str) -> Option<String> {
    Command::new("xdg-mime")
        .args(["query", "default", mime_type])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Registers an application desktop ID as the default handler for one or more MIME types via `xdg-mime`.
pub fn set_default(desktop_id: &str, mime_types: &[&str]) -> bool {
    if mime_types.is_empty() {
        return false;
    }
    let mut args = vec!["default", desktop_id];
    args.extend_from_slice(mime_types);

    Command::new("xdg-mime")
        .args(&args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Retrieves an XDG setting via `xdg-settings` (e.g. "default-web-browser").
pub fn get_setting(property: &str) -> Option<String> {
    Command::new("xdg-settings")
        .args(["get", property])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Sets an XDG setting via `xdg-settings`.
pub fn set_setting(property: &str, value: &str) -> bool {
    Command::new("xdg-settings")
        .args(["set", property, value])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Opens a file or URL with the platform default handler via `xdg-open`.
pub fn open_path(path: impl AsRef<Path>) -> bool {
    Command::new("xdg-open")
        .arg(path.as_ref())
        .spawn()
        .is_ok()
}

/// Generic helper to search installed desktop applications matching category keywords.
/// Searches system and user desktop entries, matches against keywords, and formats `AppChoice`.
pub fn find_matching_apps(keywords: &[&str], current_default: &str) -> Vec<AppChoice> {
    let mut list = Vec::new();
    let all_apps = crate::services::apps::find_desktop_apps();

    for app in &all_apps {
        let filename = app
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        let exec = app.exec.to_lowercase();
        let name_lower = app.name.to_lowercase();

        let is_match = keywords.iter().any(|k| {
            filename.contains(k) || exec.contains(k) || name_lower.contains(k)
        }) || (!current_default.is_empty() && filename == current_default.to_lowercase());

        if is_match {
            let desktop_id = app
                .file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or(&app.name)
                .to_string();

            if !list.iter().any(|c: &AppChoice| c.desktop_id == desktop_id) {
                list.push(AppChoice {
                    desktop_id,
                    name: app.name.clone(),
                });
            }
        }
    }

    if !current_default.is_empty() && !list.iter().any(|c| c.desktop_id == current_default) {
        let pretty_name = current_default
            .trim_end_matches(".desktop")
            .replace('-', " ");
        list.insert(
            0,
            AppChoice {
                desktop_id: current_default.to_string(),
                name: pretty_name,
            },
        );
    }

    list
}
