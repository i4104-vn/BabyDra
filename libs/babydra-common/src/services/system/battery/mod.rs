//! Battery subsystem service for reading /sys/class/power_supply.

use crate::models::shell::battery::BatteryInfo;
use std::path::Path;

fn format_duration(total_minutes: u32, is_charging: bool) -> String {
    let hours = total_minutes / 60;
    let mins = total_minutes % 60;
    if is_charging {
        if hours > 0 {
            format!("{}h {}m until full charge", hours, mins)
        } else {
            format!("{}m until full charge", mins)
        }
    } else {
        if hours > 0 {
            format!("{}h {}m remaining", hours, mins)
        } else {
            format!("{}m remaining", mins)
        }
    }
}

pub fn get_battery_info() -> Option<BatteryInfo> {
    let power_dir = Path::new("/sys/class/power_supply");
    if !power_dir.exists() { return None; }
    if let Ok(entries) = std::fs::read_dir(power_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(kind) = std::fs::read_to_string(path.join("type")) {
                if kind.trim() == "Battery" {
                    if let Ok(scope) = std::fs::read_to_string(path.join("scope")) {
                        if scope.trim().eq_ignore_ascii_case("Device") {
                            continue;
                        }
                    }
                    let mut capacity_opt = std::fs::read_to_string(path.join("capacity"))
                        .ok()
                        .and_then(|s| s.trim().parse::<u32>().ok());

                    let energy_now = std::fs::read_to_string(path.join("energy_now"))
                        .or_else(|_| std::fs::read_to_string(path.join("charge_now")))
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok());
                    let energy_full = std::fs::read_to_string(path.join("energy_full"))
                        .or_else(|_| std::fs::read_to_string(path.join("charge_full")))
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok());
                    let power_now = std::fs::read_to_string(path.join("power_now"))
                        .or_else(|_| std::fs::read_to_string(path.join("current_now")))
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok());

                    if capacity_opt.is_none() {
                        if let (Some(now), Some(full)) = (energy_now, energy_full) {
                            if full > 0.0 {
                                capacity_opt = Some(((now / full) * 100.0) as u32);
                            }
                        }
                    }

                    let capacity = capacity_opt.unwrap_or(100).min(100);
                    let status_raw = std::fs::read_to_string(path.join("status"))
                        .unwrap_or_else(|_| "Discharging".to_string());
                    let status_trim = status_raw.trim();
                    let is_charging = status_trim.eq_ignore_ascii_case("Charging");

                    // Compute time estimation
                    let mut time_remaining: Option<String> = None;
                    if is_charging {
                        if let Ok(secs_str) = std::fs::read_to_string(path.join("time_to_full_now")) {
                            if let Ok(secs) = secs_str.trim().parse::<u64>() {
                                if secs > 0 && secs < 86400 {
                                    time_remaining = Some(format_duration((secs / 60) as u32, true));
                                }
                            }
                        }
                        if time_remaining.is_none() {
                            if let (Some(now), Some(full), Some(power)) = (energy_now, energy_full, power_now) {
                                if power > 0.0 && full > now {
                                    let mins = (((full - now) / power) * 60.0) as u32;
                                    if mins > 0 && mins < 1440 {
                                        time_remaining = Some(format_duration(mins, true));
                                    }
                                }
                            }
                        }
                    } else if status_trim.eq_ignore_ascii_case("Discharging") {
                        if let Ok(secs_str) = std::fs::read_to_string(path.join("time_to_empty_now")) {
                            if let Ok(secs) = secs_str.trim().parse::<u64>() {
                                if secs > 0 && secs < 86400 {
                                    time_remaining = Some(format_duration((secs / 60) as u32, false));
                                }
                            }
                        }
                        if time_remaining.is_none() {
                            if let (Some(now), Some(power)) = (energy_now, power_now) {
                                if power > 0.0 && now > 0.0 {
                                    let mins = ((now / power) * 60.0) as u32;
                                    if mins > 0 && mins < 1440 {
                                        time_remaining = Some(format_duration(mins, false));
                                    }
                                }
                            }
                        }
                    }

                    return Some(BatteryInfo {
                        percentage: capacity,
                        is_charging,
                        status_text: status_trim.to_string(),
                        time_remaining,
                    });
                }
            }
        }
    }

    None
}
