//! Apply display configurations via wlr-randr commands.

use crate::error::CoreResult;
use crate::models::display::MonitorConfig;
use std::process::Command;

/// Helper to try applying a specific mode string via wlr-randr.
fn try_wlr_randr_mode(name: &str, mode: &str, pos: &str, transform: &str) -> bool {
    Command::new("wlr-randr")
        .args([
            "--output",
            name,
            "--on",
            "--mode",
            mode,
            "--pos",
            pos,
            "--transform",
            transform,
        ])
        .output()
        .map(|o| o.status.success() && !String::from_utf8_lossy(&o.stderr).contains("unknown mode"))
        .unwrap_or(false)
}

/// Applies display configurations directly via wlr-randr.
pub fn apply_display_configs(monitors: &[MonitorConfig]) -> CoreResult<()> {
    let wlr_json_val: Option<serde_json::Value> = Command::new("wlr-randr")
        .arg("--json")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| serde_json::from_slice(&o.stdout).ok());

    for m in monitors {
        if !m.enabled {
            let _ = Command::new("wlr-randr")
                .args(["--output", &m.name, "--off"])
                .status();
            continue;
        }

        let transform_arg = match m.orientation.as_str() {
            "left" | "90" => "90",
            "inverted" | "180" => "180",
            "right" | "270" => "270",
            _ => "normal",
        };

        let pos_str = format!("{},{}", m.position_x, m.position_y);

        let mut exact_mode_str: Option<String> = None;
        if let Some(ref val) = wlr_json_val {
            if let Some(arr) = val.as_array() {
                for mon_val in arr {
                    if mon_val.get("name").and_then(|v| v.as_str()) == Some(&m.name) {
                        if let Some(modes) = mon_val.get("modes").and_then(|v| v.as_array()) {
                            let mut best_match: Option<(f64, f64)> = None;
                            for mode in modes {
                                let w = mode.get("width").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                let h = mode.get("height").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                let refresh = mode.get("refresh").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                if w == m.resolution_width && h == m.resolution_height && refresh > 0.0 {
                                    let diff = (refresh - m.refresh_rate).abs();
                                    if diff < 1.0 && best_match.map_or(true, |(best_diff, _)| diff < best_diff) {
                                        best_match = Some((diff, refresh));
                                    }
                                }
                            }
                            if let Some((_, exact_refresh)) = best_match {
                                exact_mode_str = Some(format!(
                                    "{}x{}@{}Hz",
                                    m.resolution_width, m.resolution_height, exact_refresh
                                ));
                            }
                        }
                    }
                }
            }
        }

        let primary_mode = exact_mode_str.unwrap_or_else(|| {
            if m.refresh_rate.fract().abs() < 0.001 {
                format!("{}x{}@{:.0}Hz", m.resolution_width, m.resolution_height, m.refresh_rate)
            } else {
                format!("{}x{}@{:.3}Hz", m.resolution_width, m.resolution_height, m.refresh_rate)
            }
        });

        // Try primary mode -> integer Hz fallback -> resolution-only fallback
        if !try_wlr_randr_mode(&m.name, &primary_mode, &pos_str, transform_arg) {
            let int_hz_mode = format!(
                "{}x{}@{:.0}Hz",
                m.resolution_width, m.resolution_height, m.refresh_rate
            );
            if !try_wlr_randr_mode(&m.name, &int_hz_mode, &pos_str, transform_arg) {
                let res_only_mode = format!("{}x{}", m.resolution_width, m.resolution_height);
                let _ = Command::new("wlr-randr")
                    .args([
                        "--output",
                        &m.name,
                        "--on",
                        "--mode",
                        &res_only_mode,
                        "--pos",
                        &pos_str,
                        "--transform",
                        transform_arg,
                    ])
                    .status();
            }
        }
    }
    Ok(())
}
