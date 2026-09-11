//! Hardware battery charge control threshold and conservation mode configuration.

use crate::error::CoreResult;
use std::path::{Path, PathBuf};

/// Returns `true` when battery charge limiting is supported by hardware sysfs.
pub fn has_charge_limit() -> bool {
    charge_limit_path().is_some()
}

/// Returns the sysfs path for controlling battery charge limit, if detected.
pub fn charge_limit_path() -> Option<PathBuf> {
    let sysfs_paths = [
        "/sys/class/power_supply/BAT0/charge_control_end_threshold",
        "/sys/class/power_supply/BAT1/charge_control_end_threshold",
        "/sys/class/power_supply/BATT/charge_control_end_threshold",
        "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode",
    ];

    for path_str in &sysfs_paths {
        let p = Path::new(path_str);
        if p.exists() {
            return Some(p.to_path_buf());
        }
    }
    None
}

/// Sets battery charge limit to the given value (80-100%).
pub fn set_charge_limit(limit: u32) -> CoreResult<()> {
    let limit = limit.clamp(80, 100);
    let path = match charge_limit_path() {
        Some(p) => p,
        None => return Err("unsupported".into()),
    };

    let path_str = path.to_string_lossy();
    let val = if path_str.contains("conservation_mode") {
        if limit < 100 {
            "1"
        } else {
            "0"
        }
    } else {
        &limit.to_string()
    };

    if std::fs::write(&path, val).is_ok() {
        Ok(())
    } else {
        Err("permission_denied".into())
    }
}

/// Sets battery charge limit using sudo authentication.
pub fn set_charge_limit_pw(limit: u32, pwd: &str) -> CoreResult<()> {
    let limit = limit.clamp(80, 100);
    let path = match charge_limit_path() {
        Some(p) => p,
        None => return Err("unsupported".into()),
    };

    let path_str = path.to_string_lossy();
    let val = if path_str.contains("conservation_mode") {
        if limit < 100 {
            "1"
        } else {
            "0"
        }
    } else {
        &limit.to_string()
    };

    let cmd = format!(
        "chmod 666 \"{}\" 2>/dev/null || true; echo \"ACTION==\\\"add|change\\\", SUBSYSTEM==\\\"power_supply\\\", ATTR{{charge_control_end_threshold}}=\\\"\\*\\\", MODE=\\\"0666\\\"\" > /etc/udev/rules.d/99-babydra-battery.rules 2>/dev/null || true; echo {} > \"{}\"",
        path_str, val, path_str
    );

    crate::services::utils::run_sudo(pwd, "sh", &["-c", &cmd], None)
        .map_err(|_| "Authentication failed. Incorrect password.".into())
}
