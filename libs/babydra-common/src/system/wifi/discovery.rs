//! WiFi access point scanning and discovery.

use std::collections::HashMap;
use zbus::blocking::Connection;
use super::client::{
    get_wifi_device, AccessPointProxyBlocking, DeviceWifiProxyBlocking, SettingsProxyBlocking,
    ConnectionSettingsProxyBlocking, val_to_str,
};

pub fn known_networks() -> Vec<String> {
    let mut ssids = Vec::new();
    let conn = match Connection::system() {
        Ok(c) => c,
        Err(_) => return ssids,
    };

    let settings = match SettingsProxyBlocking::new(&conn) {
        Ok(s) => s,
        Err(_) => return ssids,
    };

    if let Ok(conns) = settings.list_connections() {
        for conn_path in conns {
            if let Some(c_settings) = ConnectionSettingsProxyBlocking::builder(&conn).path(conn_path).ok().and_then(|b| b.build().ok()) {
                if let Ok(details) = c_settings.get_settings() {
                    if details.contains_key("802-11-wireless") {
                        if let Some(conn_sec) = details.get("connection") {
                            if let Some(id_val) = conn_sec.get("id") {
                                if let Some(id_str) = val_to_str(id_val) {
                                    ssids.push(id_str);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ssids
}

pub fn scan_networks() -> Vec<(String, String, String, bool)> {
    let mut networks = Vec::new();
    let conn = match Connection::system() {
        Ok(c) => c,
        Err(_) => return networks,
    };

    let dev_path = match get_wifi_device(&conn) {
        Some(p) => p,
        None => return networks,
    };

    let wifi_dev = match DeviceWifiProxyBlocking::builder(&conn).path(dev_path).ok().and_then(|b| b.build().ok()) {
        Some(d) => d,
        None => return networks,
    };

    // Request async scan
    let scan_opts = HashMap::new();
    let _ = wifi_dev.request_scan(scan_opts);
    std::thread::sleep(std::time::Duration::from_millis(300));

    let active_ap_path = wifi_dev.active_access_point().ok();

    let aps = wifi_dev.get_access_points().unwrap_or_default();
    let mut seen_ssids = std::collections::HashSet::new();

    for ap_path in aps {
        if let Some(ap) = AccessPointProxyBlocking::builder(&conn).path(ap_path.clone()).ok().and_then(|b| b.build().ok()) {
            let ssid_bytes = ap.ssid().unwrap_or_default();
            let ssid = String::from_utf8_lossy(&ssid_bytes).to_string();
            if ssid.is_empty() {
                continue;
            }

            if !seen_ssids.insert(ssid.clone()) {
                continue;
            }

            let signal = ap.strength().unwrap_or(0).to_string();
            let wpa = ap.wpa_flags().unwrap_or(0);
            let rsn = ap.rsn_flags().unwrap_or(0);

            let security = if wpa == 0 && rsn == 0 {
                "open".to_string()
            } else if (wpa & 0x200) != 0 || (rsn & 0x200) != 0 {
                "8021x".to_string()
            } else {
                "psk".to_string()
            };

            let is_connected = active_ap_path.as_ref().map(|path| path == &ap_path).unwrap_or(false);
            networks.push((ssid, security, signal, is_connected));
        }
    }
    networks
}
