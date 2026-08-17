//! Integration tests: VPN config file parsing.
//!
//! Verifies that `.ovpn` (OpenVPN) and `.conf` (WireGuard) profile files
//! are parsed into structured connection details.

use babydra_common::services::system::vpn::config::parse_vpn_config_file;
use std::io::Write;

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
fn parses_openvpn_config() {
    let path = write_temp_config(
        "office.ovpn",
        "client\ndev tun\nremote vpn.company.com 1194\nca ca.crt\n",
    );
    let details = parse_vpn_config_file(path.to_str().unwrap());
    let _ = std::fs::remove_file(&path);

    assert_eq!(details.name, "office");
    assert_eq!(details.vpn_type, "openvpn");
    assert_eq!(details.gateway, "vpn.company.com:1194");
    assert_eq!(details.ca_cert, "ca.crt");
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
    let path = write_temp_config(
        "wg0.conf",
        "[Interface]\nPrivateKey = x\nEndpoint = 203.0.113.1:51820\n",
    );
    let details = parse_vpn_config_file(path.to_str().unwrap());
    let _ = std::fs::remove_file(&path);

    assert_eq!(details.name, "wg0");
    assert_eq!(details.vpn_type, "wireguard");
    assert_eq!(details.gateway, "203.0.113.1:51820");
}

#[test]
fn type_detected_from_extension() {
    let ovpn = write_temp_config("any.ovpn", "");
    let conf = write_temp_config("any.conf", "");

    let details_ovpn = parse_vpn_config_file(ovpn.to_str().unwrap());
    let details_conf = parse_vpn_config_file(conf.to_str().unwrap());

    let _ = std::fs::remove_file(&ovpn);
    let _ = std::fs::remove_file(&conf);

    assert_eq!(details_ovpn.vpn_type, "openvpn");
    assert_eq!(details_conf.vpn_type, "wireguard");
}

#[test]
fn ignores_comments_and_blank_lines() {
    let path = write_temp_config(
        "commented.ovpn",
        "# remote hidden.example.com\n\n; another comment\nremote visible.example.com 443\n",
    );
    let details = parse_vpn_config_file(path.to_str().unwrap());
    let _ = std::fs::remove_file(&path);

    assert_eq!(details.gateway, "visible.example.com:443");
}

#[test]
fn missing_file_returns_partial_defaults() {
    let details = parse_vpn_config_file("/nonexistent/profile.ovpn");
    assert_eq!(details.name, "profile");
    assert_eq!(details.vpn_type, "openvpn");
    assert!(details.gateway.is_empty());
    assert!(details.ca_cert.is_empty());
}
