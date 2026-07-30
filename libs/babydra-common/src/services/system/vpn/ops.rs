use std::collections::HashSet;
use std::process::Command;
use crate::models::vpn::*;

fn run_nmcli(args: &[&str]) -> Option<String> {
    Command::new("nmcli")
        .args(args)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
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

pub fn get_vpn_connections() -> Vec<VpnConn> {
    let mut connections = Vec::new();
    let active_names: HashSet<String> = run_nmcli(&["-t", "-f", "NAME,TYPE", "connection", "show", "--active"])
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && is_vpn_type(parts[1]) {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect();

    if let Some(stdout) = run_nmcli(&["-t", "-f", "NAME,TYPE,UUID", "connection", "show"]) {
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && is_vpn_type(parts[1]) {
                let name = parts[0].to_string();
                let conn_type = parts[1].to_string();
                let active = active_names.contains(&name);
                
                let mut gateway = String::new();
                let mut username = String::new();
                if let Some(details_str) = run_nmcli(&["-t", "-f", "vpn.data,vpn.user-name", "connection", "show", &name]) {
                    for dline in details_str.lines() {
                        if dline.starts_with("vpn.user-name:") {
                            username = dline.trim_start_matches("vpn.user-name:").to_string();
                        } else if dline.starts_with("vpn.data:") {
                            let data_str = dline.trim_start_matches("vpn.data:");
                            for item in data_str.split(',') {
                                let kv: Vec<&str> = item.split('=').map(|s| s.trim()).collect();
                                if kv.len() == 2 && (kv[0] == "remote" || kv[0] == "gateway") {
                                    gateway = kv[1].to_string();
                                }
                            }
                        }
                    }
                }

                connections.push(VpnConn {
                    name,
                    conn_type,
                    active,
                    gateway,
                    username,
                    path: String::new(),
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
        config_file: None,
    };

    if let Some(stdout) = run_nmcli(&["--show-secrets", "-t", "connection", "show", name]) {
        for line in stdout.lines() {
            if let Some((key, val)) = line.split_once(':') {
                match key {
                    "connection.type" => {
                        if val != "vpn" && !val.is_empty() {
                            details.vpn_type = val.to_string();
                        }
                    }
                    "vpn.service-type" => {
                        if let Some(last) = val.split('.').last() {
                            details.vpn_type = last.to_string();
                        }
                    }
                    "vpn.user-name" => {
                        if !val.is_empty() {
                            details.username = val.to_string();
                        }
                    }
                    "vpn.secrets" => {
                        if let Some((s_key, s_val)) = val.split_once('=') {
                            if s_key.trim() == "password" {
                                details.password = s_val.trim().to_string();
                            }
                        }
                    }
                    "vpn.data" => {
                        for item in val.split(',') {
                            if let Some((d_key, d_val)) = item.split_once('=') {
                                match d_key.trim() {
                                    "remote" | "gateway" => details.gateway = d_val.trim().to_string(),
                                    "username" | "user" => {
                                        if details.username.is_empty() {
                                            details.username = d_val.trim().to_string();
                                        }
                                    }
                                    "ca" => details.ca_cert = d_val.trim().to_string(),
                                    _ => {}
                                }
                            }
                        }
                    }
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

pub fn copy_vpn_config_to_babydra_dir(src_path: &str) -> Result<String, String> {
    let vpn_dir = crate::config::get_babydra_config_dir().join("vpn");
    std::fs::create_dir_all(&vpn_dir).map_err(|e| format!("Failed to create config dir: {}", e))?;

    let filename = std::path::Path::new(src_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("vpn_profile.conf");

    let dest_path = vpn_dir.join(filename);
    std::fs::copy(src_path, &dest_path).map_err(|e| format!("Failed to copy config file: {}", e))?;

    Ok(dest_path.to_string_lossy().to_string())
}

pub fn parse_vpn_config_file(path: &str) -> VpnConnDetails {
    let mut details = VpnConnDetails::default();
    details.config_file = Some(path.to_string());

    let path_obj = std::path::Path::new(path);
    if let Some(stem) = path_obj.file_stem().and_then(|s| s.to_str()) {
        details.name = stem.to_string();
    }

    if path.ends_with(".ovpn") {
        details.vpn_type = "openvpn".to_string();
    } else if path.ends_with(".conf") || path.contains("wireguard") {
        details.vpn_type = "wireguard".to_string();
    } else {
        details.vpn_type = "openvpn".to_string();
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return details,
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.starts_with(';') || trimmed.is_empty() {
            continue;
        }

        if details.vpn_type == "openvpn" {
            if trimmed.starts_with("remote ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    if parts.len() >= 3 {
                        details.gateway = format!("{}:{}", parts[1], parts[2]);
                    } else {
                        details.gateway = parts[1].to_string();
                    }
                }
            } else if trimmed.starts_with("ca ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    details.ca_cert = parts[1].to_string();
                }
            }
        }

        if details.vpn_type == "wireguard" {
            if trimmed.to_lowercase().starts_with("endpoint") {
                let parts: Vec<&str> = trimmed.split('=').map(|s| s.trim()).collect();
                if parts.len() >= 2 {
                    details.gateway = parts[1].to_string();
                }
            }
        }
    }

    details
}

pub fn save_vpn_connection(details: &VpnConnDetails) -> Result<(), String> {
    if let Some(ref src_path) = details.config_file {
        if !src_path.is_empty() {
            let _ = copy_vpn_config_to_babydra_dir(src_path);
        }
    }

    let conn_name = if details.name.is_empty() { "VPN Connection" } else { &details.name };
    let orig_name = details.original_name.as_deref().unwrap_or(conn_name);

    let name_str = conn_name.to_string();
    let orig_str = orig_name.to_string();
    let un_str = details.username.clone();
    let pw_str = details.password.clone();
    let gw_str = details.gateway.clone();
    let ca_str = details.ca_cert.clone();
    let vpn_type = if details.vpn_type.is_empty() { "openvpn" } else { &details.vpn_type };

    let exists = run_nmcli(&["connection", "show", &orig_str]).is_some();

    if exists {
        if orig_str != name_str {
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &orig_str, "connection.id", &name_str])
                .output();
        }
        if !un_str.is_empty() {
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &name_str, "vpn.user-name", &un_str])
                .output();
        }
        if !pw_str.is_empty() {
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &name_str, "vpn.secrets", &format!("password={}", pw_str)])
                .output();
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &name_str, "+vpn.data", "password-flags=0"])
                .output();
        }
        if !gw_str.is_empty() {
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &name_str, "+vpn.data", &format!("remote={}", gw_str)])
                .output();
        }
        if !ca_str.is_empty() {
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &name_str, "+vpn.data", &format!("ca={}", ca_str)])
                .output();
        }
        return Ok(());
    }

    // New connection creation via nmcli
    let service_type = format!("org.freedesktop.NetworkManager.{}", vpn_type);
    let mut add_args = vec![
        "connection".to_string(),
        "add".to_string(),
        "type".to_string(),
        "vpn".to_string(),
        "con-name".to_string(),
        name_str.clone(),
        "vpn-type".to_string(),
        vpn_type.to_string(),
        "vpn.service-type".to_string(),
        service_type,
    ];

    if !un_str.is_empty() {
        add_args.push("vpn.user-name".to_string());
        add_args.push(un_str.clone());
    }

    let mut vpn_data_items = vec!["password-flags=0".to_string()];
    if !gw_str.is_empty() {
        vpn_data_items.push(format!("remote={}", gw_str));
    }
    if !ca_str.is_empty() {
        vpn_data_items.push(format!("ca={}", ca_str));
    }
    let conn_type = if !ca_str.is_empty() && !un_str.is_empty() {
        "password-tls"
    } else if !ca_str.is_empty() {
        "tls"
    } else {
        "password"
    };
    vpn_data_items.push(format!("connection-type={}", conn_type));

    let vpn_data_str = vpn_data_items.join(",");
    add_args.push("vpn.data".to_string());
    add_args.push(vpn_data_str);

    if !pw_str.is_empty() {
        add_args.push("vpn.secrets".to_string());
        add_args.push(format!("password={}", pw_str));
    }

    let status = Command::new("nmcli")
        .args(&add_args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if status {
        if !pw_str.is_empty() {
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &name_str, "vpn.secrets", &format!("password={}", pw_str)])
                .output();
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", &name_str, "+vpn.data", "password-flags=0"])
                .output();
        }
        Ok(())
    } else {
        Err("Failed to create VPN connection via nmcli".to_string())
    }
}

pub fn import_vpn_profile(path: &str) -> bool {
    let vpn_type = if path.ends_with(".ovpn") {
        "openvpn"
    } else if path.ends_with(".conf") {
        "wireguard"
    } else {
        "openvpn"
    };

    let imported = if let Ok(out) = Command::new("nmcli")
        .args(&["connection", "import", "type", vpn_type, "file", path])
        .output()
    {
        out.status.success()
    } else if let Ok(out) = Command::new("nmcli")
        .args(&["connection", "import", "file", path])
        .output()
    {
        out.status.success()
    } else {
        false
    };

    if imported && vpn_type == "openvpn" {
        if let Some(filename) = std::path::Path::new(path).file_stem().and_then(|s| s.to_str()) {
            let _ = Command::new("nmcli")
                .args(&["connection", "modify", filename, "+vpn.data", "password-flags=0"])
                .output();
        }
    }

    if imported {
        return true;
    }

    let filename = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported VPN");

    let details = VpnConnDetails {
        name: filename.to_string(),
        original_name: None,
        vpn_type: vpn_type.to_string(),
        gateway: String::new(),
        username: String::new(),
        password: String::new(),
        ca_cert: String::new(),
        config_file: Some(path.to_string()),
    };
    save_vpn_connection(&details).is_ok()
}

pub fn get_active_vpn_fast() -> Option<VpnConn> {
    get_vpn_connections().into_iter().find(|v| v.active)
}

pub fn get_vpn_logs(name: &str, since: Option<&str>) -> String {
    let mut args = vec!["-u", "NetworkManager", "-n", "100", "--no-pager"];
    if let Some(since_ts) = since {
        if !since_ts.is_empty() {
            args.push("--since");
            args.push(since_ts);
        }
    }

    if let Ok(output) = Command::new("journalctl").args(&args).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut formatted: Vec<String> = stdout
            .lines()
            .filter(|line| {
                line.contains(name)
                    || line.contains("vpn")
                    || line.contains("nm-openvpn")
                    || line.contains("WireGuard")
                    || line.contains("VPN")
            })
            .filter_map(clean_vpn_log_line)
            .collect();

        if !formatted.is_empty() {
            if formatted.len() > 60 {
                formatted = formatted.split_off(formatted.len() - 60);
            }
            return formatted.join("\n");
        }
    }

    "No connection logs found for NetworkManager.".to_string()
}

fn clean_vpn_log_line(line: &str) -> Option<String> {
    let line_str = line.trim();
    if line_str.is_empty() {
        return None;
    }

    let time_str = if line_str.len() >= 15 && line_str.as_bytes()[6] == b':' {
        &line_str[7..15]
    } else {
        ""
    };

    let level = if line_str.contains("<warn>") || line_str.contains("warn") || line_str.contains("failed") {
        "[WARN] "
    } else if line_str.contains("<error>") || line_str.contains("error") || line_str.contains("Error") {
        "[ERROR]"
    } else if line_str.contains("<info>") {
        "[INFO] "
    } else {
        "[LOG]  "
    };

    let msg = if let Some(idx) = line_str.rfind("]: ") {
        &line_str[idx + 3..]
    } else if let Some(idx) = line_str.find("NetworkManager[") {
        if let Some(rel_idx) = line_str[idx..].find(": ") {
            &line_str[idx + rel_idx + 2..]
        } else {
            line_str
        }
    } else {
        line_str
    };

    let clean_msg = if let Some(idx) = msg.find("]: ") {
        &msg[idx + 3..]
    } else {
        msg
    };

    if time_str.is_empty() {
        Some(format!("{} {}", level, clean_msg))
    } else {
        Some(format!("{}  {} {}", time_str, level, clean_msg))
    }
}
