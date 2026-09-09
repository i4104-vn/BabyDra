//! Labwc titlebar theme synchronization service.

use crate::error::CoreResult;
use crate::services::utils::{get_home_dir, run_cmd};
use std::fs;
use std::path::Path;

const LABWC_THEME_DARK: &str = include_asset!("labwc/themes/dark");
const LABWC_THEME_LIGHT: &str = include_asset!("labwc/themes/light");

/// Synchronizes the labwc window titlebar themerc-override with the active dark/light mode.
///
/// Writes the embedded themerc configuration to `$HOME/.config/labwc/themerc-override`
/// and executes `labwc --reconfigure` to apply it immediately to all window titlebars.
pub fn sync_labwc_titlebar_theme(dark: bool) -> CoreResult<()> {
    let home = get_home_dir();
    let config_dir = Path::new(&home).join(".config/labwc");
    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    let override_path = config_dir.join("themerc-override");
    let content = if dark {
        LABWC_THEME_DARK
    } else {
        LABWC_THEME_LIGHT
    };

    fs::write(&override_path, content)?;

    // Tell labwc compositor to reload theme if running
    let _ = run_cmd(&["labwc", "--reconfigure"]);

    Ok(())
}
