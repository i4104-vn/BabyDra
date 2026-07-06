//! Window discovery and focus activation handlers for the Alt-Tab switcher.
//! Uses `wlrctl` to interact with compositor toplevel windows.

use babydra_common::desktop::DesktopApp;
use crate::history::get_history;

/// Queries the Wayland compositor for running windows and matches them with local desktop entries.
/// Returns matched windows sorted by most recently used (MRU) order.
pub fn get_running_apps() -> Vec<DesktopApp> {
    let desktop_apps = babydra_common::desktop::find_desktop_apps();
    let mut running = Vec::new();
    let mut detected_windows = std::collections::HashSet::new();

    let mut running_windows = Vec::new();
    if let Ok(output) = std::process::Command::new("wlrctl").args(&["toplevel", "list"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(pos) = line.find(':') {
                let app_id = line[..pos].trim().to_string();
                let title = line[pos + 1..].trim().to_string();
                if !app_id.is_empty() {
                    running_windows.push((app_id, title));
                }
            }
        }
    }

    for (app_id, title) in running_windows {
        let app_id_lower = app_id.to_lowercase();
        let title_lower = title.to_lowercase();
        let mut matched_app = None;

        let window_key = format!("{}::{}", app_id, title);
        if detected_windows.contains(&window_key) {
            continue;
        }
        detected_windows.insert(window_key);

        for app in &desktop_apps {
            let exec_parts: Vec<&str> = app.exec.split_whitespace().collect();
            if exec_parts.is_empty() {
                continue;
            }
            let exec_path = std::path::Path::new(exec_parts[0]);
            let exec_name = exec_path.file_name()
                .map(|f| f.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            if exec_name == app_id_lower || app.name.to_lowercase() == app_id_lower {
                matched_app = Some(app.clone());
                break;
            }
        }

        if matched_app.is_none() {
            for app in &desktop_apps {
                let exec_parts: Vec<&str> = app.exec.split_whitespace().collect();
                if exec_parts.is_empty() {
                    continue;
                }
                let exec_path = std::path::Path::new(exec_parts[0]);
                let exec_name = exec_path.file_name()
                    .map(|f| f.to_string_lossy().to_lowercase())
                    .unwrap_or_default();

                if app_id_lower.contains(&exec_name) || exec_name.contains(&app_id_lower) || 
                   title_lower.contains(&app.name.to_lowercase()) || app.name.to_lowercase().contains(&app_id_lower) {
                    matched_app = Some(app.clone());
                    break;
                }
            }
        }

        if matched_app.is_none() && (app_id_lower == "navigator" || title_lower.contains("firefox")) {
            for app in &desktop_apps {
                if app.name.to_lowercase().contains("firefox") {
                    matched_app = Some(app.clone());
                    break;
                }
            }
        }

        if let Some(mut app) = matched_app {
            app.app_id = Some(app_id.clone());
            app.window_title = Some(title.clone());
            running.push(app);
        } else {
            let mut chars = app_id.chars();
            let display_name = match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            };
            
            running.push(DesktopApp {
                name: display_name,
                exec: app_id.clone(),
                icon: Some(app_id.clone()),
                is_dependency: false,
                app_id: Some(app_id.clone()),
                window_title: Some(title.clone()),
            });
        }
    }

    let history = get_history();
    running.sort_by(|a, b| {
        let key_a = a.window_title.as_deref().unwrap_or(&a.name);
        let key_b = b.window_title.as_deref().unwrap_or(&b.name);
        let idx_a = history.iter().position(|x| x == key_a).unwrap_or(usize::MAX);
        let idx_b = history.iter().position(|x| x == key_b).unwrap_or(usize::MAX);
        idx_a.cmp(&idx_b)
    });

    running
}

/// Commands the Wayland compositor to focus/activate a specific application window.
/// Utilizes window title first, falling back to app_id and name comparisons.
pub fn activate_app(app: &DesktopApp) {
    if let Some(ref title) = app.window_title {
        let status = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &format!("title:{}", title)])
            .status();
        if let Ok(s) = status {
            if s.success() {
                return;
            }
        }
        
        for delim in &[" — ", " - "] {
            if let Some(pos) = title.rfind(delim) {
                let short_title = title[..pos].trim();
                if !short_title.is_empty() {
                    let status = std::process::Command::new("wlrctl")
                        .args(&["window", "focus", &format!("title:{}", short_title)])
                        .status();
                    if let Ok(s) = status {
                        if s.success() {
                            return;
                        }
                    }
                }
            }
        }
    }

    if let Some(ref app_id) = app.app_id {
        let status = std::process::Command::new("wlrctl")
            .args(&["window", "focus", app_id])
            .status();
        if let Ok(s) = status {
            if s.success() {
                return;
            }
        }
        let status_lower = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &app_id.to_lowercase()])
            .status();
        if let Ok(s) = status_lower {
            if s.success() {
                return;
            }
        }
    }

    let name = &app.name;
    let exec_parts: Vec<&str> = app.exec.split_whitespace().collect();
    let exec_name = if !exec_parts.is_empty() {
        std::path::Path::new(exec_parts[0])
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    if !exec_name.is_empty() {
        let _ = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &exec_name])
            .status();
        let _ = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &format!("title:{}", exec_name)])
            .status();
    }
    
    if !app.exec.is_empty() {
        let _ = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &app.exec])
            .status();
    }

    let _ = std::process::Command::new("wlrctl")
        .args(&["window", "focus", name])
        .status();
    let _ = std::process::Command::new("wlrctl")
        .args(&["window", "focus", &format!("title:{}", name)])
        .status();
}


