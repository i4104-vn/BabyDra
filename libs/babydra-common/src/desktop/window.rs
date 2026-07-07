//! Window layout builders, layer shell configuration, and compositor window helpers.

use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use gtk4::prelude::*;

/// Standard window layer shell initialization helper to configure widgets as desktop shell layers.
pub fn init_layer_window(
    window: &gtk4::ApplicationWindow,
    layer: Layer,
    kbd_mode: KeyboardMode,
    exclusive_zone: i32,
    anchors: &[(Edge, bool)],
    margin_bottom: i32,
) {
    window.init_layer_shell();
    window.set_layer(layer);
    window.set_keyboard_mode(kbd_mode);
    window.set_exclusive_zone(exclusive_zone);
    for &(edge, anchor) in anchors {
        window.set_anchor(edge, anchor);
    }
    if margin_bottom > 0 {
        window.set_margin(Edge::Bottom, margin_bottom);
    }
}

/// Registers a click gesture to dismiss/close the window when clicking outside the specified container.
pub fn setup_click_outside_dismiss<W: IsA<gtk4::Widget>, C: IsA<gtk4::Widget>>(
    window: &W,
    container: &C,
) {
    let click_gesture = gtk4::GestureClick::new();
    let container_c = container.clone();
    let window_c = window.clone();
    click_gesture.connect_pressed(move |_, _, x, y| {
        let picked = window_c.pick(x, y, gtk4::PickFlags::DEFAULT);
        let inside = picked
            .map(|w| w.is_ancestor(&container_c) || w == container_c)
            .unwrap_or(false);
        if !inside {
            if let Some(win) = window_c.clone().dynamic_cast::<gtk4::Window>().ok() {
                win.close();
            }
        }
    });
    window.add_controller(click_gesture);
}

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

/// Activates/Focuses an application window using wlrctl via custom rules.
pub fn focus_app(name: &str, exec: &str, app_id: Option<&str>, window_title: Option<&str>) {
    if let Some(title) = window_title {
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

    if let Some(id) = app_id {
        let status = std::process::Command::new("wlrctl")
            .args(&["window", "focus", id])
            .status();
        if let Ok(s) = status {
            if s.success() {
                return;
            }
        }
        let status_lower = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &id.to_lowercase()])
            .status();
        if let Ok(s) = status_lower {
            if s.success() {
                return;
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
        let _ = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &exec_name])
            .status();
        let _ = std::process::Command::new("wlrctl")
            .args(&["window", "focus", &format!("title:{}", exec_name)])
            .status();
    }
    
    if !exec.is_empty() {
        let _ = std::process::Command::new("wlrctl")
            .args(&["window", "focus", exec])
            .status();
    }

    let _ = std::process::Command::new("wlrctl")
        .args(&["window", "focus", name])
        .status();
    let _ = std::process::Command::new("wlrctl")
        .args(&["window", "focus", &format!("title:{}", name)])
        .status();
}
