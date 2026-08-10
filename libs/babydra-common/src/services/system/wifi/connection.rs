//! WiFi connection profile management and connection triggers.

use zbus::blocking::Connection;
use crate::models::wifi::WifiConfig;
use super::client::{
    get_wifi_device, owned_val_to_str, SettingsProxyBlocking,
    ConnectionSettingsProxyBlocking,
};

pub fn strip_ansi_escapes(input: &str) -> String {
    input.to_string()
}

pub fn connect_wifi(ssid: &str, username: Option<&str>, password: Option<&str>) -> bool {
    if let Some(user) = username {
        let _ = std::process::Command::new("nmcli")
            .args(["connection", "delete", ssid])
            .output();
        let add_status = std::process::Command::new("nmcli")
            .args([
                "connection", "add", 
                "type", "wifi", 
                "con-name", ssid,
                "ifname", "*",
                "ssid", ssid,
                "wifi-sec.key-mgmt", "wpa-eap",
                "802-1x.eap", "peap",
                "802-1x.phase2-auth", "mschapv2",
                "802-1x.identity", user,
                "802-1x.password", password.unwrap_or(""),
            ]).output();
        if !add_status.map(|s| s.status.success()).unwrap_or(false) {
            return false;
        }
        return std::process::Command::new("nmcli")
            .args(["connection", "up", ssid])
            .output()
            .map(|s| s.status.success())
            .unwrap_or(false);
    }

    let mut cmd = std::process::Command::new("nmcli");
    cmd.args(["dev", "wifi", "connect", ssid]);
    
    if let Some(pwd) = password {
        cmd.args(["password", pwd]);
    }
    
    if let Ok(output) = cmd.output() {
        return output.status.success();
    }
    
    false
}

pub fn forget_wifi(ssid: &str) -> bool {
    let mut deleted_any = false;
    let _ = std::process::Command::new("nmcli")
        .args(["connection", "delete", "id", ssid])
        .output();

    if let Ok(output) = std::process::Command::new("nmcli")
        .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && (parts[1] == "802-11-wireless" || parts[1] == "wifi") {
                let conn_name = parts[0];
                let is_match = if conn_name == ssid {
                    true
                } else if let Ok(ssid_out) = std::process::Command::new("nmcli")
                    .args(["-g", "802-11-wireless.ssid", "connection", "show", conn_name])
                    .output()
                {
                    let real_ssid = String::from_utf8_lossy(&ssid_out.stdout).trim().to_string();
                    real_ssid == ssid
                } else {
                    false
                };

                if is_match {
                    let res = std::process::Command::new("nmcli")
                        .args(["connection", "delete", conn_name])
                        .output();
                    if res.map(|s| s.status.success()).unwrap_or(false) {
                        deleted_any = true;
                    }
                }
            }
        }
    }
    deleted_any
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
