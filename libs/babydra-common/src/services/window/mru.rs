//! Alt-Tab application switcher MRU (Most Recently Used) window history.

use crate::services::apps::DesktopApp;
use std::io::Write;

/// Retrieves the switcher window focus history list.
pub fn get_history() -> Vec<String> {
    let history_path = "/tmp/babydra-switcher-history.txt";
    if let Ok(content) = std::fs::read_to_string(history_path) {
        content.lines().map(|s| s.to_string()).collect()
    } else {
        Vec::new()
    }
}

/// Prepends the recently activated window to the MRU history list and saves it back to the temporary file.
pub fn save_history(active_name: &str) {
    let history_path = "/tmp/babydra-switcher-history.txt";
    let mut history = get_history();
    
    history.retain(|x| x != active_name);
    history.insert(0, active_name.to_string());
    history.truncate(20);
    
    if let Ok(mut file) = std::fs::File::create(history_path) {
        for name in history {
            let _ = writeln!(file, "{}", name);
        }
    }
}

/// Queries the Wayland compositor for running windows and matches them with local desktop entries.
/// Returns matched windows sorted by most recently used (MRU) order.
pub fn get_running_apps() -> Vec<DesktopApp> {
    let desktop_apps = crate::services::apps::find_desktop_apps();
    let mut running = Vec::new();
    let mut detected_windows = std::collections::HashSet::new();

    let running_windows = super::get_running_windows();

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
    super::focus_app(
        &app.name,
        &app.exec,
        app.app_id.as_deref(),
        app.window_title.as_deref(),
    );
}
