//! VPN and WireGuard connection management querying nmcli.

use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConn {
    pub name: String,
    pub conn_type: String,
    pub active: bool,
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

            if conn_type == "vpn" || conn_type == "wireguard" {
                connections.push(VpnConn { name, conn_type, active });
            }
        }
    }
    connections
}
