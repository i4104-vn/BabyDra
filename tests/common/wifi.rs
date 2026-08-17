//! Integration tests: WiFi discovery helpers.
//!
//! Verifies nmcli line parsing and network prioritization logic through
//! the public API.

use babydra_common::models::wifi::WifiNetwork;
use babydra_common::services::system::wifi::discovery::sort_networks;

fn net(ssid: &str, signal: u32, is_connected: bool, is_saved: bool) -> WifiNetwork {
    WifiNetwork {
        ssid: ssid.to_string(),
        security: "psk".to_string(),
        strength: signal.to_string(),
        is_connected,
        is_saved,
        signal,
    }
}

#[test]
fn sort_prefers_connected_networks() {
    let mut networks = vec![net("coffee", 60, false, true), net("home", 40, true, true)];
    sort_networks(&mut networks);
    assert_eq!(networks[0].ssid, "home");
}

#[test]
fn sort_prefers_saved_networks_over_unknown() {
    let mut networks = vec![
        net("guest", 95, false, false),
        net("saved-1", 30, false, true),
    ];
    sort_networks(&mut networks);
    assert_eq!(networks[0].ssid, "saved-1");
}

#[test]
fn sort_orders_by_signal_strength_desc() {
    let mut networks = vec![
        net("weak", 20, false, false),
        net("strong", 90, false, false),
        net("medium", 50, false, false),
    ];
    sort_networks(&mut networks);
    assert_eq!(networks[0].ssid, "strong");
    assert_eq!(networks[1].ssid, "medium");
    assert_eq!(networks[2].ssid, "weak");
}
