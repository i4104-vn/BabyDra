//! WiFi access point scanning and discovery using nmcli.

use crate::models::wifi::WifiNetwork;
use std::collections::HashMap;
use std::process::Command;

/// Splits an nmcli-escaped field (e.g. SSID) into parts on unescaped `:`.
///
/// Escaped colons (`\:`) stay inside the current part; plain colons split.
pub fn parse_nmcli_escaped(line: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut escape = false;
    for c in line.chars() {
        if escape {
            current.push(c);
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == ':' {
            parts.push(current.clone());
            current.clear();
        } else {
            current.push(c);
        }
    }
    parts.push(current);
    parts
}

/// Known networks.
pub fn known_networks() -> Vec<String> {
    let mut ssids = Vec::new();
    if let Ok(output) = Command::new("nmcli")
        .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts = parse_nmcli_escaped(line);
            if parts.len() >= 2
                && (parts[1] == "802-11-wireless" || parts[1] == "wifi")
                && !parts[0].is_empty()
            {
                if let Ok(ssid_out) = Command::new("nmcli")
                    .args([
                        "-g",
                        "802-11-wireless.ssid",
                        "connection",
                        "show",
                        &parts[0],
                    ])
                    .output()
                {
                    let ssid = String::from_utf8_lossy(&ssid_out.stdout).trim().to_string();
                    if !ssid.is_empty() && !ssids.contains(&ssid) {
                        ssids.push(ssid);
                    }
                }
            }
        }
    }
    ssids
}

/// Scans for `networks`.
pub fn scan_networks() -> Vec<WifiNetwork> {
    let known = known_networks();
    let mut ap_map: HashMap<String, WifiNetwork> = HashMap::new();

    // Request rescan
    let _ = Command::new("nmcli")
        .args(&["device", "wifi", "rescan"])
        .output();

    if let Ok(output) = Command::new("nmcli")
        .args(&[
            "-t",
            "-f",
            "SSID,SECURITY,SIGNAL,ACTIVE",
            "device",
            "wifi",
            "list",
        ])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts = parse_nmcli_escaped(line);
            if parts.len() >= 4 {
                let ssid = parts[0].trim().to_string();
                if ssid.is_empty() {
                    continue;
                }

                let security = parts[1].trim().to_lowercase();
                let signal = parts[2].trim().parse::<u32>().unwrap_or(0);
                let is_connected = parts[3].trim() == "yes";

                let sec_str = if security.is_empty() {
                    "open".to_string()
                } else if security.contains("802.1x") {
                    "8021x".to_string()
                } else {
                    "psk".to_string()
                };

                let is_saved = known.contains(&ssid);

                ap_map
                    .entry(ssid.clone())
                    .and_modify(|net| {
                        if is_connected {
                            net.is_connected = true;
                        }
                        if signal > net.signal {
                            net.signal = signal;
                            net.strength = signal.to_string();
                            net.security = sec_str.clone();
                        }
                    })
                    .or_insert(WifiNetwork {
                        ssid,
                        security: sec_str,
                        strength: signal.to_string(),
                        is_connected,
                        is_saved,
                        signal,
                    });
            }
        }
    }

    let mut networks: Vec<WifiNetwork> = ap_map.into_values().collect();
    sort_networks(&mut networks);
    networks
}

/// Sorts WiFi networks by priority: connected first, then saved, then by signal strength (desc).
pub fn sort_networks(networks: &mut [WifiNetwork]) {
    networks.sort_by(|a, b| {
        if a.is_connected != b.is_connected {
            b.is_connected.cmp(&a.is_connected)
        } else if a.is_saved != b.is_saved {
            b.is_saved.cmp(&a.is_saved)
        } else {
            b.signal.cmp(&a.signal)
        }
    });
}
