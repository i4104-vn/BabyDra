use std::collections::HashMap;
use zbus::blocking::Proxy;
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

use super::dbus::*;
use super::types::*;

pub fn get_vpn_connections() -> Vec<VpnConn> {
    let mut connections = Vec::new();
    let bus = match get_dbus() {
        Ok(b) => b,
        Err(_) => return connections,
    };

    let active_paths = get_active_connection_paths(&bus);

    let settings_proxy = match Proxy::new(
        &bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager/Settings",
        "org.freedesktop.NetworkManager.Settings",
    ) {
        Ok(p) => p,
        Err(_) => return connections,
    };

    let conn_paths: Vec<OwnedObjectPath> = match settings_proxy.call("ListConnections", &()) {
        Ok(paths) => paths,
        Err(_) => return connections,
    };

    for path in conn_paths {
        let path_str = path.as_str();
        let conn_proxy = match Proxy::new(
            &bus,
            "org.freedesktop.NetworkManager",
            path_str,
            "org.freedesktop.NetworkManager.Settings.Connection",
        ) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Ok(settings) = fetch_settings(&conn_proxy) {
            if let Some(conn_setting) = settings.get("connection") {
                let name = conn_setting.get("id").and_then(owned_val_to_string).unwrap_or_default();
                let conn_type = conn_setting.get("type").and_then(owned_val_to_string).unwrap_or_default();

                if is_vpn_type(&conn_type) {
                    let active = active_paths.contains(&path_str.to_string());
                    connections.push(VpnConn {
                        name,
                        conn_type,
                        active,
                        gateway: String::new(),
                        username: String::new(),
                        path: path_str.to_string(),
                    });
                }
            }
        }
    }

    connections
}

pub fn get_vpn_details(name: &str) -> VpnConnDetails {
    let mut details = VpnConnDetails {
        name: name.to_string(),
        original_name: Some(name.to_string()),
        vpn_type: "openvpn".to_string(),
        gateway: String::new(),
        username: String::new(),
        password: String::new(),
        ca_cert: String::new(),
    };

    let bus = match get_dbus() {
        Ok(b) => b,
        Err(_) => return details,
    };

    let settings_proxy = match Proxy::new(
        &bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager/Settings",
        "org.freedesktop.NetworkManager.Settings",
    ) {
        Ok(p) => p,
        Err(_) => return details,
    };

    let conn_paths: Vec<OwnedObjectPath> = match settings_proxy.call("ListConnections", &()) {
        Ok(paths) => paths,
        Err(_) => return details,
    };

    for path in conn_paths {
        let conn_proxy = match Proxy::new(
            &bus,
            "org.freedesktop.NetworkManager",
            path.as_str(),
            "org.freedesktop.NetworkManager.Settings.Connection",
        ) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Ok(settings) = fetch_settings(&conn_proxy) {
            if let Some(conn_setting) = settings.get("connection") {
                let id = conn_setting.get("id").and_then(owned_val_to_string).unwrap_or_default();

                if id == name {
                    if let Some(t) = conn_setting.get("type").and_then(owned_val_to_string) {
                        details.vpn_type = t;
                    }

                    if let Some(vpn_setting) = settings.get("vpn") {
                        if let Some(st) = vpn_setting.get("service-type").and_then(owned_val_to_string) {
                            if let Some(last) = st.split('.').last() {
                                details.vpn_type = last.to_string();
                            }
                        }
                        if let Some(un) = vpn_setting.get("user-name").and_then(owned_val_to_string) {
                            details.username = un;
                        }
                        if let Some(data) = vpn_setting.get("data") {
                            if let Ok(dict) = data.downcast_ref::<zbus::zvariant::Dict>() {
                                for (k, v) in dict.iter() {
                                    if let (Ok(ks), Ok(vs)) = (<&str>::try_from(k), <&str>::try_from(v)) {
                                        match ks {
                                            "remote" | "gateway" => details.gateway = vs.to_string(),
                                            "username" | "user" => details.username = vs.to_string(),
                                            "ca" => details.ca_cert = vs.to_string(),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    details
}

pub fn connect_vpn(name: &str) -> bool {
    let bus = match get_dbus() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let settings_proxy = match Proxy::new(
        &bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager/Settings",
        "org.freedesktop.NetworkManager.Settings",
    ) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let conn_paths: Vec<OwnedObjectPath> = match settings_proxy.call("ListConnections", &()) {
        Ok(paths) => paths,
        Err(_) => return false,
    };

    let mut target_path = None;
    for path in conn_paths {
        let conn_proxy = match Proxy::new(
            &bus,
            "org.freedesktop.NetworkManager",
            path.as_str(),
            "org.freedesktop.NetworkManager.Settings.Connection",
        ) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Ok(settings) = fetch_settings(&conn_proxy) {
            if let Some(conn_setting) = settings.get("connection") {
                let id = conn_setting.get("id").and_then(owned_val_to_string).unwrap_or_default();

                if id == name {
                    target_path = Some(path.clone());
                    break;
                }
            }
        }
    }

    let conn_path = match target_path {
        Some(p) => p,
        None => return false,
    };

    let nm_proxy = match Proxy::new(
        &bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    ) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let null_path = ObjectPath::try_from("/").unwrap();
    let res: Result<OwnedObjectPath, _> = nm_proxy.call("ActivateConnection", &(conn_path, &null_path, &null_path));
    res.is_ok()
}

pub fn disconnect_vpn(name: &str) -> bool {
    let bus = match get_dbus() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let nm_proxy = match Proxy::new(
        &bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    ) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let active_paths: Vec<OwnedObjectPath> = match nm_proxy.get_property("ActiveConnections") {
        Ok(paths) => paths,
        Err(_) => return false,
    };

    for ap in active_paths {
        let active_proxy = match Proxy::new(
            &bus,
            "org.freedesktop.NetworkManager",
            ap.as_str(),
            "org.freedesktop.NetworkManager.Connection.Active",
        ) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Ok(id) = active_proxy.get_property::<String>("Id") {
            if id == name {
                let ap_clone = ap.clone();
                let res: Result<(), _> = nm_proxy.call("DeactivateConnection", &(ap_clone,));
                return res.is_ok();
            }
        }
    }

    false
}

pub fn delete_vpn_connection(name: &str) -> bool {
    let bus = match get_dbus() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let settings_proxy = match Proxy::new(
        &bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager/Settings",
        "org.freedesktop.NetworkManager.Settings",
    ) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let conn_paths: Vec<OwnedObjectPath> = match settings_proxy.call("ListConnections", &()) {
        Ok(paths) => paths,
        Err(_) => return false,
    };

    for path in conn_paths {
        let conn_proxy = match Proxy::new(
            &bus,
            "org.freedesktop.NetworkManager",
            path.as_str(),
            "org.freedesktop.NetworkManager.Settings.Connection",
        ) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Ok(settings) = fetch_settings(&conn_proxy) {
            if let Some(conn_setting) = settings.get("connection") {
                let id = conn_setting.get("id").and_then(owned_val_to_string).unwrap_or_default();

                if id == name {
                    let res: Result<(), _> = conn_proxy.call("Delete", &());
                    return res.is_ok();
                }
            }
        }
    }

    false
}

pub fn save_vpn_connection(details: &VpnConnDetails) -> Result<(), String> {
    let bus = get_dbus()?;

    let settings_proxy = Proxy::new(
        &bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager/Settings",
        "org.freedesktop.NetworkManager.Settings",
    )
    .map_err(|e| e.to_string())?;

    let vpn_type = if details.vpn_type.is_empty() { "openvpn" } else { &details.vpn_type };
    let conn_name = if details.name.is_empty() { "VPN Connection" } else { &details.name };

    let mut settings: HashMap<String, HashMap<String, zbus::zvariant::Value>> = HashMap::new();

    let mut connection_map: HashMap<String, zbus::zvariant::Value> = HashMap::new();
    connection_map.insert("id".to_string(), zbus::zvariant::Value::from(conn_name.to_string()));
    connection_map.insert("uuid".to_string(), zbus::zvariant::Value::from(uuid::Uuid::new_v4().to_string()));

    if vpn_type == "wireguard" {
        connection_map.insert("type".to_string(), zbus::zvariant::Value::from("wireguard"));
    } else {
        connection_map.insert("type".to_string(), zbus::zvariant::Value::from("vpn"));

        let service_type = format!("org.freedesktop.NetworkManager.{}", vpn_type);
        let mut vpn_map: HashMap<String, zbus::zvariant::Value> = HashMap::new();
        vpn_map.insert("service-type".to_string(), zbus::zvariant::Value::from(service_type));
        if !details.username.is_empty() {
            vpn_map.insert("user-name".to_string(), zbus::zvariant::Value::from(details.username.clone()));
        }

        let mut data_map: HashMap<String, String> = HashMap::new();
        if !details.gateway.is_empty() {
            data_map.insert("remote".to_string(), details.gateway.clone());
        }
        if !details.ca_cert.is_empty() {
            data_map.insert("ca".to_string(), details.ca_cert.clone());
        }
        vpn_map.insert("data".to_string(), zbus::zvariant::Value::from(data_map));

        if !details.password.is_empty() {
            let mut secrets_map: HashMap<String, String> = HashMap::new();
            secrets_map.insert("password".to_string(), details.password.clone());
            vpn_map.insert("secrets".to_string(), zbus::zvariant::Value::from(secrets_map));
        }

        settings.insert("vpn".to_string(), vpn_map);
    }

    settings.insert("connection".to_string(), connection_map);

    // If updating existing connection:
    if let Some(ref orig) = details.original_name {
        if !orig.is_empty() {
            let conn_paths: Vec<OwnedObjectPath> = settings_proxy.call("ListConnections", &()).map_err(|e| e.to_string())?;
            for path in conn_paths {
                let conn_proxy = match Proxy::new(
                    &bus,
                    "org.freedesktop.NetworkManager",
                    path.as_str(),
                    "org.freedesktop.NetworkManager.Settings.Connection",
                ) {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                if let Ok(existing_settings) = fetch_settings(&conn_proxy) {
                    if let Some(conn_setting) = existing_settings.get("connection") {
                        let id = conn_setting.get("id").and_then(owned_val_to_string).unwrap_or_default();

                        if id == *orig {
                            let res: Result<(), _> = conn_proxy.call("Update", &(settings,));
                            return res.map_err(|e| format!("Failed to update connection: {}", e));
                        }
                    }
                }
            }
        }
    }

    // Add new connection via D-Bus
    let res: Result<OwnedObjectPath, _> = settings_proxy.call("AddConnection", &(settings,));
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to add D-Bus connection: {}", e)),
    }
}

pub fn import_vpn_profile(path: &str) -> bool {
    let filename = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported VPN");

    let vpn_type = if path.ends_with(".ovpn") { "openvpn" } else { "wireguard" };

    let details = VpnConnDetails {
        name: filename.to_string(),
        original_name: None,
        vpn_type: vpn_type.to_string(),
        gateway: String::new(),
        username: String::new(),
        password: String::new(),
        ca_cert: String::new(),
    };

    save_vpn_connection(&details).is_ok()
}
