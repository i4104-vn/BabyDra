use std::collections::HashMap;
use zbus::blocking::Connection as DBusConn;
use zbus::blocking::Proxy;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Str};

pub fn get_dbus() -> Result<DBusConn, String> {
    DBusConn::system().map_err(|e| format!("Failed to connect to system bus: {}", e))
}

pub fn owned_val_to_string(v: &OwnedValue) -> Option<String> {
    v.downcast_ref::<Str>().ok().map(|s| s.as_str().to_string())
}

pub fn is_vpn_type(t: &str) -> bool {
    let lower = t.to_lowercase();
    lower == "vpn"
        || lower == "wireguard"
        || lower.contains("vpn")
        || lower.contains("wireguard")
        || lower.contains("openconnect")
        || lower.contains("pptp")
        || lower.contains("l2tp")
        || lower.contains("strongswan")
}

pub fn get_active_connection_paths(bus: &DBusConn) -> Vec<String> {
    let mut active_conn_paths = Vec::new();
    let proxy = match Proxy::new(
        bus,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    ) {
        Ok(p) => p,
        Err(_) => return active_conn_paths,
    };

    if let Ok(paths) = proxy.get_property::<Vec<OwnedObjectPath>>("ActiveConnections") {
        for ap in paths {
            if let Ok(active_proxy) = Proxy::new(
                bus,
                "org.freedesktop.NetworkManager",
                ap.as_str(),
                "org.freedesktop.NetworkManager.Connection.Active",
            ) {
                if let Ok(conn_path) = active_proxy.get_property::<OwnedObjectPath>("Connection") {
                    active_conn_paths.push(conn_path.as_str().to_string());
                }
            }
        }
    }
    active_conn_paths
}

pub fn fetch_settings(proxy: &Proxy) -> Result<HashMap<String, HashMap<String, OwnedValue>>, zbus::Error> {
    proxy.call("GetSettings", &())
}
