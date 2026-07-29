//! Monitor/Display configuration service.

use crate::models::display::MonitorConfig;
use std::process::Command;

/// Retrieves current connected monitors.
pub fn get_displays() -> Vec<MonitorConfig> {
    let mut monitors = Vec::new();
    
    // 1. Try wlr-randr --json
    if let Ok(output) = Command::new("wlr-randr").arg("--json").output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout) {
                if let Some(arr) = val.as_array() {
                    for (idx, m) in arr.iter().enumerate() {
                        let name = m.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        if name.is_empty() { continue; }

                        let make = m.get("make").and_then(|v| v.as_str()).unwrap_or("");
                        let model = m.get("model").and_then(|v| v.as_str()).unwrap_or("");
                        let description = if !make.is_empty() || !model.is_empty() {
                            format!("{make} {model}").trim().to_string()
                        } else {
                            m.get("description").and_then(|v| v.as_str()).unwrap_or("Display").to_string()
                        };

                        let enabled = m.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
                        let x = m.get("position").and_then(|p| p.get("x")).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                        let y = m.get("position").and_then(|p| p.get("y")).and_then(|v| v.as_i64()).unwrap_or(0) as i32;

                        let transform_str = m.get("transform").and_then(|v| v.as_str()).unwrap_or("normal");
                        let orientation = match transform_str {
                            "90" | "270" | "left" | "right" | "inverted" | "180" => transform_str.to_string(),
                            _ => "normal".to_string(),
                        };

                        let mut cur_w = 1920;
                        let mut cur_h = 1080;
                        let mut cur_rate = 60.0;
                        let mut res_list: Vec<String> = Vec::new();
                        let mut rate_list: Vec<f64> = Vec::new();

                        if let Some(modes_arr) = m.get("modes").and_then(|v| v.as_array()) {
                            for mode in modes_arr {
                                let w = mode.get("width").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                let h = mode.get("height").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                let refresh = mode.get("refresh").and_then(|v| v.as_f64()).unwrap_or(60.0);
                                let is_current = mode.get("current").and_then(|v| v.as_bool()).unwrap_or(false);

                                if is_current {
                                    cur_w = w;
                                    cur_h = h;
                                    cur_rate = refresh;
                                }

                                if w > 0 && h > 0 {
                                    let res_str = format!("{w}x{h}");
                                    if !res_list.contains(&res_str) {
                                        res_list.push(res_str);
                                    }
                                }

                                let rounded_rate = (refresh * 10.0).round() / 10.0;
                                if !rate_list.contains(&rounded_rate) {
                                    rate_list.push(rounded_rate);
                                }
                            }
                        }

                        if res_list.is_empty() {
                            res_list = vec!["1920x1080".to_string(), "1280x720".to_string()];
                        }
                        if rate_list.is_empty() {
                            rate_list = vec![60.0];
                        }

                        // Sort resolutions descending by area (width * height)
                        res_list.sort_by(|a, b| {
                            let parse = |s: &String| {
                                let parts: Vec<&str> = s.split('x').collect();
                                if parts.len() == 2 {
                                    let w = parts[0].parse::<u32>().unwrap_or(0);
                                    let h = parts[1].parse::<u32>().unwrap_or(0);
                                    (w * h, w, h)
                                } else {
                                    (0, 0, 0)
                                }
                            };
                            parse(b).cmp(&parse(a))
                        });

                        // Sort refresh rates descending (highest Hz first)
                        rate_list.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

                        monitors.push(MonitorConfig {
                            id: idx.to_string(),
                            name,
                            description,
                            resolution_width: cur_w,
                            resolution_height: cur_h,
                            refresh_rate: cur_rate,
                            position_x: x,
                            position_y: y,
                            orientation,
                            mode: "extend".to_string(),
                            mirror_of: None,
                            enabled,
                            available_resolutions: res_list,
                            available_rates: rate_list,
                        });
                    }
                }
            }
        }
    }

    monitors
}

/// Saves monitor configurations and applies changes via wlr-randr.
pub fn save_displays(monitors: &[MonitorConfig]) -> Result<(), String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    let path = std::path::PathBuf::from(&home).join(".config/babydra/monitors.conf");

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // Try to get exact mode refresh rates from wlr-randr --json
    let wlr_json_val: Option<serde_json::Value> = Command::new("wlr-randr")
        .arg("--json")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| serde_json::from_slice(&o.stdout).ok());

    let mut lines = Vec::new();
    for m in monitors {
        if !m.enabled {
            lines.push(format!("monitor={},disable", m.name));
            // Apply via wlr-randr
            let _ = Command::new("wlr-randr")
                .args(["--output", &m.name, "--off"])
                .status();
        } else {
            lines.push(format!(
                "monitor={},{}x{}@{:.1},{}x{},1",
                m.name, m.resolution_width, m.resolution_height, m.refresh_rate, m.position_x, m.position_y
            ));

            let transform_arg = match m.orientation.as_str() {
                "left" | "90" => "90",
                "inverted" | "180" => "180",
                "right" | "270" => "270",
                _ => "normal",
            };

            let pos_str = format!("{},{}", m.position_x, m.position_y);
            let mode_res_only = format!("{}x{}", m.resolution_width, m.resolution_height);

            // Find exact mode string matching resolution and target refresh rate
            let mut exact_mode_str: Option<String> = None;

            if let Some(ref val) = wlr_json_val {
                if let Some(arr) = val.as_array() {
                    for mon_val in arr {
                        let name = mon_val.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        if name == m.name {
                            if let Some(modes) = mon_val.get("modes").and_then(|v| v.as_array()) {
                                let mut best_match: Option<(f64, f64)> = None; // (difference, refresh)
                                for mode in modes {
                                    let w = mode.get("width").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                    let h = mode.get("height").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                    let refresh = mode.get("refresh").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                    if w == m.resolution_width && h == m.resolution_height && refresh > 0.0 {
                                        let diff = (refresh - m.refresh_rate).abs();
                                        if diff < 1.0 {
                                            if best_match.map_or(true, |(best_diff, _)| diff < best_diff) {
                                                best_match = Some((diff, refresh));
                                            }
                                        }
                                    }
                                }
                                if let Some((_, exact_refresh)) = best_match {
                                    exact_mode_str = Some(format!("{}x{}@{}", m.resolution_width, m.resolution_height, exact_refresh));
                                }
                            }
                        }
                    }
                }
            }

            let mode_arg = exact_mode_str.unwrap_or_else(|| format!("{}x{}@{:.1}", m.resolution_width, m.resolution_height, m.refresh_rate));

            let output = Command::new("wlr-randr")
                .args([
                    "--output", &m.name,
                    "--on",
                    "--mode", &mode_arg,
                    "--pos", &pos_str,
                    "--transform", transform_arg,
                ])
                .output();

            let success = output.as_ref().map(|o| o.status.success() && !String::from_utf8_lossy(&o.stderr).contains("unknown mode")).unwrap_or(false);

            if !success {
                let fallback_mode_str = format!("{}x{}@{:.1}", m.resolution_width, m.resolution_height, m.refresh_rate);
                let output2 = Command::new("wlr-randr")
                    .args([
                        "--output", &m.name,
                        "--on",
                        "--mode", &fallback_mode_str,
                        "--pos", &pos_str,
                        "--transform", transform_arg,
                    ])
                    .output();

                let success2 = output2.as_ref().map(|o| o.status.success() && !String::from_utf8_lossy(&o.stderr).contains("unknown mode")).unwrap_or(false);

                if !success2 {
                    // Final fallback to mode without rate
                    let _ = Command::new("wlr-randr")
                        .args([
                            "--output", &m.name,
                            "--on",
                            "--mode", &mode_res_only,
                            "--pos", &pos_str,
                            "--transform", transform_arg,
                        ])
                        .status();
                }
            }
        }
    }

    let content = lines.join("\n");
    std::fs::write(&path, content).map_err(|e| e.to_string())
}
