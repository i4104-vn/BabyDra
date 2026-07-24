//! Performance profile service.

use crate::models::power::PerformanceProfile;
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

    let (governor, epp) = match profile {
        PerformanceProfile::Normal => ("powersave", "power"),
        PerformanceProfile::Balanced => ("powersave", "balance_performance"),
        PerformanceProfile::HighPerformance => ("performance", "performance"),
    };

    // 1. Try powerprofilesctl (non-root DBus service if running)
    let _ = std::process::Command::new("powerprofilesctl")
        .arg("set")
        .arg(match profile {
            PerformanceProfile::Normal => "power-saver",
            PerformanceProfile::Balanced => "balanced",
            PerformanceProfile::HighPerformance => "performance",
        })
        .spawn();

    // 2. Direct write to sysfs scaling_governor and energy_performance_preference (zero sudo/password needed!)
    if let Ok(entries) = std::fs::read_dir("/sys/devices/system/cpu") {
        for entry in entries.flatten() {
            let path = entry.path();
            let gov_path = path.join("cpufreq/scaling_governor");
            if gov_path.exists() {
                let _ = std::fs::write(&gov_path, governor);
            }

            let epp_path = path.join("cpufreq/energy_performance_preference");
            if epp_path.exists() {
                let _ = std::fs::write(&epp_path, epp);
            }
        }
    }
}

pub fn get_profile_config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    Path::new(&home).join(".config/babydra/perf_profile")
}
