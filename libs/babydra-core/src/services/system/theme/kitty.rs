//! Kitty terminal theme synchronization service (Catppuccin).

use crate::error::CoreResult;
use crate::services::utils::{get_home_dir, run_cmd, run_cmd_bool};
use std::fs;
use std::path::Path;

const KITTY_THEME_DARK: &str = include_asset!("kitty/themes/dark");
const KITTY_THEME_LIGHT: &str = include_asset!("kitty/themes/light");

/// Checks whether Kitty terminal is installed on the system.
pub fn is_kitty_available() -> bool {
    run_cmd_bool(&["which", "kitty"])
}

/// Synchronizes the Kitty terminal theme with the active dark/light mode.
///
/// If Kitty is not installed, this exits early without modifying configuration.
/// When Kitty is installed, writes the theme to `$HOME/.config/kitty/theme.conf`,
/// ensures `kitty.conf` includes `theme.conf`, and reloads running Kitty instances.
pub fn sync_kitty_theme(dark: bool) -> CoreResult<()> {
    if !is_kitty_available() {
        return Ok(());
    }

    let home = get_home_dir();
    let config_dir = Path::new(&home).join(".config/kitty");
    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    // 1. Write theme override
    let theme_path = config_dir.join("theme.conf");
    let content = if dark {
        KITTY_THEME_DARK
    } else {
        KITTY_THEME_LIGHT
    };
    fs::write(&theme_path, content)?;

    // 2. Ensure kitty.conf includes theme.conf
    append_if_missing(&config_dir.join("kitty.conf"), "include theme.conf");

    // 3. Reload running Kitty instances if any
    let _ = run_cmd(&["pkill", "-SIGUSR1", "-x", "kitty"]);

    Ok(())
}

/// Appends a configuration line to a file if not already present.
fn append_if_missing(file: &Path, line: &str) {
    if file.exists() {
        if let Ok(content) = fs::read_to_string(file) {
            if content.contains(line) {
                return;
            }
            let prefix = if content.ends_with('\n') { "" } else { "\n" };
            let _ = fs::write(file, format!("{content}{prefix}{line}\n"));
        }
    } else {
        let _ = fs::write(file, format!("{line}\n"));
    }
}
