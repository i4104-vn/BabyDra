//! System power action wrappers.

use std::process::Command;

/// Triggers a system poweroff.
pub fn poweroff() {
    println!("Power Off requested...");
    let _ = Command::new("systemctl").arg("poweroff").spawn();
}

/// Triggers a system reboot.
pub fn reboot() {
    println!("Reboot requested...");
    let _ = Command::new("systemctl").arg("reboot").spawn();
}

/// Triggers a system suspend.
pub fn suspend() {
    println!("Suspend requested...");
    let _ = Command::new("systemctl").arg("suspend").spawn();
}
