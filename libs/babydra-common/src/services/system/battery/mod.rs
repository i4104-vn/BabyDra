//! Battery subsystem service for reading /sys/class/power_supply.

use crate::models::battery::BatteryInfo;
use std::path::Path;

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
