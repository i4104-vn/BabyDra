use crate::models::vpn::*;
use crate::services::utils::run_cmd_bool;

/// Copy VPN config to babydra dir.
pub fn copy_vpn_config_to_babydra_dir(src_path: &str) -> Result<String, String> {
    let vpn_dir = crate::config::get_babydra_config_dir().join("vpn");
    std::fs::create_dir_all(&vpn_dir).map_err(|e| format!("Failed to create config dir: {}", e))?;

    let filename = std::path::Path::new(src_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("vpn_profile.conf");

    let dest_path = vpn_dir.join(filename);
    std::fs::copy(src_path, &dest_path)
        .map_err(|e| format!("Failed to copy config file: {}", e))?;

    Ok(dest_path.to_string_lossy().to_string())
}

/// Parses `VPN config file`.
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

/// Import VPN profile.
pub fn import_vpn_profile(path: &str) -> bool {
    let vpn_type = if path.ends_with(".ovpn") {
        "openvpn"
    } else if path.ends_with(".conf") {
        "wireguard"
    } else {
        "openvpn"
    };

    let imported = if run_cmd_bool(&[
        "nmcli",
        "connection",
        "import",
        "type",
        vpn_type,
        "file",
        path,
    ]) {
        true
    } else if run_cmd_bool(&["nmcli", "connection", "import", "file", path]) {
        true
    } else {
        false
    };

    if imported && vpn_type == "openvpn" {
        if let Some(filename) = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                filename,
                "+vpn.data",
                "password-flags=0",
            ]);
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
    crate::services::system::vpn::actions::save_vpn_connection(&details).is_ok()
}
