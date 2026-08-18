pub use crate::models::shell::appearance::CurrentAppearance;
use crate::services::utils::{get_home_dir, run_cmd};
use std::fs;
use std::path::Path;

/// Returns the current `gtk themes`.
pub fn get_gtk_themes() -> Vec<String> {
    let mut names = Vec::new();
    let home = get_home_dir();
    let dirs = [
        Path::new("/usr/share/themes").to_path_buf(),
        Path::new(&home).join(".themes"),
        Path::new(&home).join(".local/share/themes"),
    ];

    for d in &dirs {
        if d.exists() {
            if let Ok(entries) = fs::read_dir(d) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if (path.join("gtk-3.0").exists()
                            || path.join("gtk-4.0").exists()
                            || path.join("index.theme").exists())
                            && !names.contains(&name)
                        {
                            names.push(name);
                        }
                    }
                }
            }
        }
    }

    names.sort();
    if names.is_empty() {
        names.push("Adwaita".to_string());
    }
    names
}

/// Returns the current `cursor themes`.
pub fn get_cursor_themes() -> Vec<String> {
    let mut names = Vec::new();
    let home = get_home_dir();
    let dirs = [
        Path::new("/usr/share/icons").to_path_buf(),
        Path::new(&home).join(".icons"),
        Path::new(&home).join(".local/share/icons"),
    ];

    for d in &dirs {
        if d.exists() {
            if let Ok(entries) = fs::read_dir(d) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if path.join("cursors").exists() && !names.contains(&name) {
                            names.push(name);
                        }
                    }
                }
            }
        }
    }

    names.sort();
    if names.is_empty() {
        names.push("Adwaita".to_string());
    }
    names
}

/// Returns the current `icon themes`.
pub fn get_icon_themes() -> Vec<String> {
    let mut names = Vec::new();
    let home = get_home_dir();
    let dirs = [
        Path::new("/usr/share/icons").to_path_buf(),
        Path::new(&home).join(".icons"),
        Path::new(&home).join(".local/share/icons"),
    ];

    for d in &dirs {
        if d.exists() {
            if let Ok(entries) = fs::read_dir(d) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if path.join("index.theme").exists() && !names.contains(&name) {
                            names.push(name);
                        }
                    }
                }
            }
        }
    }

    names.sort();
    if names.is_empty() {
        names.push("Adwaita".to_string());
    }
    names
}

/// Returns the current `current appearance`.
pub fn get_appearance() -> CurrentAppearance {
    let get_val = |key: &str| -> String {
        if let Some(s) = run_cmd(&["gsettings", "get", "org.gnome.desktop.interface", key]) {
            if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 {
                s[1..s.len() - 1].to_string()
            } else {
                s
            }
        } else {
            String::new()
        }
    };

    let gtk = get_val("gtk-theme");
    let icon = get_val("icon-theme");
    let cursor = get_val("cursor-theme");
    let size_str = get_val("cursor-size");
    let size = size_str.parse::<u32>().unwrap_or(24);

    CurrentAppearance {
        gtk_theme: if gtk.is_empty() {
            "Adwaita".to_string()
        } else {
            gtk
        },
        icon_theme: if icon.is_empty() {
            "Adwaita".to_string()
        } else {
            icon
        },
        cursor_theme: if cursor.is_empty() {
            "Adwaita".to_string()
        } else {
            cursor
        },
        cursor_size: size,
    }
}
