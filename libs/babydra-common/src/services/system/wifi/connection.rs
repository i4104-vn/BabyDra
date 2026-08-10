//! WiFi connection profile management and connection triggers.

use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant::{ObjectPath, Value};
use crate::models::wifi::WifiConfig;
use super::client::{
    get_wifi_device, owned_val_to_str, NetworkManagerProxyBlocking, SettingsProxyBlocking,
    ConnectionSettingsProxyBlocking,
};

pub fn strip_ansi_escapes(input: &str) -> String {
    input.to_string()
}

pub fn generate_uuid() -> String {
    let mut bytes = [0u8; 16];
    for i in 0..16 {
        bytes[i] = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() >> (i * 4)) as u8;
    }
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

pub fn delete_existing_connection(conn: &Connection, ssid: &str) {
    if let Ok(settings) = SettingsProxyBlocking::new(conn) {
        if let Ok(conns) = settings.list_connections() {
            for conn_path in conns {
                if let Some(c_settings) = ConnectionSettingsProxyBlocking::builder(conn).path(conn_path).ok().and_then(|b| b.build().ok()) {
                    if let Ok(details) = c_settings.get_settings() {
                        if let Some(conn_sec) = details.get("connection") {
                            if let Some(id_val) = conn_sec.get("id") {
                                if let Some(id_str) = owned_val_to_str(id_val) {
                                    if id_str == ssid {
                                        let _ = c_settings.delete();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn connect_wifi(ssid: &str, username: Option<&str>, password: Option<&str>) -> bool {
    let conn = match Connection::system() {
        Ok(c) => c,
        Err(_) => return false,
    };

    let dev_path = match get_wifi_device(&conn) {
        Some(p) => p,
        None => return false,
    };

    delete_existing_connection(&conn, ssid);

    let settings = match SettingsProxyBlocking::new(&conn) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let mut connection_profile = HashMap::new();

    // 1. Connection setting
    let mut s_con = HashMap::new();
    s_con.insert("id", Value::from(ssid));
    s_con.insert("type", Value::from("802-11-wireless"));
    s_con.insert("uuid", Value::from(generate_uuid()));
    connection_profile.insert("connection", s_con);

    // 2. Wireless setting
    let mut s_wifi = HashMap::new();
    s_wifi.insert("ssid", Value::from(ssid.as_bytes().to_vec()));
    s_wifi.insert("mode", Value::from("infrastructure"));
    connection_profile.insert("802-11-wireless", s_wifi);

    // 3. Security settings
    let is_enterprise = username.is_some();
    let has_password = password.is_some();

    if is_enterprise {
        let mut s_wifi_sec = HashMap::new();
        s_wifi_sec.insert("key-mgmt", Value::from("wpa-eap"));
        connection_profile.insert("802-11-wireless-security", s_wifi_sec);

        let mut s_8021x = HashMap::new();
        s_8021x.insert("eap", Value::from(vec!["peap"]));
        s_8021x.insert("phase2-auth", Value::from("mschapv2"));
        if let Some(user) = username {
            s_8021x.insert("identity", Value::from(user));
        }
        if let Some(pass) = password {
            s_8021x.insert("password", Value::from(pass));
        }
        connection_profile.insert("802-1x", s_8021x);
    } else if has_password {
        let mut s_wifi_sec = HashMap::new();
        s_wifi_sec.insert("key-mgmt", Value::from("wpa-psk"));
        if let Some(pass) = password {
            s_wifi_sec.insert("psk", Value::from(pass));
        }
        connection_profile.insert("802-11-wireless-security", s_wifi_sec);
    }

    let new_conn_path = match settings.add_connection(connection_profile) {
        Ok(path) => path,
        Err(_) => return false,
    };

    let nm = match NetworkManagerProxyBlocking::new(&conn) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let null_path = ObjectPath::try_from("/").unwrap();
    nm.activate_connection(&new_conn_path, &dev_path, &null_path).is_ok()
}

pub fn get_wifi_config(ssid: &str) -> WifiConfig {
    let mut config = WifiConfig {
        method: "auto".to_string(),
        ip_address: String::new(),
        prefix: 24,
        gateway: String::new(),
        dns: String::new(),
        bssid: None,
        frequency: None,
        speed: None,
        interface: None,
        mac_address: None,
    };

    let conn = match Connection::system() {
        Ok(c) => c,
        Err(_) => return config,
    };

    if let Some(dev_path) = get_wifi_device(&conn) {
        if let Ok(builder) = super::client::DeviceProxyBlocking::builder(&conn).path(dev_path.clone()) {
            if let Ok(dev) = builder.build() {
                if let Ok(iface) = dev.interface() {
                    config.interface = Some(iface);
                }
            }
        }
    }

    if let Ok(settings) = SettingsProxyBlocking::new(&conn) {
        if let Ok(conns) = settings.list_connections() {
            for conn_path in conns {
                if let Some(c_settings) = ConnectionSettingsProxyBlocking::builder(&conn).path(conn_path).ok().and_then(|b| b.build().ok()) {
                    if let Ok(details) = c_settings.get_settings() {
                        let is_target = details.get("connection")
                            .and_then(|sec| sec.get("id"))
                            .and_then(owned_val_to_str)
                            .map(|id| id == ssid)
                            .unwrap_or(false);

                        if is_target {
                            if let Some(ipv4) = details.get("ipv4") {
                                if let Some(m) = ipv4.get("method").and_then(owned_val_to_str) {
                                    config.method = m;
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    if let Some(ref iface) = config.interface {
        if let Ok(output) = std::process::Command::new("ip").args(["-4", "addr", "show", iface]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("inet ") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let ip_prefix = parts[1];
                        if let Some((ip, pfx)) = ip_prefix.split_once('/') {
                            config.ip_address = ip.to_string();
                            if let Ok(p) = pfx.parse::<u32>() {
                                config.prefix = p;
                            }
                        }
                    }
                }
            }
        }

        if config.gateway.is_empty() {
            if let Ok(output) = std::process::Command::new("ip").args(["route", "show", "dev", iface]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 && parts[0] == "default" && parts[1] == "via" {
                        config.gateway = parts[2].to_string();
                        break;
                    }
                }
            }
        }
    }

    config
}

pub fn set_wifi_config(ssid: &str, new_config: &WifiConfig) -> bool {
    let method = if new_config.method == "manual" { "manual" } else { "auto" };
    let mut cmd = std::process::Command::new("nmcli");
    cmd.args(["connection", "modify", ssid, "ipv4.method", method]);

    if new_config.method == "manual" && !new_config.ip_address.is_empty() {
        let ip_with_prefix = format!("{}/{}", new_config.ip_address, new_config.prefix);
        cmd.args(["ipv4.addresses", &ip_with_prefix]);
        if !new_config.gateway.is_empty() {
            cmd.args(["ipv4.gateway", &new_config.gateway]);
        }
    } else {
        cmd.args(["ipv4.addresses", "", "ipv4.gateway", ""]);
    }

    if !new_config.dns.is_empty() {
        cmd.args(["ipv4.dns", &new_config.dns]);
    } else {
        cmd.args(["ipv4.dns", ""]);
    }

    let status = cmd.status();
    if status.map(|s| s.success()).unwrap_or(false) {
        let _ = std::process::Command::new("nmcli").args(["connection", "up", ssid]).status();
        return true;
    }

    false
}
