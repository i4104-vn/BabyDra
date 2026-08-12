//! Bluetooth subsystem interface querying bluetoothctl.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtDevice {
    pub mac: String,
    pub name: String,
    pub connected: bool,
}

pub fn is_bluetooth_enabled() -> bool {
    if let Ok(output) = Command::new("rfkill").args(["list", "bluetooth"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            if stdout.contains("Soft blocked: yes") || stdout.contains("Hard blocked: yes") {
                return false;
            }
            if stdout.contains("Soft blocked: no") {
                return true;
            }
        }
    }
    if let Ok(output) = Command::new("bluetoothctl").args(["--timeout", "1", "show"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains("Powered: yes")
    } else {
        false
    }
}

pub fn set_bluetooth_enabled(enabled: bool) {
    let rf_arg = if enabled { "unblock" } else { "block" };
    let _ = Command::new("rfkill").args([rf_arg, "bluetooth"]).spawn();

    let bt_arg = if enabled { "power on" } else { "power off" };
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!("bluetoothctl --timeout 1 {}", bt_arg))
        .spawn();
}

pub fn get_bluetooth_devices() -> Vec<BtDevice> {
    let mut devices = Vec::new();

    let output = match Command::new("bluetoothctl").args(["--timeout", "1", "devices"]).output() {
        Ok(out) => out,
        Err(_) => return devices,
    };

    let mut connected_macs = HashSet::new();
    if let Ok(conn_out) = Command::new("bluetoothctl").args(["--timeout", "1", "devices", "Connected"]).output() {
        let conn_stdout = String::from_utf8_lossy(&conn_out.stdout);
        for line in conn_stdout.lines() {
            if line.starts_with("Device ") {
                let parts: Vec<&str> = line.splitn(3, ' ').collect();
                if parts.len() >= 2 {
                    connected_macs.insert(parts[1].to_string());
                }
            }
        }
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Device ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let mac = parts[1].to_string();
                let name = parts[2].to_string();
                let connected = connected_macs.contains(&mac);
                devices.push(BtDevice { mac, name, connected });
            }
        }
    }

    devices
}

pub fn connect_device(mac: &str) -> bool {
    Command::new("bluetoothctl")
        .args(["--timeout", "5", "connect", mac])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn disconnect_device(mac: &str) -> bool {
    Command::new("bluetoothctl")
        .args(["--timeout", "5", "disconnect", mac])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
