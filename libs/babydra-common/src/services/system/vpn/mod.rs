//! VPN and WireGuard connection management querying nmcli.

use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConn {
    pub name: String,
    pub conn_type: String,
    pub active: bool,
    pub gateway: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VpnConnDetails {
    pub name: String,
    pub original_name: Option<String>,
    pub vpn_type: String,
    pub gateway: String,
    pub username: String,
    pub password: String,
    pub ca_cert: String,
}

fn is_vpn_type(t: &str) -> bool {
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

pub fn get_vpn_connections() -> Vec<VpnConn> {
    let mut connections = Vec::new();
    let output = match Command::new("nmcli").args(&["-g", "name,type,active", "connection", "show"]).output() {
        Ok(out) => out,
        Err(_) => return connections,
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 {
            let name = parts[0].to_string();
            let conn_type = parts[1].to_string();
            let active = parts[2] == "yes";

            if is_vpn_type(&conn_type) {
                connections.push(VpnConn {
                    name,
                    conn_type,
                    active,
                    gateway: String::new(),
                    username: String::new(),
                });
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

    if let Ok(out) = Command::new("nmcli").args(&["connection", "show", name]).output() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            if parts.len() >= 2 {
                let key = parts[0];
                let val = parts[1..].join(":");
                match key {
                    "connection.type" => {
                        details.vpn_type = val;
                    }
                    "vpn.data" => {
                        for pair in val.split(',') {
                            let kv: Vec<&str> = pair.split('=').map(|s| s.trim()).collect();
                            if kv.len() == 2 {
                                match kv[0] {
                                    "remote" | "gateway" => details.gateway = kv[1].to_string(),
                                    "username" | "user" => details.username = kv[1].to_string(),
                                    "ca" => details.ca_cert = kv[1].to_string(),
                                    _ => {}
                                }
                            }
                        }
                    }
                    "vpn.user-name" => details.username = val,
                    _ => {}
                }
            }
        }
    }
    details
}

pub fn connect_vpn(name: &str) -> bool {
    Command::new("nmcli")
        .args(&["connection", "up", name])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn disconnect_vpn(name: &str) -> bool {
    Command::new("nmcli")
        .args(&["connection", "down", name])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn delete_vpn_connection(name: &str) -> bool {
    Command::new("nmcli")
        .args(&["connection", "delete", name])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn save_vpn_connection(details: &VpnConnDetails) -> Result<(), String> {
    if let Some(ref orig) = details.original_name {
        if !orig.is_empty() {
            // Modify existing connection
            if orig != &details.name {
                let _ = Command::new("nmcli").args(&["connection", "modify", orig, "connection.id", &details.name]).status();
            }
            let conn_name = &details.name;
            if !details.gateway.is_empty() {
                let vpn_data = format!("remote={}, username={}", details.gateway, details.username);
                let _ = Command::new("nmcli").args(&["connection", "modify", conn_name, "vpn.data", &vpn_data]).status();
            }
            if !details.username.is_empty() {
                let _ = Command::new("nmcli").args(&["connection", "modify", conn_name, "vpn.user-name", &details.username]).status();
            }
            if !details.password.is_empty() {
                let _ = Command::new("nmcli").args(&["connection", "modify", conn_name, "vpn.secrets", &format!("password={}", details.password)]).status();
            }
            return Ok(());
        }
    }

    // Add new connection
    let vpn_type = if details.vpn_type.is_empty() { "openvpn" } else { &details.vpn_type };
    let conn_name = if details.name.is_empty() { "VPN Connection" } else { &details.name };

    let status = if vpn_type == "wireguard" {
        Command::new("nmcli")
            .args(&["connection", "add", "type", "wireguard", "con-name", conn_name])
            .status()
    } else {
        let vpn_data = format!("remote={}, username={}", details.gateway, details.username);
        Command::new("nmcli")
            .args(&[
                "connection", "add",
                "type", "vpn",
                "vpn-type", vpn_type,
                "con-name", conn_name,
                "vpn.data", &vpn_data,
            ])
            .status()
    };

    match status {
        Ok(s) if s.success() => {
            if !details.password.is_empty() {
                let _ = Command::new("nmcli").args(&["connection", "modify", conn_name, "vpn.secrets", &format!("password={}", details.password)]).status();
            }
            Ok(())
        }
        Ok(_) => Err("Failed to save VPN connection via nmcli".to_string()),
        Err(e) => Err(format!("nmcli error: {}", e)),
    }
}

pub fn import_vpn_profile(path: &str) -> bool {
    let type_str = if path.ends_with(".ovpn") {
        "openvpn"
    } else if path.contains("wireguard") || path.ends_with(".conf") {
        "wireguard"
    } else {
        "openvpn"
    };
    Command::new("nmcli")
        .args(&["connection", "import", "type", type_str, "file", path])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
