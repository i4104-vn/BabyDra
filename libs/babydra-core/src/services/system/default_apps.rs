//! System default applications and sound effects configuration.

use std::process::Command;

/// Returns the desktop ID of the current default web browser.
pub fn get_default_browser() -> String {
    if let Ok(output) = Command::new("xdg-settings")
        .args(["get", "default-web-browser"])
        .output()
    {
        let res = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !res.is_empty() {
            return res;
        }
    }

    if let Ok(output) = Command::new("xdg-mime")
        .args(["query", "default", "x-scheme-handler/http"])
        .output()
    {
        return String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    String::new()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppChoice {
    pub desktop_id: String,
    pub name: String,
}


/// Returns list of detected installed browsers.
pub fn get_available_browsers() -> Vec<AppChoice> {
    let mut list = Vec::new();
    let current_default = get_default_browser();

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

        let is_browser = filename.contains("browser")
            || filename.contains("chrome")
            || filename.contains("firefox")
            || filename.contains("opera")
            || filename.contains("brave")
            || filename.contains("zen")
            || filename.contains("vivaldi")
            || filename.contains("chromium")
            || exec.contains("browser")
            || exec.contains("firefox")
            || exec.contains("chrome")
            || exec.contains("opera")
            || name_lower.contains("browser")
            || filename == current_default.to_lowercase();

        if is_browser {
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
                desktop_id: current_default,
                name: pretty_name,
            },
        );
    }

    list
}

/// Returns list of detected installed file managers.
pub fn get_available_file_managers() -> Vec<AppChoice> {
    let mut list = Vec::new();
    let current_default = get_default_file_manager();

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

        let is_fm = filename.contains("explore")
            || filename.contains("file")
            || filename.contains("nautilus")
            || filename.contains("thunar")
            || filename.contains("dolphin")
            || filename.contains("pcmanfm")
            || filename.contains("nemo")
            || filename.contains("caja")
            || exec.contains("explore")
            || exec.contains("thunar")
            || exec.contains("dolphin")
            || name_lower.contains("files")
            || filename == current_default.to_lowercase();

        if is_fm {
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
                desktop_id: current_default,
                name: pretty_name,
            },
        );
    }

    list
}

/// Returns list of detected installed terminals.
pub fn get_available_terminals() -> Vec<AppChoice> {
    let mut list = Vec::new();
    let current_default = get_default_terminal();

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

        let is_term = filename.contains("term")
            || filename.contains("kitty")
            || filename.contains("alacritty")
            || filename.contains("foot")
            || filename.contains("ghostty")
            || filename.contains("konsole")
            || filename.contains("wezterm")
            || exec.contains("kitty")
            || exec.contains("alacritty")
            || exec.contains("foot")
            || name_lower.contains("terminal")
            || filename == current_default.to_lowercase();

        if is_term {
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
                desktop_id: current_default,
                name: pretty_name,
            },
        );
    }

    list
}


/// Sets the system default web browser to the specified desktop ID.
pub fn set_default_browser(desktop_id: &str) -> bool {
    let _ = Command::new("xdg-settings")
        .args(["set", "default-web-browser", desktop_id])
        .status();

    let mime_status = Command::new("xdg-mime")
        .args([
            "default",
            desktop_id,
            "x-scheme-handler/http",
            "x-scheme-handler/https",
            "text/html",
        ])
        .status();

    mime_status.map(|s| s.success()).unwrap_or(false)
}

/// Returns the desktop ID of the current default file manager.
pub fn get_default_file_manager() -> String {
    if let Ok(output) = Command::new("xdg-mime")
        .args(["query", "default", "inode/directory"])
        .output()
    {
        return String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
    String::new()
}

/// Sets the system default file manager to the specified desktop ID.
pub fn set_default_file_manager(desktop_id: &str) -> bool {
    let status = Command::new("xdg-mime")
        .args(["default", desktop_id, "inode/directory"])
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Returns the current default terminal command/desktop ID.
pub fn get_default_terminal() -> String {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.default-applications.terminal", "exec"])
        .output()
    {
        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let cleaned = raw.trim_matches('\'').trim_matches('"');
        if !cleaned.is_empty() {
            return format!("{}.desktop", cleaned);
        }
    }
    "kitty.desktop".to_string()
}

/// Sets the system default terminal.
pub fn set_default_terminal(desktop_id: &str) -> bool {
    let bin_name = desktop_id.trim_end_matches(".desktop");
    let status = Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.default-applications.terminal",
            "exec",
            bin_name,
        ])
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Returns whether system event sounds are enabled.
pub fn get_event_sounds_enabled() -> bool {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.sound", "event-sounds"])
        .output()
    {
        return String::from_utf8_lossy(&output.stdout).trim() == "true";
    }
    true
}

/// Sets whether system event sounds are enabled.
pub fn set_event_sounds_enabled(enabled: bool) {
    let val = if enabled { "true" } else { "false" };
    let _ = Command::new("gsettings")
        .args(["set", "org.gnome.desktop.sound", "event-sounds", val])
        .status();
}

/// Returns whether input feedback sounds are enabled.
pub fn get_input_feedback_sounds_enabled() -> bool {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.sound", "input-feedback-sounds"])
        .output()
    {
        return String::from_utf8_lossy(&output.stdout).trim() == "true";
    }
    false
}

/// Sets whether input feedback sounds are enabled.
pub fn set_input_feedback_sounds_enabled(enabled: bool) {
    let val = if enabled { "true" } else { "false" };
    let _ = Command::new("gsettings")
        .args(["set", "org.gnome.desktop.sound", "input-feedback-sounds", val])
        .status();
}

/// Plays a system alert sound for testing audio output.
pub fn play_test_alert_sound() {
    let sound_paths = [
        "/usr/share/sounds/freedesktop/stereo/bell.oga",
        "/usr/share/sounds/freedesktop/stereo/audio-volume-change.oga",
        "/usr/share/sounds/freedesktop/stereo/message.oga",
    ];

    let found_path = sound_paths.iter().find(|p| std::path::Path::new(p).exists());

    if let Some(path) = found_path {
        // Try pw-play first, then paplay
        if Command::new("pw-play").arg(path).spawn().is_err() {
            let _ = Command::new("paplay").arg(path).spawn();
        }
    }
}
