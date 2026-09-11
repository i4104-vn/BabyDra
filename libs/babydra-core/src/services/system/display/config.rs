//! Monitor configuration persistence in babydra.conf.

use crate::error::CoreResult;
use crate::models::display::MonitorConfig;
use crate::services::system::display::apply::apply_display_configs;

/// Saves monitor configurations into babydra.conf and applies changes via wlr-randr.
pub fn save_displays(monitors: &[MonitorConfig]) -> CoreResult<()> {
    let mut conf = crate::config::load_babydra_config();
    conf.display.monitors = monitors
        .iter()
        .map(|m| crate::config::settings::DisplayMonitorSetting {
            name: m.name.clone(),
            resolution_width: m.resolution_width,
            resolution_height: m.resolution_height,
            refresh_rate: m.refresh_rate,
            position_x: m.position_x,
            position_y: m.position_y,
            orientation: m.orientation.clone(),
            enabled: m.enabled,
            scale: 1.0,
        })
        .collect();
    crate::config::save_babydra_config(&conf);

    apply_display_configs(monitors)
}

/// Reads saved monitor configurations from babydra.conf and applies them via wlr-randr.
pub fn apply_saved_displays() {
    let conf = crate::config::load_babydra_config();
    if conf.display.monitors.is_empty() {
        return;
    }

    let monitors: Vec<MonitorConfig> = conf
        .display
        .monitors
        .into_iter()
        .map(|m| MonitorConfig {
            id: m.name.clone(),
            name: m.name,
            description: "Display".to_string(),
            resolution_width: m.resolution_width,
            resolution_height: m.resolution_height,
            refresh_rate: m.refresh_rate,
            position_x: m.position_x,
            position_y: m.position_y,
            orientation: m.orientation,
            mode: "extend".to_string(),
            mirror_of: None,
            enabled: m.enabled,
            available_resolutions: Vec::new(),
            available_rates: Vec::new(),
        })
        .collect();

    let _ = apply_display_configs(&monitors);
}
