//! Performance profile service with fallback elevation support.

use crate::models::shell::power::PerformanceProfile;
use std::process::Command;
use crate::config::{load_babydra_config, save_babydra_config, get_babydra_conf_path};

pub fn get_current_profile() -> PerformanceProfile {
    let conf = load_babydra_config();
    PerformanceProfile::from_key(&conf.power.profile)
}

fn save_profile_to_config(profile: PerformanceProfile) {
    let mut conf = load_babydra_config();
    conf.power.profile = profile.key().to_string();
    save_babydra_config(&conf);
}

pub fn set_performance_profile(profile: PerformanceProfile) -> Result<(), String> {
    save_profile_to_config(profile);

    let (governor, epp) = match profile {
        PerformanceProfile::Normal => ("powersave", "power"),
        PerformanceProfile::Balanced => ("powersave", "balance_performance"),
        PerformanceProfile::HighPerformance => ("performance", "performance"),
    };

    // 1. Try direct write to sysfs scaling_governor and energy_performance_preference
    let mut direct_success = false;
    if let Ok(entries) = std::fs::read_dir("/sys/devices/system/cpu") {
        let mut writen_count = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            let gov_path = path.join("cpufreq/scaling_governor");
            if gov_path.exists() {
                if std::fs::write(&gov_path, governor).is_ok() {
                    writen_count += 1;
                }
            }

            let epp_path = path.join("cpufreq/energy_performance_preference");
            if epp_path.exists() {
                let _ = std::fs::write(&epp_path, epp);
            }
        }
        if writen_count > 0 {
            direct_success = true;
        }
    }

    if direct_success {
        return Ok(());
    }

    // 2. Fallback: use pkexec to chmod sysfs files or write directly
    let cmd_chmod = "chmod 666 /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 2>/dev/null || true";
    if let Ok(status) = Command::new("pkexec").args(["sh", "-c", cmd_chmod]).status() {
        if status.success() {
            // Retry direct write after chmod
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
            return Ok(());
        }
    }

    // 3. Direct pkexec write fallback
    let cmd_write = format!(
        "for f in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo {} > \"$f\" 2>/dev/null; done; for f in /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference; do echo {} > \"$f\" 2>/dev/null; done",
        governor, epp
    );
    if let Ok(status) = Command::new("pkexec").args(["sh", "-c", &cmd_write]).status() {
        if status.success() {
            return Ok(());
        }
    }

    Err("Permission denied. Could not update CPU governor.".to_string())
}

pub fn set_performance_profile_with_password(profile: PerformanceProfile, password: &str) -> Result<(), String> {
    let (governor, epp) = match profile {
        PerformanceProfile::Normal => ("powersave", "power"),
        PerformanceProfile::Balanced => ("powersave", "balance_performance"),
        PerformanceProfile::HighPerformance => ("performance", "performance"),
    };

    let safe_pwd = password.replace('\'', "'\\''");
    let cmd = format!(
        "echo '{}' | sudo -S sh -c 'chmod 666 /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 2>/dev/null || true; for f in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo {} > \"$f\" 2>/dev/null; done; for f in /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference; do echo {} > \"$f\" 2>/dev/null; done'",
        safe_pwd, governor, epp
    );

    if let Ok(status) = Command::new("sh").args(["-c", &cmd]).status() {
        if status.success() {
            save_profile_to_config(profile);
            return Ok(());
        }
    }

    Err("Authentication failed. Incorrect password.".to_string())
}

pub fn apply_saved_profile() {
    let profile = get_current_profile();
    let _ = set_performance_profile(profile);
}

pub fn get_profile_config_path() -> std::path::PathBuf {
    get_babydra_conf_path()
}
