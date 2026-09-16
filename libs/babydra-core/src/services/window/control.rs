//! Window focus, query, minimize, and close controls via wlrctl.

use std::path::Path;
use std::process::Command;

/// Queries the compositor via `wlrctl` for all running window instances.
/// Returns a list of (app_id, window_title) pairs.
pub fn get_running_windows() -> Vec<(String, String)> {
    let mut running_windows = Vec::new();
    if let Ok(output) = Command::new("wlrctl").args(&["toplevel", "list"]).output() {
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
    let output = Command::new("wlrctl")
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

pub fn find_best_window_match(
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
        Path::new(exec_parts[0])
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
        let _ = Command::new("wlrctl")
            .args(&["window", "focus", &identifier])
            .status();
    } else if !exec.is_empty() {
        let exec_parts: Vec<&str> = exec.split_whitespace().collect();
        if let Some(cmd) = exec_parts.first() {
            let _ = Command::new(cmd).spawn();
        }
    }
}

/// Activates and focuses an application by its name or window title,
/// switching to its workspace if already running, or launching it if not.
pub fn jump_to_app(app_name: &str, title: Option<&str>) {
    let query = app_name.trim();
    if query.is_empty() {
        return;
    }

    let running_apps = super::mru::get_running_apps();
    let current_ws = crate::services::workspace::get_current_workspace();
    let ws_map =
        crate::services::workspace::windows::sync_workspace_apps(current_ws, &running_apps);

    let query_lower = query.to_lowercase();
    let title_lower = title.unwrap_or("").trim().to_lowercase();

    if let Some(app) = running_apps.iter().find(|app| {
        app.name.to_lowercase().contains(&query_lower)
            || app
                .app_id
                .as_deref()
                .map_or(false, |id| id.to_lowercase().contains(&query_lower))
            || (!title_lower.is_empty()
                && app
                    .window_title
                    .as_deref()
                    .map_or(false, |t| t.to_lowercase().contains(&title_lower)))
    }) {
        let ws = crate::services::workspace::windows::get_app_workspace(app, &ws_map, current_ws);
        crate::services::workspace::switch_workspace(ws);
        super::mru::activate_app(app);
    } else if let Some(app) = crate::services::apps::find_desktop_apps()
        .iter()
        .find(|a| a.name.to_lowercase().contains(&query_lower))
    {
        focus_app(&app.name, &app.exec, app.app_id.as_deref(), None);
    }
}

/// Closes a single window instance using wlrctl safely.
pub fn close_window(app_id: &str, title: &str) {
    let running = get_running_windows();

    let actual_title = if !title.is_empty() {
        if running
            .iter()
            .any(|(id, t)| (id == app_id || id.is_empty()) && t == title)
        {
            title.to_string()
        } else {
            running
                .iter()
                .find(|(id, t)| {
                    (id == app_id || id.is_empty() || app_id.is_empty())
                        && (t.contains(title) || title.contains(t.as_str()))
                })
                .map(|(_, t)| t.clone())
                .unwrap_or_else(|| title.to_string())
        }
    } else {
        String::new()
    };

    let target_title = if !actual_title.is_empty() {
        &actual_title
    } else {
        title
    };

    let same_title_count = if !target_title.is_empty() {
        running.iter().filter(|(_, t)| t == target_title).count()
    } else {
        running.iter().filter(|(id, _)| id == app_id).count()
    };

    if same_title_count > 1 && !target_title.is_empty() {
        let _ = Command::new("wlrctl")
            .args(&["window", "focus", &format!("title:{}", target_title)])
            .status();

        let mut confirmed_active = false;
        for _ in 0..12 {
            std::thread::sleep(std::time::Duration::from_millis(25));
            let is_active = Command::new("wlrctl")
                .args(&[
                    "window",
                    "find",
                    &format!("title:{}", target_title),
                    "state:active",
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if is_active {
                confirmed_active = true;
                break;
            }
        }

        if confirmed_active {
            let _ = Command::new("wlrctl")
                .args(&[
                    "window",
                    "close",
                    &format!("title:{}", target_title),
                    "state:active",
                ])
                .status();
        }
        return;
    }

    if !target_title.is_empty() {
        let _ = Command::new("wlrctl")
            .args(&["window", "close", &format!("title:{}", target_title)])
            .status();
    }
}

/// Closes all windows matching an application ID.
pub fn close_all_windows(app_id: &str) {
    let _ = Command::new("wlrctl")
        .args(&["window", "close", app_id])
        .status();
}

/// Closes a window without blocking the UI caller on compositor I/O.
pub fn close_window_async(app_id: &str, title: &str) {
    let app_id = app_id.to_owned();
    let title = title.to_owned();
    std::thread::spawn(move || close_window(&app_id, &title));
}

/// Closes all windows for an application without blocking the UI caller.
pub fn close_all_windows_async(app_id: &str) {
    let app_id = app_id.to_owned();
    std::thread::spawn(move || close_all_windows(&app_id));
}

/// Minimizes a specific window instance using wlrctl.
pub fn minimize_window(app_id: &str, title: &str) {
    let running = get_running_windows();
    let app_id_clean = app_id.strip_suffix(".desktop").unwrap_or(app_id);

    if !title.is_empty() {
        let clean_title = title.trim_end_matches('●').trim();
        if let Some((_, exact_title)) = running.iter().find(|(id, t)| {
            (id.eq_ignore_ascii_case(app_id_clean) || id.is_empty() || app_id_clean.is_empty())
                && t.trim_end_matches('●').trim() == clean_title
        }) {
            if let Ok(s) = Command::new("wlrctl")
                .args(&["toplevel", "minimize", &format!("title:{}", exact_title)])
                .status()
            {
                if s.success() {
                    return;
                }
            }
        }
    }

    if let Some((exact_id, _)) = running
        .iter()
        .find(|(id, _)| id.eq_ignore_ascii_case(app_id_clean))
    {
        if let Ok(s) = Command::new("wlrctl")
            .args(&["toplevel", "minimize", exact_id])
            .status()
        {
            if s.success() {
                return;
            }
        }
    }

    let _ = Command::new("wlrctl")
        .args(&["toplevel", "minimize", app_id_clean])
        .status();
}

/// Focuses a window using wlrctl with case-insensitive app_id and title matching.
pub fn focus_window(app_id: &str, title: &str) {
    let running = get_running_windows();
    let app_id_clean = app_id.strip_suffix(".desktop").unwrap_or(app_id);

    if !title.is_empty() {
        let clean_title = title.trim_end_matches('●').trim();

        // 1. Exact match on title (ignoring unsaved markers)
        if let Some((_, exact_title)) = running.iter().find(|(id, t)| {
            (id.eq_ignore_ascii_case(app_id_clean) || id.is_empty() || app_id_clean.is_empty())
                && t.trim_end_matches('●').trim() == clean_title
        }) {
            if let Ok(s) = Command::new("wlrctl")
                .args(&["toplevel", "focus", &format!("title:{}", exact_title)])
                .status()
            {
                if s.success() {
                    return;
                }
            }
        }

        // 2. Substring match on title
        if let Some((_, exact_title)) = running.iter().find(|(id, t)| {
            (id.eq_ignore_ascii_case(app_id_clean) || id.is_empty() || app_id_clean.is_empty())
                && (t.contains(clean_title) || clean_title.contains(t.as_str()))
        }) {
            if let Ok(s) = Command::new("wlrctl")
                .args(&["toplevel", "focus", &format!("title:{}", exact_title)])
                .status()
            {
                if s.success() {
                    return;
                }
            }
        }
    }

    // 3. Match running app_id case-insensitively
    if let Some((exact_id, _)) = running
        .iter()
        .find(|(id, _)| id.eq_ignore_ascii_case(app_id_clean))
    {
        if let Ok(s) = Command::new("wlrctl")
            .args(&["toplevel", "focus", exact_id])
            .status()
        {
            if s.success() {
                return;
            }
        }
    }

    // 4. Direct fallback
    let _ = Command::new("wlrctl")
        .args(&["toplevel", "focus", app_id_clean])
        .status();
}

/// Focuses a window without making the GTK event callback wait for `wlrctl`.
pub fn focus_window_async(app_id: &str, title: &str) {
    let app_id = app_id.to_owned();
    let title = title.to_owned();
    std::thread::spawn(move || focus_window(&app_id, &title));
}

/// Toggles an application window: minimizes it if currently active, or focuses it if inactive.
pub fn toggle_app_window(app_id: &str, title: &str) {
    let active = get_active_window();
    let app_id_clean = app_id.strip_suffix(".desktop").unwrap_or(app_id);

    let is_currently_active = if let Some((ref active_id, ref active_title)) = active {
        let id_matches = active_id.eq_ignore_ascii_case(app_id_clean);
        if !title.is_empty() {
            let clean_act = active_title.trim_end_matches('●').trim();
            let clean_t = title.trim_end_matches('●').trim();
            id_matches && (clean_act == clean_t || clean_act.contains(clean_t) || clean_t.contains(clean_act))
        } else {
            id_matches
        }
    } else {
        false
    };

    if is_currently_active {
        minimize_window(app_id_clean, title);
    } else {
        focus_window(app_id_clean, title);
    }
}

/// Toggles a window asynchronously for responsive taskbar clicks.
pub fn toggle_app_window_async(app_id: &str, title: &str) {
    let app_id = app_id.to_owned();
    let title = title.to_owned();
    std::thread::spawn(move || toggle_app_window(&app_id, &title));
}

/// Switches workspace and focuses the target in one ordered background task.
pub fn focus_window_on_workspace_async(workspace_id: u32, app_id: &str, title: &str) {
    let app_id = app_id.to_owned();
    let title = title.to_owned();
    std::thread::spawn(move || {
        crate::services::workspace::switch_workspace(workspace_id);
        focus_window(&app_id, &title);
    });
}

/// Minimizes all open application windows to show the desktop.
pub fn minimize_all_windows() {
    let running = get_running_windows();
    for (app_id, title) in running {
        if !title.is_empty() {
            let _ = Command::new("wlrctl")
                .args(&["toplevel", "minimize", &format!("title:{}", title)])
                .status();
        } else if !app_id.is_empty() {
            let _ = Command::new("wlrctl")
                .args(&["toplevel", "minimize", &app_id])
                .status();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_best_window_match_by_title() {
        let windows = vec![
            ("Opera".to_string(), "YouTube - Opera".to_string()),
            ("antigravity-ide".to_string(), "BabyDra - IDE".to_string()),
        ];

        let res = find_best_window_match(&windows, "Opera", "opera", Some("Opera"), Some("YouTube - Opera"));
        assert_eq!(res, Some("title:YouTube - Opera".to_string()));
    }

    #[test]
    fn test_find_best_window_match_case_insensitive_app_id() {
        let windows = vec![
            ("Opera".to_string(), "Opera Browser".to_string()),
        ];

        let res = find_best_window_match(&windows, "opera", "opera", Some("opera"), None);
        assert_eq!(res, Some("Opera".to_string()));
    }

    #[test]
    fn test_app_id_clean_suffix() {
        let id = "antigravity-ide.desktop";
        let clean = id.strip_suffix(".desktop").unwrap_or(id);
        assert_eq!(clean, "antigravity-ide");
    }
}
