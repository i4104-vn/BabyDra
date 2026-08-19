//! Filesystem scanning and parsing of `.desktop` files.

use super::pacman::{get_explicit_pkgs, get_package_owner, is_dep_heuristic};
use super::DesktopApp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Scans for `desktop apps from filesystem`.
pub fn scan_desktop_apps() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let paths = vec![
        PathBuf::from("/usr/share/applications"),
        dirs::data_dir()
            .unwrap_or_else(|| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".local/share")
            })
            .join("applications"),
    ];

    let explicit_packages = get_explicit_pkgs();

    for path in paths {
        if !path.exists() {
            continue;
        }
        let is_system_dir = path.to_string_lossy().contains("/usr/share");
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path
                    .extension()
                    .map(|e| e == "desktop")
                    .unwrap_or(false)
                {
                    if let Some(mut app) = parse_desktop_file(&entry_path) {
                        let filename = entry_path
                            .file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or("");
                        let mut is_dep = false;
                        if is_system_dir {
                            let mut pacman_success = false;
                            if !explicit_packages.is_empty() {
                                if let Some(owner) = get_package_owner(&entry_path) {
                                    pacman_success = true;
                                    is_dep = !explicit_packages.contains(&owner);
                                }
                            }
                            if !pacman_success {
                                is_dep = is_dep_heuristic(filename, &app.name, &app.exec);
                            }
                        }
                        app.is_dependency = is_dep;
                        apps.push(app);
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps.dedup_by(|a, b| a.name.to_lowercase() == b.name.to_lowercase());

    apps
}

/// Parses `desktop file`.
pub fn parse_desktop_file(path: &Path) -> Option<DesktopApp> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut no_display = false;
    let mut in_desktop_entry = false;

    for line in reader.lines().flatten() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            if line == "[Desktop Entry]" {
                in_desktop_entry = true;
            } else {
                in_desktop_entry = false;
            }
            continue;
        }

        if !in_desktop_entry {
            continue;
        }

        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim();
            let value = line[pos + 1..].trim();

            match key {
                "Name" if name.is_none() => name = Some(value.to_string()),
                "Exec" if exec.is_none() => {
                    let clean_exec = value
                        .split_whitespace()
                        .filter(|word| !word.starts_with('%'))
                        .collect::<Vec<&str>>()
                        .join(" ");
                    exec = Some(clean_exec);
                }
                "Icon" if icon.is_none() => icon = Some(value.to_string()),
                "NoDisplay" => {
                    if value.to_lowercase() == "true" {
                        no_display = true;
                    }
                }
                _ => {}
            }
        }
    }

    if no_display {
        return None;
    }

    match (name, exec) {
        (Some(n), Some(e)) => Some(DesktopApp {
            name: n,
            exec: e,
            icon,
            file_path: Some(path.to_path_buf()),
            is_dependency: false,
            app_id: None,
            window_title: None,
        }),
        _ => None,
    }
}
