//! Integration tests: display service and configuration.

use babydra_core::models::display::MonitorConfig;
use babydra_core::services::system::display::{
    apply_display_configs, apply_saved_displays, get_displays, save_displays,
};

#[test]
fn test_get_displays_returns_valid_list() {
    let monitors = get_displays();
    for mon in &monitors {
        assert!(!mon.name.is_empty(), "Monitor name should not be empty");
        assert!(mon.resolution_width > 0, "Width should be > 0");
        assert!(mon.resolution_height > 0, "Height should be > 0");
        assert!(mon.refresh_rate > 0.0, "Refresh rate should be > 0");
        assert!(!mon.available_resolutions.is_empty());
        assert!(!mon.available_rates.is_empty());
    }
}

#[test]
fn test_save_and_load_display_config() {
    let test_monitor = MonitorConfig {
        id: "0".to_string(),
        name: "TEST-DISPLAY-1".to_string(),
        description: "Test Display Device".to_string(),
        resolution_width: 2560,
        resolution_height: 1440,
        refresh_rate: 144.0,
        position_x: 0,
        position_y: 0,
        orientation: "normal".to_string(),
        mode: "extend".to_string(),
        mirror_of: None,
        enabled: true,
        available_resolutions: vec!["2560x1440".to_string(), "1920x1080".to_string()],
        available_rates: vec![144.0, 60.0],
    };

    let prev_conf = babydra_core::config::load_babydra_config();

    // Save test monitor
    let res = save_displays(&[test_monitor.clone()]);
    assert!(res.is_ok(), "save_displays should succeed");

    let loaded = babydra_core::config::load_babydra_config();
    assert_eq!(loaded.display.monitors.len(), 1);
    assert_eq!(loaded.display.monitors[0].name, "TEST-DISPLAY-1");
    assert_eq!(loaded.display.monitors[0].resolution_width, 2560);
    assert_eq!(loaded.display.monitors[0].resolution_height, 1440);
    assert_eq!(loaded.display.monitors[0].refresh_rate, 144.0);

    // Apply saved displays should not panic
    apply_saved_displays();

    // Restore previous configuration
    babydra_core::config::save_babydra_config(&prev_conf);
}

#[test]
fn test_apply_display_configs_empty_and_dummy() {
    // Empty list should safely succeed
    let res = apply_display_configs(&[]);
    assert!(res.is_ok());
}
