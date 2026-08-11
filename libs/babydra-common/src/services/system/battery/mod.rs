//! Battery subsystem service for reading /sys/class/power_supply.

use crate::models::shell::battery::BatteryInfo;
use std::io::Write;
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

                        let info = BatteryInfo {
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
                        };
                        check_and_apply_auto_battery_saver(&info);
                        return Some(info);
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
    let bat = BatteryInfo {
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
    };
    check_and_apply_auto_battery_saver(&bat);
    Some(bat)
}

static LAST_SAVER_CHECK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn check_and_apply_auto_battery_saver(battery_info: &BatteryInfo) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default().as_secs();
    let last = LAST_SAVER_CHECK.load(std::sync::atomic::Ordering::Relaxed);
    if now - last < 60 {
        return;
    }
    LAST_SAVER_CHECK.store(now, std::sync::atomic::Ordering::Relaxed);

    if battery_info.is_ac_only || battery_info.is_charging {
        return;
    }
    let conf = crate::config::load_babydra_config();
    if !conf.power.auto_saver_enabled {
        return;
    }
    if battery_info.percentage <= conf.power.saver_threshold {
        let cur_profile = crate::services::system::power::profile::get_current_profile();
        if cur_profile != crate::PerformanceProfile::Normal {
            if crate::services::system::power::profile::set_performance_profile(crate::PerformanceProfile::Normal).is_ok() {
                let title = crate::i18n::t("settings.notif_auto_saver_title");
                let msg = crate::i18n::t("settings.notif_auto_saver_msg").replace("{level}", &battery_info.percentage.to_string());
                crate::send_notification(&title, &msg);

                // Reduce screen brightness by 50% when auto saver mode activates
                let cur_b = crate::services::system::backlight::get_current_brightness();
                let target_b = (cur_b * 0.5).max(10.0);
                crate::services::system::backlight::set_brightness(target_b);
            }
        }
    }
}

pub fn has_charge_limit_support() -> bool {
    charge_limit_path().is_some()
}

pub fn charge_limit_path() -> Option<std::path::PathBuf> {
    let sysfs_paths = [
        "/sys/class/power_supply/BAT0/charge_control_end_threshold",
        "/sys/class/power_supply/BAT1/charge_control_end_threshold",
        "/sys/class/power_supply/BATT/charge_control_end_threshold",
        "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode",
    ];

    for path_str in &sysfs_paths {
        let p = std::path::Path::new(path_str);
        if p.exists() {
            return Some(p.to_path_buf());
        }
    }
    None
}

pub fn set_charge_limit(limit: u32) -> Result<(), String> {
    let limit = limit.clamp(80, 100);
    let path = match charge_limit_path() {
        Some(p) => p,
        None => return Err("unsupported".to_string()),
    };

    let path_str = path.to_string_lossy();
    let val = if path_str.contains("conservation_mode") {
        if limit < 100 { "1" } else { "0" }
    } else {
        &limit.to_string()
    };

    if std::fs::write(&path, val).is_ok() {
        Ok(())
    } else {
        Err("permission_denied".to_string())
    }
}

pub fn set_charge_limit_auth(limit: u32, pwd: &str) -> Result<(), String> {
    let limit = limit.clamp(80, 100);
    let path = match charge_limit_path() {
        Some(p) => p,
        None => return Err("unsupported".to_string()),
    };

    let path_str = path.to_string_lossy();
    let val = if path_str.contains("conservation_mode") {
        if limit < 100 { "1" } else { "0" }
    } else {
        &limit.to_string()
    };

    let cmd = format!(
        "chmod 666 \"{}\" 2>/dev/null || true; echo \"ACTION==\\\"add|change\\\", SUBSYSTEM==\\\"power_supply\\\", ATTR{{charge_control_end_threshold}}=\\\"\\*\\\", MODE=\\\"0666\\\"\" > /etc/udev/rules.d/99-babydra-battery.rules 2>/dev/null || true; echo {} > \"{}\"",
        path_str, val, path_str
    );

    let mut child = match std::process::Command::new("sudo")
        .args(["-S", "sh", "-c", &cmd])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return Err("Failed to execute sudo".to_string()),
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(format!("{}\n", pwd).as_bytes());
    }

    if let Ok(status) = child.wait() {
        if status.success() {
            return Ok(());
        }
    }

    Err("Authentication failed. Incorrect password.".to_string())
}

