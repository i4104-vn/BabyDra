//! Display and connected monitor discovery service via wlr-randr.

use crate::models::display::MonitorConfig;
use std::process::Command;

/// Retrieves current connected monitors.
pub fn get_displays() -> Vec<MonitorConfig> {
    let mut monitors = Vec::new();

    let output = match Command::new("wlr-randr").arg("--json").output() {
        Ok(out) if out.status.success() => out,
        _ => return monitors,
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout) else {
        return monitors;
    };

    let Some(arr) = val.as_array() else {
        return monitors;
    };

    for (idx, m) in arr.iter().enumerate() {
        let name = m
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            continue;
        }

        let make = m.get("make").and_then(|v| v.as_str()).unwrap_or("");
        let model = m.get("model").and_then(|v| v.as_str()).unwrap_or("");
        let description = if !make.is_empty() || !model.is_empty() {
            format!("{make} {model}").trim().to_string()
        } else {
            m.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("Display")
                .to_string()
        };

        let enabled = m.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
        let x = m
            .get("position")
            .and_then(|p| p.get("x"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let y = m
            .get("position")
            .and_then(|p| p.get("y"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        let transform_str = m
            .get("transform")
            .and_then(|v| v.as_str())
            .unwrap_or("normal");
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
                let is_current = mode
                    .get("current")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

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

    monitors
}
