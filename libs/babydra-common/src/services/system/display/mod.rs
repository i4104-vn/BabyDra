//! Monitor/Display configuration service.

use crate::models::display::MonitorConfig;
use std::process::Command;

/// Retrieves current connected monitors.
pub fn get_displays() -> Vec<MonitorConfig> {
    let mut monitors = Vec::new();
    
    // Try hyprctl or wlr-randr or xrandr
    if let Ok(output) = Command::new("hyprctl").args(["monitors", "-j"]).output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout) {
                if let Some(arr) = val.as_array() {
                    for m in arr {
                        let id = m.get("id").and_then(|v| v.as_i64()).unwrap_or(0).to_string();
                        let name = m.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let description = m.get("model").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let width = m.get("width").and_then(|v| v.as_u64()).unwrap_or(1920) as u32;
                        let height = m.get("height").and_then(|v| v.as_u64()).unwrap_or(1080) as u32;
                        let refresh_rate = m.get("refreshRate").and_then(|v| v.as_f64()).unwrap_or(60.0);
                        let x = m.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                        let y = m.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                        let transform = m.get("transform").and_then(|v| v.as_i64()).unwrap_or(0);
                        let orientation = match transform {
                            1 => "left",
                            2 => "inverted",
                            3 => "right",
                            _ => "normal",
                        }.to_string();
                        let disabled = m.get("disabled").and_then(|v| v.as_bool()).unwrap_or(false);
                        let mirror = m.get("mirror").and_then(|v| v.as_str()).map(|s| s.to_string());

                        monitors.push(MonitorConfig {
                            id,
                            name,
                            description,
                            resolution_width: width,
                            resolution_height: height,
                            refresh_rate,
                            position_x: x,
                            position_y: y,
                            orientation,
                            mode: if mirror.is_some() { "mirror".to_string() } else { "extend".to_string() },
                            mirror_of: mirror,
                            enabled: !disabled,
                            available_resolutions: vec![
                                "3840x2160".to_string(),
                                "2560x1440".to_string(),
                                "1920x1080".to_string(),
                                "1600x900".to_string(),
                                "1366x768".to_string(),
                                "1280x720".to_string(),
                            ],
                            available_rates: vec![144.0, 120.0, 75.0, 60.0, 50.0],
                        });
                    }
                }
            }
        }
    }

    if monitors.is_empty() {
        // Fallback default monitor
        monitors.push(MonitorConfig {
            id: "0".to_string(),
            name: "eDP-1".to_string(),
            description: "Built-in Display".to_string(),
            resolution_width: 1920,
            resolution_height: 1080,
            refresh_rate: 60.0,
            position_x: 0,
            position_y: 0,
            orientation: "normal".to_string(),
            mode: "extend".to_string(),
            mirror_of: None,
            enabled: true,
            available_resolutions: vec![
                "3840x2160".to_string(),
                "2560x1440".to_string(),
                "1920x1080".to_string(),
                "1366x768".to_string(),
                "1280x720".to_string(),
            ],
            available_rates: vec![144.0, 120.0, 75.0, 60.0],
        });
    }

    monitors
}

/// Saves monitor configurations.
pub fn save_displays(monitors: &[MonitorConfig]) -> Result<(), String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    let path = std::path::PathBuf::from(&home).join(".config/babydra/monitors.conf");

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut lines = Vec::new();
    for m in monitors {
        if !m.enabled {
            lines.push(format!("monitor={},disable", m.name));
        } else if let Some(ref mirror_source) = m.mirror_of {
            lines.push(format!(
                "monitor={},{}x{}@{:.1},{}x{},1,mirror,{}",
                m.name, m.resolution_width, m.resolution_height, m.refresh_rate, m.position_x, m.position_y, mirror_source
            ));
        } else {
            lines.push(format!(
                "monitor={},{}x{}@{:.1},{}x{},1",
                m.name, m.resolution_width, m.resolution_height, m.refresh_rate, m.position_x, m.position_y
            ));
        }
    }

    let content = lines.join("\n");
    std::fs::write(&path, content).map_err(|e| e.to_string())
}
