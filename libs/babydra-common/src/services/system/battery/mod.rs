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
    let active_profile = Some(crate::services::system::power::profile::get_current_profile().label().to_string());
    let power_dir = Path::new("/sys/class/power_supply");

    if power_dir.exists() {
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
                        let energy_full_design = std::fs::read_to_string(path.join("energy_full_design"))
                            .or_else(|_| std::fs::read_to_string(path.join("charge_full_design")))
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

                        let health = match (energy_full, energy_full_design) {
                            (Some(full), Some(design)) if design > 0.0 => {
                                let pct = (full / design) * 100.0;
                                Some(format!("{:.1}% Health", pct.min(100.0)))
                            }
                            _ => std::fs::read_to_string(path.join("health"))
                                .ok()
                                .map(|s| s.trim().to_string()),
                        };

                        let technology = std::fs::read_to_string(path.join("technology"))
                            .ok()
                            .map(|s| s.trim().to_string());

                        let power_source = if is_charging {
                            Some("AC Power Adapter".to_string())
                        } else {
                            Some("Internal Battery".to_string())
                        };

                        let cycle_count = std::fs::read_to_string(path.join("cycle_count"))
                            .ok()
                            .and_then(|s| s.trim().parse::<u32>().ok());

                        let voltage = std::fs::read_to_string(path.join("voltage_now"))
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok())
                            .map(|uv_val| format!("{:.2} V", uv_val / 1_000_000.0));

                        let energy_rate = power_now.map(|uw_val| {
                            let watts = uw_val / 1_000_000.0;
                            if is_charging {
                                format!("+{:.1} W", watts)
                            } else {
                                format!("-{:.1} W", watts)
                            }
                        });

                        let capacity_wh = match (energy_now, energy_full) {
                            (Some(now), Some(full)) => Some(format!("{:.1} Wh / {:.1} Wh", now / 1_000_000.0, full / 1_000_000.0)),
                            _ => None,
                        };

                        let design_capacity = energy_full_design.map(|design_val| {
                            format!("{:.1} Wh", design_val / 1_000_000.0)
                        });

                        let manufacturer = std::fs::read_to_string(path.join("manufacturer"))
                            .or_else(|_| std::fs::read_to_string(path.join("vendor")))
                            .ok()
                            .map(|s| s.trim().to_string());

                        let model_name = std::fs::read_to_string(path.join("model_name"))
                            .ok()
                            .map(|s| s.trim().to_string());

                        let serial_number = std::fs::read_to_string(path.join("serial_number"))
                            .ok()
                            .map(|s| s.trim().to_string());

                        let temperature = std::fs::read_to_string(path.join("temp"))
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok())
                            .map(|t_val| format!("{:.1} °C", t_val / 10.0));

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
                            is_ac_only: false,
                            status_text: status_trim.to_string(),
                            time_remaining,
                            health,
                            technology,
                            power_source,
                            cycle_count,
                            voltage,
                            energy_rate,
                            capacity_wh,
                            design_capacity,
                            manufacturer,
                            model_name,
                            serial_number,
                            temperature,
                            active_profile,
                        });
                    }
                }
            }
        }
    }

    // Desktop PC / Device without physical battery: Read AC power supply info if present
    let mut power_source_name = "AC Power Adapter".to_string();
    let mut online_status = "Online".to_string();

    if power_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(power_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(kind) = std::fs::read_to_string(path.join("type")) {
                    let k = kind.trim();
                    if k == "Mains" || k == "AC" {
                        if let Ok(online) = std::fs::read_to_string(path.join("online")) {
                            if online.trim() == "0" {
                                online_status = "Offline".to_string();
                            }
                        }
                        if let Ok(model) = std::fs::read_to_string(path.join("model_name")) {
                            power_source_name = model.trim().to_string();
                        }
                    }
                }
            }
        }
    }

    Some(BatteryInfo {
        percentage: 100,
        is_charging: true,
        is_ac_only: true,
        status_text: online_status,
        time_remaining: None,
        health: Some("N/A (Direct AC)".to_string()),
        technology: Some("Direct Mains".to_string()),
        power_source: Some(power_source_name),
        cycle_count: None,
        voltage: None,
        energy_rate: None,
        capacity_wh: None,
        design_capacity: None,
        manufacturer: None,
        model_name: None,
        serial_number: None,
        temperature: None,
        active_profile,
    })
}
