pub mod render;
use gtk4::prelude::*;
use tokio::sync::mpsc;

pub fn strip_ansi_escapes(input: &str) -> String {
    let mut output = String::new();
    let mut in_escape = false;
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            in_escape = true;
            continue;
        }
        if in_escape {
            if c.is_alphabetic() {
                in_escape = false;
            }
            continue;
        }
        output.push(c);
    }
    output
}

pub fn get_wifi_state() -> (bool, String) {
    let dev_output = std::process::Command::new("iwctl")
        .args(&["device", "list"])
        .output();
    let is_powered = if let Ok(out) = dev_output {
        let stdout = strip_ansi_escapes(&String::from_utf8_lossy(&out.stdout));
        let mut powered = false;
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == "wlan0" {
                powered = parts[2] == "on";
                break;
            }
        }
        powered
    } else {
        false
    };

    if !is_powered {
        return (false, "Off".to_string());
    }

    let station_output = std::process::Command::new("iwctl")
        .args(&["station", "wlan0", "show"])
        .output();
    if let Ok(out) = station_output {
        let stdout = strip_ansi_escapes(&String::from_utf8_lossy(&out.stdout));
        let mut state = "Disconnected".to_string();
        let mut connected_network = None;

        for line in stdout.lines() {
            if line.contains("State") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    state = parts[parts.len() - 1].to_string();
                }
            } else if line.contains("Connected network") {
                if let Some(pos) = line.find("Connected network") {
                    let val = &line[pos + "Connected network".len()..];
                    let clean = val.trim().to_string();
                    if !clean.is_empty() {
                        connected_network = Some(clean);
                    }
                }
            }
        }

        if let Some(net) = connected_network {
            (true, net)
        } else if state == "connecting" {
            (true, "Connecting...".to_string())
        } else if state == "authenticating" {
            (true, "Authenticating...".to_string())
        } else {
            (true, "Disconnected".to_string())
        }
    } else {
        (true, "Disconnected".to_string())
    }
}

pub fn known_networks() -> Vec<String> {
    let mut ssids = Vec::new();
    let output = std::process::Command::new("iwctl")
        .args(&["known-networks", "list"])
        .output();
    if let Ok(out) = output {
        let stdout = strip_ansi_escapes(&String::from_utf8_lossy(&out.stdout));
        let mut start_parsing = false;
        for line in stdout.lines() {
            if line.contains("----") {
                start_parsing = true;
                continue;
            }
            if start_parsing {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some(idx) = parts.iter().position(|&x| x == "psk" || x == "open" || x == "8021x") {
                        let ssid = parts[0..idx].join(" ");
                        ssids.push(ssid);
                    }
                }
            }
        }
    }
    ssids
}

pub fn scan_networks() -> Vec<(String, String, String, bool)> {
    let _ = std::process::Command::new("iwctl")
        .args(&["station", "wlan0", "scan"])
        .output();
        
    std::thread::sleep(std::time::Duration::from_millis(150));

    let mut networks = Vec::new();
    let output = std::process::Command::new("iwctl")
        .args(&["station", "wlan0", "get-networks"])
        .output();

    if let Ok(out) = output {
        let stdout = strip_ansi_escapes(&String::from_utf8_lossy(&out.stdout));
        let mut start_parsing = false;
        for line in stdout.lines() {
            if line.contains("----") {
                start_parsing = true;
                continue;
            }
            if start_parsing {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let is_connected = line.starts_with("  >");
                
                let clean_line = if is_connected {
                    line.replacen('>', "", 1)
                } else {
                    line.to_string()
                };

                let parts: Vec<&str> = clean_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some(idx) = parts.iter().position(|&x| x == "psk" || x == "open" || x == "8021x") {
                        let ssid = parts[0..idx].join(" ");
                        let security = parts[idx].to_string();
                        let signal = if idx + 1 < parts.len() {
                            parts[idx + 1].to_string()
                        } else {
                            "****".to_string()
                        };
                        networks.push((ssid, security, signal, is_connected));
                    }
                }
            }
        }
    }
    networks
}

pub fn connect_wifi_async(
    ssid: &str,
    username: Option<String>,
    password: Option<String>,
    sub_label: gtk4::Label,
    left_btn: gtk4::Button,
    circle: gtk4::Box,
    icon_widget: gtk4::Image,
    popover: gtk4::Popover,
) {
    let (tx, mut rx) = mpsc::unbounded_channel::<bool>();

    let ssid_str = ssid.to_string();
    std::thread::spawn(move || {
        let mut cmd = std::process::Command::new("iwctl");
        
        let mut is_enterprise = false;
        if let Some(user) = username {
            cmd.arg("--username").arg(user);
            is_enterprise = true;
        }
        if let Some(pass) = password {
            if is_enterprise {
                cmd.arg("--password").arg(pass);
            } else {
                cmd.arg("--passphrase").arg(pass);
            }
        }
        
        cmd.args(&["station", "wlan0", "connect", &ssid_str]);
        
        let status = cmd.status();
        let success = status.map(|s| s.success()).unwrap_or(false);
        let _ = tx.send(success);
    });

    let sub_label_c = sub_label.clone();
    let left_btn_c = left_btn.clone();
    let circle_c = circle.clone();
    let icon_widget_c = icon_widget.clone();
    let popover_c = popover.clone();
    let ssid_str2 = ssid.to_string();

    glib::spawn_future_local(async move {
        if let Some(success) = rx.recv().await {
            if success {
                sub_label_c.set_text(&ssid_str2);
                left_btn_c.add_css_class("active");
                circle_c.add_css_class("active");
                let new_img = babydra_common::icon::get_icon_colored("wifi", 14, "#ffffff");
                if let Some(paintable) = new_img.paintable() {
                    icon_widget_c.set_paintable(Some(&paintable));
                }
                popover_c.popdown();
            } else {
                sub_label_c.set_text("Failed");
                popover_c.popdown();
            }
        }
    });
}
