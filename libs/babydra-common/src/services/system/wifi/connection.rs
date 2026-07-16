//! WiFi connection profile management and connection triggers.

use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant::{ObjectPath, Value};
use super::client::{
    get_wifi_device, NetworkManagerProxyBlocking, SettingsProxyBlocking,
    ConnectionSettingsProxyBlocking, val_to_str,
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
                                if let Some(id_str) = val_to_str(id_val) {
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
