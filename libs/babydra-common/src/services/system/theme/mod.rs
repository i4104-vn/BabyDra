//! System theme service (GTK themes & Cursor themes).

use std::fs;
use std::path::Path;
use std::process::Command;

pub struct ThemeItem {
    pub name: String,
    pub path: String,
}

fn get_home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string())
}

/// Retrieves available GTK themes.
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
                        if (path.join("gtk-3.0").exists() || path.join("gtk-4.0").exists() || path.join("index.theme").exists())
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

/// Retrieves available cursor themes.
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

/// Applies appearance settings via gsettings.
pub fn apply_appearance(gtk_theme: &str, cursor_theme: &str, cursor_size: u32) -> Result<(), String> {
    let size_str = cursor_size.to_string();
    let cmds = vec![
        vec!["gsettings", "set", "org.gnome.desktop.interface", "gtk-theme", gtk_theme],
        vec!["gsettings", "set", "org.gnome.desktop.interface", "cursor-theme", cursor_theme],
        vec!["gsettings", "set", "org.gnome.desktop.interface", "cursor-size", &size_str],
    ];

    for cmd in cmds {
        let _ = Command::new(cmd[0]).args(&cmd[1..]).output();
    }

    Ok(())
}
