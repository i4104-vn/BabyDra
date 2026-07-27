//! Bluetooth subsystem interface querying bluetoothctl.

use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtDevice {
    pub mac: String,
    pub name: String,
    pub connected: bool,
}

pub fn is_bluetooth_enabled() -> bool {
    if let Ok(output) = Command::new("bluetoothctl").arg("show").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains("Powered: yes")
    } else {
        false
    }
}

pub fn set_bluetooth_enabled(enabled: bool) {
    let arg = if enabled { "power on" } else { "power off" };
    let _ = Command::new("sh").arg("-c").arg(&format!("bluetoothctl {}", arg)).output();
}

pub fn get_bluetooth_devices() -> Vec<BtDevice> {
    let mut devices = Vec::new();
    
    let output = match Command::new("bluetoothctl").arg("devices").output() {
        Ok(out) => out,
        Err(_) => return devices,
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Device ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let mac = parts[1].to_string();
                let name = parts[2].to_string();
                
                let mut connected = false;
                if let Ok(info_out) = Command::new("bluetoothctl").arg("info").arg(&mac).output() {
                    let info_str = String::from_utf8_lossy(&info_out.stdout);
                    connected = info_str.contains("Connected: yes");
                }
                
                devices.push(BtDevice { mac, name, connected });
            }
        }
    }
    
    devices
}

pub fn connect_device(mac: &str) -> bool {
    Command::new("bluetoothctl")
        .arg("connect")
        .arg(mac)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn disconnect_device(mac: &str) -> bool {
    Command::new("bluetoothctl")
        .arg("disconnect")
        .arg(mac)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

