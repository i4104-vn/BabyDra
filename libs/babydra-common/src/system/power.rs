//! System power action wrappers.

use std::process::Command;

/// Triggers a system poweroff.
pub fn poweroff() {
    let _ = Command::new("systemctl").arg("poweroff").spawn();
}

/// Triggers a system reboot.
pub fn reboot() {
    let _ = Command::new("systemctl").arg("reboot").spawn();
}

/// Triggers a system suspend.
pub fn suspend() {
    let _ = Command::new("systemctl").arg("suspend").spawn();
}
