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

    let governor = match profile {
        PerformanceProfile::Normal => "powersave",
        PerformanceProfile::Balanced => "schedutil",
        PerformanceProfile::HighPerformance => "performance",
    };
    
    if let Ok(entries) = std::fs::read_dir("/sys/devices/system/cpu") {
        for entry in entries.flatten() {
            let gov_path = entry.path().join("cpufreq/scaling_governor");
            if gov_path.exists() {
                let _ = std::fs::write(gov_path, governor);
            }
        }
    }
}

pub fn get_profile_config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    Path::new(&home).join(".config/babydra/perf_profile")
}
