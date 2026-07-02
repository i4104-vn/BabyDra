use babydra_common::desktop::{DesktopApp, find_desktop_apps};

/// Lists all running windows matched against desktop entries.
pub fn get_running_windows() -> Vec<DesktopApp> {
    let desktop_apps = find_desktop_apps();
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
                app_id: Some(app_id.clone()),
                window_title: Some(title.clone()),
            });
        }
    }

    running
}

/// Focuses a window using wlrctl.
pub fn focus_window(app_id: &str, title: &str) {
    println!("Focusing window: {} - {}", app_id, title);
    let status = std::process::Command::new("wlrctl")
        .args(&["window", "focus", &format!("title:{}", title)])
        .status();
    if let Ok(s) = status {
        if s.success() {
            return;
        }
    }
    let _ = std::process::Command::new("wlrctl")
        .args(&["window", "focus", app_id])
        .status();
}

/// Closes a window using wlrctl.
pub fn close_window(app_id: &str, title: &str) {
    println!("Closing window: {} - {}", app_id, title);
    let status = std::process::Command::new("wlrctl")
        .args(&["window", "close", &format!("title:{}", title)])
        .status();
    if let Ok(s) = status {
        if s.success() {
            return;
        }
    }
    let _ = std::process::Command::new("wlrctl")
        .args(&["window", "close", app_id])
        .status();
}
