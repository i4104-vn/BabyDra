//! Integration tests: configuration (config module).
//!
//! These exercise the public config API end-to-end, including TOML
//! round-trips and backward-compatible defaults for legacy config files.

use babydra_core::config::settings::CustomContextItem;
use babydra_core::config::{BabyDraConfig, ExploreSettings, PowerConfig};

#[test]
fn default_config_has_sane_values() {
    let config = BabyDraConfig::default();
    assert_eq!(config.power.profile, "balanced");
    assert_eq!(config.power.saver_threshold, 20);
    assert_eq!(config.power.charge_limit, 80);
    assert!(config.explore.preview_visible);
    assert_eq!(config.explore.view_mode, "icons");
}

#[test]
fn explore_settings_default_keybinds_are_available() {
    let settings = ExploreSettings::default();
    assert_eq!(settings.get_keybind("toggle_split"), "F3");
    assert_eq!(settings.get_keybind("toggle_preview"), "F4");
    assert_eq!(settings.get_keybind("cut"), "Ctrl + X");
    assert_eq!(settings.get_keybind("undo"), "Ctrl + Z");
}

#[test]
fn legacy_config_without_new_fields_still_loads() {
    // An old `babydra.conf` that predates newer sections must deserialize
    // with defaults instead of failing.
    let legacy = "[power]\nprofile = \"saver\"\n";
    let parsed: BabyDraConfig = toml::from_str(legacy).expect("legacy config must parse");
    assert_eq!(parsed.power.profile, "saver");
    assert_eq!(
        parsed.power.charge_limit, 80,
        "missing field falls back to default"
    );
    assert!(parsed.explore.show_hidden == false);
}

#[test]
fn power_config_defaults_apply() {
    let power = PowerConfig::default();
    assert_eq!(power.profile, "balanced");
    assert!(power.auto_saver_enabled);
}

#[test]
fn config_roundtrip_preserves_nested_sections() {
    let mut config = BabyDraConfig::default();
    config.notification.dnd = true;
    config.explore.view_mode = "list".to_string();
    config
        .explore
        .custom_context_items
        .push(babydra_core::config::settings::CustomContextItem {
            name: "Open in terminal".to_string(),
            command: "kitty".to_string(),
            icon: Some("terminal".to_string()),
        });

    let encoded = toml::to_string(&config).expect("serialize");
    let decoded: BabyDraConfig = toml::from_str(&encoded).expect("deserialize");

    assert!(decoded.notification.dnd);
    assert_eq!(decoded.explore.view_mode, "list");
    assert_eq!(decoded.explore.custom_context_items.len(), 1);
    assert_eq!(
        decoded.explore.custom_context_items[0].name,
        "Open in terminal"
    );
}

#[test]
fn get_keybind_returns_custom_value_when_set() {
    let mut settings = ExploreSettings::default();
    settings
        .keybinds
        .insert("toggle_split".to_string(), "F2".to_string());
    assert_eq!(settings.get_keybind("toggle_split"), "F2");
}

#[test]
fn get_keybind_returns_empty_for_unknown_action() {
    let settings = ExploreSettings::default();
    assert_eq!(settings.get_keybind("no_such_action"), "");
}

#[test]
fn explore_settings_serde_roundtrip() {
    let mut original = ExploreSettings::default();
    original.view_mode = "list".to_string();
    original.custom_context_items.push(CustomContextItem {
        name: "Open terminal here".to_string(),
        command: "kitty".to_string(),
        icon: Some("terminal".to_string()),
    });

    let encoded = serde_json::to_string(&original).unwrap();
    let decoded: ExploreSettings = serde_json::from_str(&encoded).unwrap();

    assert_eq!(decoded.view_mode, "list");
    assert_eq!(decoded.custom_context_items.len(), 1);
    assert_eq!(decoded.custom_context_items[0].name, "Open terminal here");
}

#[test]
fn babydra_config_toml_roundtrip() {
    let mut original = BabyDraConfig::default();
    original.power.profile = "performance".to_string();
    original.notification.dnd = true;

    let encoded = toml::to_string(&original).unwrap();
    let decoded: BabyDraConfig = toml::from_str(&encoded).unwrap();

    assert_eq!(decoded.power.profile, "performance");
    assert!(decoded.notification.dnd);
}

#[test]
fn display_config_toml_roundtrip() {
    use babydra_core::config::settings::DisplayMonitorSetting;
    let mut original = BabyDraConfig::default();
    original.display.monitors.push(DisplayMonitorSetting {
        name: "HDMI-A-1".to_string(),
        resolution_width: 1920,
        resolution_height: 1080,
        refresh_rate: 100.0,
        position_x: 0,
        position_y: 0,
        orientation: "normal".to_string(),
        enabled: true,
        scale: 1.0,
    });

    let encoded_pretty = toml::to_string_pretty(&original).expect("serialize pretty");
    println!("ENCODED PRETTY:\n{}", encoded_pretty);

    let decoded: BabyDraConfig = toml::from_str(&encoded_pretty).expect("deserialize pretty");
    assert_eq!(decoded.display.monitors.len(), 1);
    assert_eq!(decoded.display.monitors[0].name, "HDMI-A-1");
    assert_eq!(decoded.display.monitors[0].resolution_width, 1920);
    assert_eq!(decoded.display.monitors[0].resolution_height, 1080);
    assert_eq!(decoded.display.monitors[0].refresh_rate, 100.0);
}

