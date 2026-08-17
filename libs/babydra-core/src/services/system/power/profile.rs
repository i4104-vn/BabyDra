//! Performance profile service with fallback elevation support.

use crate::config::{load_babydra_config, save_babydra_config};
use crate::models::shell::power::PerformanceProfile;
use std::io::Write;
use std::process::Command;

/// Returns the currently active power profile.
pub fn get_current_profile() -> PerformanceProfile {
    let conf = load_babydra_config();
    PerformanceProfile::from_key(&conf.power.profile)
}

fn save_profile_to_config(profile: PerformanceProfile) {
    let mut conf = load_babydra_config();
    conf.power.profile = profile.key().to_string();
    save_babydra_config(&conf);
}

/// Sets `performance profile` to the given value.
pub fn set_performance_profile(profile: PerformanceProfile) -> Result<(), String> {
    save_profile_to_config(profile);

    let (governor, epp) = match profile {
        PerformanceProfile::Normal => ("powersave", "power"),
        PerformanceProfile::Balanced => ("powersave", "balance_performance"),
        PerformanceProfile::HighPerformance => ("performance", "performance"),
    };

    // Try direct write to sysfs scaling_governor and energy_performance_preference
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
        Ok(())
    } else {
        Err("Permission denied. Sudo password required to update CPU governor.".to_string())
    }
}

/// Sets `performance profile with password` to the given value.
pub fn set_performance_profile_with_password(
    profile: PerformanceProfile,
    password: &str,
) -> Result<(), String> {
    let (governor, epp) = match profile {
        PerformanceProfile::Normal => ("powersave", "power"),
        PerformanceProfile::Balanced => ("powersave", "balance_performance"),
        PerformanceProfile::HighPerformance => ("performance", "performance"),
    };

    let cmd = format!(
        "chmod 666 /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 2>/dev/null || true; for f in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo {} > \"$f\" 2>/dev/null; done; for f in /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference; do echo {} > \"$f\" 2>/dev/null; done",
        governor, epp
    );

    let mut child = match Command::new("sudo")
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
        let _ = stdin.write_all(format!("{}\n", password).as_bytes());
    }

    if let Ok(status) = child.wait() {
        if status.success() {
            save_profile_to_config(profile);
            return Ok(());
        }
    }

    Err("Authentication failed. Incorrect password.".to_string())
}

/// Applies `saved profile`.
pub fn apply_saved_profile() {
    let profile = get_current_profile();
    let _ = set_performance_profile(profile);
}
