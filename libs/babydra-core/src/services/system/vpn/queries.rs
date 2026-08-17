use crate::models::vpn::*;
use crate::services::utils::run_cmd;

/// Returns `true` when `VPN type` holds, `false` otherwise.
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

/// Returns the current `VPN connections`.
pub fn get_vpn_connections() -> Vec<VpnConn> {
    let mut connections = Vec::new();
    let mut active_info = std::collections::HashMap::<String, String>::new();
    if let Some(act_out) = run_cmd(&[
        "nmcli",
        "-t",
        "-f",
        "NAME,TYPE,DEVICE",
        "connection",
        "show",
        "--active",
    ]) {
        for line in act_out.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 && is_vpn_type(parts[1]) {
                active_info.insert(parts[0].to_string(), parts[2].to_string());
            }
        }
    }

    if let Some(stdout) = run_cmd(&["nmcli", "-t", "-f", "NAME,TYPE,UUID", "connection", "show"]) {
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && is_vpn_type(parts[1]) {
                let name = parts[0].to_string();
                let conn_type = parts[1].to_string();
                let active_iface = active_info.get(&name).cloned();
                let active = active_iface.is_some();
                let dev_iface = active_iface.unwrap_or_default();

                let mut gateway = String::new();
                let mut username = String::new();
                let mut ip_address = String::new();
                let mut remote_server = String::new();
                let mut cipher = String::new();

                if let Some(details_str) = run_cmd(&[
                    "nmcli",
                    "-t",
                    "-f",
                    "IP4.ADDRESS,IP4.GATEWAY,vpn.user-name,vpn.data",
                    "connection",
                    "show",
                    &name,
                ]) {
                    for dline in details_str.lines() {
                        if dline.starts_with("vpn.user-name:") {
                            username = dline.trim_start_matches("vpn.user-name:").to_string();
                        } else if dline.starts_with("IP4.ADDRESS") {
                            if let Some((_, val)) = dline.split_once(':') {
                                ip_address = val.to_string();
                            }
                        } else if dline.starts_with("IP4.GATEWAY:") {
                            gateway = dline.trim_start_matches("IP4.GATEWAY:").to_string();
                        } else if dline.starts_with("vpn.data:") {
                            let data_str = dline.trim_start_matches("vpn.data:");
                            for item in data_str.split(',') {
                                let kv: Vec<&str> = item.split('=').map(|s| s.trim()).collect();
                                if kv.len() == 2 {
                                    match kv[0] {
                                        "remote" => remote_server = kv[1].to_string(),
                                        "gateway" => {
                                            if gateway.is_empty() {
                                                gateway = kv[1].to_string();
                                            }
                                        }
                                        "cipher" => cipher = kv[1].to_string(),
                                        _ => {}
                                    }
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
                    ip_address,
                    remote_server,
                    dev_iface,
                    cipher,
                });
            }
        }
    }

    connections
}

/// Returns the current `VPN details`.
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

    if let Some(stdout) = run_cmd(&["nmcli", "--show-secrets", "-t", "connection", "show", name]) {
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
                                    "remote" | "gateway" => {
                                        details.gateway = d_val.trim().to_string()
                                    }
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

/// Returns `true` when `VPN active fast` holds, `false` otherwise.
pub fn is_vpn_active_fast() -> bool {
    if let Some(act_out) = run_cmd(&[
        "nmcli",
        "-t",
        "-f",
        "TYPE",
        "connection",
        "show",
        "--active",
    ]) {
        for line in act_out.lines() {
            if is_vpn_type(line.trim()) {
                return true;
            }
        }
    }
    false
}

/// Returns the current `active VPN fast`.
pub fn get_active_vpn_fast() -> Option<VpnConn> {
    get_vpn_connections().into_iter().find(|v| v.active)
}

/// Returns the current `VPN logs`.
pub fn get_vpn_logs(name: &str, since: Option<&str>) -> String {
    let mut args = vec![
        "journalctl",
        "-u",
        "NetworkManager",
        "-n",
        "100",
        "--no-pager",
    ];
    if let Some(since_ts) = since {
        if !since_ts.is_empty() {
            args.push("--since");
            args.push(since_ts);
        }
    }

    if let Some(stdout) = run_cmd(&args) {
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

    let level = if line_str.contains("<warn>")
        || line_str.contains("warn")
        || line_str.contains("failed")
    {
        "[WARN] "
    } else if line_str.contains("<error>")
        || line_str.contains("error")
        || line_str.contains("Error")
    {
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
