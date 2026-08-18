//! WiFi power state toggling and current interface status.

use super::client::{
    get_wifi_device, AccessPointProxyBlocking, DeviceWifiProxyBlocking, NetworkManagerProxyBlocking,
};
use zbus::blocking::Connection;

/// Returns the current `wifi state`.
pub fn get_wifi_state() -> (bool, String) {
    let conn = match Connection::system() {
        Ok(c) => c,
        Err(_) => return (false, "Off".to_string()),
    };

    let nm = match NetworkManagerProxyBlocking::new(&conn) {
        Ok(m) => m,
        Err(_) => return (false, "Off".to_string()),
    };

    let is_enabled = nm.wireless_enabled().unwrap_or(false);
    if !is_enabled {
        return (false, "Off".to_string());
    }

    let dev_path = match get_wifi_device(&conn) {
        Some(p) => p,
        None => return (true, "Disconnected".to_string()),
    };

    let wifi_dev = match DeviceWifiProxyBlocking::builder(&conn)
        .path(dev_path)
        .ok()
        .and_then(|b| b.build().ok())
    {
        Some(d) => d,
        None => return (true, "Disconnected".to_string()),
    };

    let ap_path = match wifi_dev.active_access_point() {
        Ok(path) => path,
        Err(_) => return (true, "Disconnected".to_string()),
    };

    if ap_path.as_str() == "/" {
        return (true, "Disconnected".to_string());
    }

    let ap = match AccessPointProxyBlocking::builder(&conn)
        .path(ap_path)
        .ok()
        .and_then(|b| b.build().ok())
    {
        Some(a) => a,
        None => return (true, "Disconnected".to_string()),
    };

    let ssid_bytes = ap.ssid().unwrap_or_default();
    let ssid = String::from_utf8_lossy(&ssid_bytes).to_string();
    if ssid.is_empty() {
        (true, "Disconnected".to_string())
    } else {
        (true, ssid)
    }
}

/// Enables or disables `wifi`.
pub fn set_wifi_enabled(enabled: bool) -> bool {
    let conn = match Connection::system() {
        Ok(c) => c,
        Err(_) => return false,
    };

    if let Ok(nm) = NetworkManagerProxyBlocking::new(&conn) {
        return nm.set_wireless_enabled(enabled).is_ok();
    }
    false
}

/// Returns the current `wifi signal strength`.
pub fn get_wifi_signal() -> (bool, bool, u8) {
    let conn = match Connection::system() {
        Ok(c) => c,
        Err(_) => return (false, false, 0),
    };

    let nm = match NetworkManagerProxyBlocking::new(&conn) {
        Ok(m) => m,
        Err(_) => return (false, false, 0),
    };

    let is_enabled = nm.wireless_enabled().unwrap_or(false);
    if !is_enabled {
        return (false, false, 0);
    }

    let dev_path = match get_wifi_device(&conn) {
        Some(p) => p,
        None => return (true, false, 0),
    };

    let wifi_dev = match DeviceWifiProxyBlocking::builder(&conn)
        .path(dev_path)
        .ok()
        .and_then(|b| b.build().ok())
    {
        Some(d) => d,
        None => return (true, false, 0),
    };

    let ap_path = match wifi_dev.active_access_point() {
        Ok(path) => path,
        Err(_) => return (true, false, 0),
    };

    if ap_path.as_str() == "/" {
        return (true, false, 0);
    }

    let ap = match AccessPointProxyBlocking::builder(&conn)
        .path(ap_path)
        .ok()
        .and_then(|b| b.build().ok())
    {
        Some(a) => a,
        None => return (true, false, 0),
    };

    let strength = ap.strength().unwrap_or(0);
    (true, true, strength)
}
