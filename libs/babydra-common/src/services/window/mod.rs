//! Window layout builders, layer shell configuration, and compositor window helpers.

pub mod tracker;
pub mod mru;
/// Queries the compositor via `wlrctl` for all running window instances.
/// Returns a list of (app_id, window_title) pairs.
pub fn get_running_windows() -> Vec<(String, String)> {
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
    running_windows
}

/// Queries the compositor via `wlrctl` for the currently focused window instance.
/// Returns Some((app_id, window_title)) or None.
pub fn get_active_window() -> Option<(String, String)> {
    let output = std::process::Command::new("wlrctl")
        .args(&["window", "list", "state:focused"])
        .output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if let Some(line) = stdout.lines().next() {
            if let Some(pos) = line.find(':') {
                let app_id = line[..pos].trim().to_string();
                let title = line[pos + 1..].trim().to_string();
                if !app_id.is_empty() {
                    return Some((app_id, title));
                }
            }
        }
    }
    None
}

fn find_best_window_match(
    windows: &[(String, String)],
    name: &str,
    exec: &str,
    app_id: Option<&str>,
    window_title: Option<&str>,
) -> Option<String> {
    if let Some(title) = window_title {
        for (_, w_title) in windows {
            if w_title == title {
                return Some(format!("title:{}", title));
            }
        }
        for delim in &[" — ", " - "] {
            if let Some(pos) = title.rfind(delim) {
                let short_title = title[..pos].trim();
                if !short_title.is_empty() {
                    for (_, w_title) in windows {
                        if w_title == short_title {
                            return Some(format!("title:{}", short_title));
                        }
                    }
                }
            }
        }
    }

    if let Some(id) = app_id {
        for (w_id, _) in windows {
            if w_id == id || w_id.to_lowercase() == id.to_lowercase() {
                return Some(w_id.clone());
            }
        }
    }

    let exec_parts: Vec<&str> = exec.split_whitespace().collect();
    let exec_name = if !exec_parts.is_empty() {
        std::path::Path::new(exec_parts[0])
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    if !exec_name.is_empty() {
        for (w_id, w_title) in windows {
            if w_id == &exec_name || w_title == &exec_name {
                return Some(w_id.clone());
            }
        }
    }

    if !exec.is_empty() {
        for (w_id, _) in windows {
            if w_id == exec {
                return Some(w_id.clone());
            }
        }
    }

    for (w_id, w_title) in windows {
        if w_id == name || w_title == name {
            return Some(w_id.clone());
        }
    }

    None
}

/// Activates/Focuses an application window using wlrctl via custom rules.
pub fn focus_app(name: &str, exec: &str, app_id: Option<&str>, window_title: Option<&str>) {
    let running = get_running_windows();
    
    if let Some(identifier) = find_best_window_match(&running, name, exec, app_id, window_title) {
        let _ = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &identifier])
            .status();
    } else if !exec.is_empty() {
        let exec_parts: Vec<&str> = exec.split_whitespace().collect();
        if let Some(cmd) = exec_parts.first() {
            let _ = std::process::Command::new(cmd).spawn();
        }
    }
}

/// Closes a window using wlrctl.
pub fn close_window(app_id: &str, title: &str) {
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

/// Focuses a window using wlrctl.
pub fn focus_window(app_id: &str, title: &str) {
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
