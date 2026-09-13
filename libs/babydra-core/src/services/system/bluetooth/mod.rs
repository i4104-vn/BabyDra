pub use crate::models::settings::bluetooth::BtDevice;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::process::Command;
use zbus::blocking::Connection;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

#[zbus::proxy(
    gen_blocking = true,
    default_service = "org.bluez",
    default_path = "/",
    interface = "org.freedesktop.DBus.ObjectManager"
)]
trait BluezObjectManager {
    fn get_managed_objects(
        &self,
    ) -> zbus::Result<HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>>;
}

/// Represents a connected Bluetooth device with optional battery percentage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BtConnectedDevice {
    pub mac: String,
    pub name: String,
    pub battery: Option<u8>,
}

/// Returns all currently connected Bluetooth devices with their battery percentage if available.
/// Queries BlueZ D-Bus ObjectManager directly (0 subprocesses spawned).
pub fn get_connected_bt_devices() -> Vec<BtConnectedDevice> {
    if let Ok(conn) = Connection::system() {
        if let Ok(manager) = BluezObjectManagerProxyBlocking::new(&conn) {
            if let Ok(objects) = manager.get_managed_objects() {
                let mut connected_devs = Vec::new();
                for (_path, interfaces) in objects {
                    if let Some(dev_props) = interfaces.get("org.bluez.Device1") {
                        let is_connected = dev_props
                            .get("Connected")
                            .map(|v| match v.deref() {
                                Value::Bool(b) => *b,
                                _ => false,
                            })
                            .unwrap_or(false);

                        if is_connected {
                            let mac = dev_props
                                .get("Address")
                                .and_then(|v| match v.deref() {
                                    Value::Str(s) => Some(s.as_str().to_string()),
                                    _ => None,
                                })
                                .unwrap_or_default();

                            let name = dev_props
                                .get("Alias")
                                .or_else(|| dev_props.get("Name"))
                                .and_then(|v| match v.deref() {
                                    Value::Str(s) => Some(s.as_str().to_string()),
                                    _ => None,
                                })
                                .unwrap_or_else(|| mac.clone());

                            let battery = interfaces.get("org.bluez.Battery1").and_then(|bat_props| {
                                bat_props.get("Percentage").and_then(|v| match v.deref() {
                                    Value::U8(b) => Some(*b),
                                    Value::I16(n) => Some(*n as u8),
                                    Value::U16(n) => Some(*n as u8),
                                    Value::I32(n) => Some(*n as u8),
                                    Value::U32(n) => Some(*n as u8),
                                    _ => None,
                                })
                            });

                            connected_devs.push(BtConnectedDevice {
                                mac,
                                name,
                                battery,
                            });
                        }
                    }
                }
                return connected_devs;
            }
        }
    }

    Vec::new()
}

/// Returns `true` when `bluetooth enabled` holds, `false` otherwise.
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
    if let Ok(output) = Command::new("bluetoothctl")
        .args(["--timeout", "1", "show"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains("Powered: yes")
    } else {
        false
    }
}

/// Enables or disables the Bluetooth adapter.
pub fn set_bt_enabled(enabled: bool) {
    let rf_arg = if enabled { "unblock" } else { "block" };
    let _ = Command::new("rfkill").args([rf_arg, "bluetooth"]).spawn();

    let bt_arg = if enabled { "power on" } else { "power off" };
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!("bluetoothctl --timeout 1 {}", bt_arg))
        .spawn();
}

/// Returns the current `bluetooth devices`.
pub fn get_bt_devices() -> Vec<BtDevice> {
    let mut devices = Vec::new();

    let output = match Command::new("bluetoothctl")
        .args(["--timeout", "1", "devices"])
        .output()
    {
        Ok(out) => out,
        Err(_) => return devices,
    };

    let mut connected_macs = HashSet::new();
    if let Ok(conn_out) = Command::new("bluetoothctl")
        .args(["--timeout", "1", "devices", "Connected"])
        .output()
    {
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
                devices.push(BtDevice {
                    mac,
                    name,
                    connected,
                });
            }
        }
    }

    devices
}

/// Connects to `device`.
pub fn connect_device(mac: &str) -> bool {
    Command::new("bluetoothctl")
        .args(["--timeout", "5", "connect", mac])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Disconnects from `device`.
pub fn disconnect_device(mac: &str) -> bool {
    Command::new("bluetoothctl")
        .args(["--timeout", "5", "disconnect", mac])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
