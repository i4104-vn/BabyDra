use crate::models::vpn::*;
use crate::services::utils::run_cmd_bool;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Writes `content` to a uniquely-named temp file and returns its path.
    /// A dedicated per-pid subdirectory keeps the file stem clean (the parsed
    /// profile name is derived from the file stem).
    fn write_temp_config(name: &str, content: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir()
            .join("babydra_test_vpn")
            .join(std::process::id().to_string());
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let path = dir.join(name);
        let mut file = std::fs::File::create(&path).expect("create temp file");
        file.write_all(content.as_bytes()).expect("write temp file");
        path
    }

    #[test]
    fn parses_openvpn_remote_and_ca() {
        let path = write_temp_config(
            "office.ovpn",
            "client\nremote vpn.example.com 1194\nca ca.crt\n# comment\n",
        );
        let details = parse_vpn_config_file(path.to_str().unwrap());
        let _ = std::fs::remove_file(&path);

        assert_eq!(details.name, "office");
        assert_eq!(details.vpn_type, "openvpn");
        assert_eq!(details.gateway, "vpn.example.com:1194");
        assert_eq!(details.ca_cert, "ca.crt");
        assert_eq!(details.config_file.as_deref(), Some(path.to_str().unwrap()));
    }

    #[test]
    fn parses_openvpn_remote_without_port() {
        let path = write_temp_config("simple.ovpn", "remote vpn.example.com\n");
        let details = parse_vpn_config_file(path.to_str().unwrap());
        let _ = std::fs::remove_file(&path);

        assert_eq!(details.gateway, "vpn.example.com");
    }

    #[test]
    fn parses_wireguard_endpoint() {
        let path = write_temp_config("wg0.conf", "[Interface]\nEndpoint = 203.0.113.1:51820\n");
        let details = parse_vpn_config_file(path.to_str().unwrap());
        let _ = std::fs::remove_file(&path);

        assert_eq!(details.name, "wg0");
        assert_eq!(details.vpn_type, "wireguard");
        assert_eq!(details.gateway, "203.0.113.1:51820");
    }

    #[test]
    fn detects_type_from_extension() {
        let ovpn = write_temp_config("a.ovpn", "");
        let details_ovpn = parse_vpn_config_file(ovpn.to_str().unwrap());
        let _ = std::fs::remove_file(&ovpn);
        assert_eq!(details_ovpn.vpn_type, "openvpn");

        let conf = write_temp_config("b.conf", "");
        let details_conf = parse_vpn_config_file(conf.to_str().unwrap());
        let _ = std::fs::remove_file(&conf);
        assert_eq!(details_conf.vpn_type, "wireguard");
    }

    #[test]
    fn missing_file_returns_defaults() {
        let details = parse_vpn_config_file("/nonexistent/path/profile.ovpn");
        assert_eq!(details.name, "profile");
        assert_eq!(details.vpn_type, "openvpn");
        assert!(details.gateway.is_empty());
        assert!(details.ca_cert.is_empty());
    }

    #[test]
    fn skips_comments_and_blank_lines() {
        let path = write_temp_config(
            "commented.ovpn",
            "# remote hidden.example.com\n\n; another\nremote real.example.com 443\n",
        );
        let details = parse_vpn_config_file(path.to_str().unwrap());
        let _ = std::fs::remove_file(&path);

        assert_eq!(details.gateway, "real.example.com:443");
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
