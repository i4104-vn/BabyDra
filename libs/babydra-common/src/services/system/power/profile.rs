//! Performance profile and battery info services.

use crate::models::power::{BatteryInfo, PerformanceProfile};
use std::path::Path;

pub fn get_current_profile() -> PerformanceProfile {
    let config_path = get_profile_config_path();
    if let Ok(content) = std::fs::read_to_string(config_path) {
        PerformanceProfile::from_key(content.trim())
    } else {
        PerformanceProfile::Balanced
    }
}

pub fn set_performance_profile(profile: PerformanceProfile) {
    let config_path = get_profile_config_path();
    if let Some(parent) = config_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&config_path, profile.key());

    let governor = match profile {
        PerformanceProfile::Normal => "powersave",
        PerformanceProfile::Balanced => "schedutil",
        PerformanceProfile::HighPerformance => "performance",
    };
    
    let cmd = format!("echo '{}' | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 2>/dev/null || true", governor);
    let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
}

pub fn get_profile_config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    Path::new(&home).join(".config/babydra/perf_profile")
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
                    
                    if capacity_opt.is_none() {
                        let energy_now = std::fs::read_to_string(path.join("energy_now"))
                            .or_else(|_| std::fs::read_to_string(path.join("charge_now")))
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok());
                        let energy_full = std::fs::read_to_string(path.join("energy_full"))
                            .or_else(|_| std::fs::read_to_string(path.join("charge_full")))
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok());
                        if let (Some(now), Some(full)) = (energy_now, energy_full) {
                            if full > 0.0 {
                                capacity_opt = Some(((now / full) * 100.0) as u32);
                            }
                        }
                    }

                    let capacity = capacity_opt.unwrap_or(100);
                    let status = std::fs::read_to_string(path.join("status"))
                        .unwrap_or_default();
                    let is_charging = status.trim().eq_ignore_ascii_case("Charging");

                    return Some(BatteryInfo {
                        percentage: capacity.min(100),
                        is_charging,
                    });
                }
            }
        }
    }

    None
}
