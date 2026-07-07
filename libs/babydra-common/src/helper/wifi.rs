//! WiFi backend helpers using NetworkManager D-Bus API via `zbus`.

use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant::{ObjectPath, Value};

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    #[zbus(property)]
    fn wireless_enabled(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_wireless_enabled(&self, value: bool) -> zbus::Result<()>;

    #[zbus(property)]
    fn all_devices(&self) -> zbus::Result<Vec<ObjectPath<'static>>>;

    fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<ObjectPath<'static>>;

    fn activate_connection(
        &self,
        connection: &ObjectPath<'_>,
        device: &ObjectPath<'_>,
        specific_object: &ObjectPath<'_>,
    ) -> zbus::Result<(ObjectPath<'static>, ObjectPath<'static>)>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Device",
    default_service = "org.freedesktop.NetworkManager"
)]
trait Device {
    #[zbus(property)]
    fn device_type(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn interface(&self) -> zbus::Result<String>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Device.Wifi",
    default_service = "org.freedesktop.NetworkManager"
)]
trait DeviceWifi {
    #[zbus(property)]
    fn active_access_point(&self) -> zbus::Result<ObjectPath<'static>>;

    fn get_access_points(&self) -> zbus::Result<Vec<ObjectPath<'static>>>;

    fn request_scan(&self, options: HashMap<&str, Value<'_>>) -> zbus::Result<()>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.AccessPoint",
    default_service = "org.freedesktop.NetworkManager"
)]
trait AccessPoint {
    #[zbus(property)]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;

    #[zbus(property)]
    fn strength(&self) -> zbus::Result<u8>;

    #[zbus(property)]
    fn wpa_flags(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn rsn_flags(&self) -> zbus::Result<u32>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Settings",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings"
)]
trait Settings {
    fn list_connections(&self) -> zbus::Result<Vec<ObjectPath<'static>>>;

    fn add_connection(
        &self,
        connection: HashMap<&str, HashMap<&str, Value<'_>>>,
    ) -> zbus::Result<ObjectPath<'static>>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Settings.Connection",
    default_service = "org.freedesktop.NetworkManager"
)]
trait ConnectionSettings {
    fn get_settings(&self) -> zbus::Result<HashMap<String, HashMap<String, Value<'static>>>>;
    fn delete(&self) -> zbus::Result<()>;
}

pub fn strip_ansi_escapes(input: &str) -> String {
    input.to_string()
}

fn val_to_str(val: &Value<'_>) -> Option<String> {
    if let Ok(s) = <&str>::try_from(val) {
        return Some(s.to_string());
    }
    if let Ok(s) = String::try_from(val.clone()) {
        return Some(s);
    }
    None
}

fn get_wifi_device(conn: &Connection) -> Option<ObjectPath<'static>> {
    let nm = NetworkManagerProxyBlocking::new(conn).ok()?;
    let devices = nm.all_devices().ok()?;
    for dev_path in devices {
        if let Ok(dev) = DeviceProxyBlocking::builder(conn).path(dev_path.clone()).ok()?.build() {
            if let Ok(dtype) = dev.device_type() {
                if dtype == 2 {
                    return Some(dev_path);
                }
            }
        }
    }
    None
}

fn generate_uuid() -> String {
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

fn delete_existing_connection(conn: &Connection, ssid: &str) {
    if let Ok(settings) = SettingsProxyBlocking::new(conn) {
        if let Ok(conns) = settings.list_connections() {
            for conn_path in conns {
                if let Ok(c_settings) = ConnectionSettingsProxyBlocking::builder(conn).path(conn_path).ok().and_then(|b| b.build().ok()) {
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

    let wifi_dev = match DeviceWifiProxyBlocking::builder(&conn).path(dev_path).ok().and_then(|b| b.build().ok()) {
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

    let ap = match AccessPointProxyBlocking::builder(&conn).path(ap_path).ok().and_then(|b| b.build().ok()) {
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
            if let Ok(c_settings) = ConnectionSettingsProxyBlocking::builder(&conn).path(conn_path).ok().and_then(|b| b.build().ok()) {
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
        if let Ok(ap) = AccessPointProxyBlocking::builder(&conn).path(ap_path.clone()).ok().and_then(|b| b.build().ok()) {
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
            } else if (wpa & 0x8) != 0 || (rsn & 0x8) != 0 {
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
