//! Global shortcut (babydra-keymap) configuration service.
//!
//! Shortcuts live in `~/.config/babydra/keymap.toml`.  The `babydra-keymap`
//! daemon hot-reloads the file whenever it changes, so saving from Settings
//! takes effect immediately.

use crate::error::CoreResult;
use crate::models::shortcut::Shortcut;
use std::fs;
use std::path::PathBuf;

/// Default shortcuts shipped on first run (mirrors the old labwc keybinds).
pub const DEFAULT_SHORTCUTS: &[(&str, &str, &str)] = &[
    ("A", "Tab", "~/.local/bin/babydra-switcher"),
    ("W", "q", "~/.local/bin/babydra-launcher"),
    ("W", "F12", "~/.local/bin/babydra-screenshot"),
    ("", "Print", "~/.local/bin/babydra-screenshot"),
    ("W", "l", "~/.local/bin/babydra-lock"),
];

/// Resolves the keymap config path `~/.config/babydra/keymap.toml`.
pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home)
        .join(".config")
        .join("babydra")
        .join("keymap.toml")
}

/// Maximum age (seconds) of the pause flag before it is considered stale and
/// ignored — protects against a crashed client leaving shortcuts disabled.
const PAUSE_TIMEOUT_SECS: u64 = 300;

/// Resolves the pause flag path `~/.cache/babydra/keymap.pause`.
fn get_pause_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home)
        .join(".cache")
        .join("babydra")
        .join("keymap.pause")
}

/// Suppresses global shortcut handling while a key-capture dialog is open.
///
/// The flag file stores its creation timestamp so a stale pause (client
/// crashed without resuming) expires after [`PAUSE_TIMEOUT_SECS`].
pub fn pause_shortcuts() {
    let path = get_pause_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let _ = fs::write(path, now.to_string());
}

/// Resumes global shortcut handling.
pub fn resume_shortcuts() {
    let _ = fs::remove_file(get_pause_path());
}

/// True when shortcuts are currently paused (flag exists and is fresh).
pub fn is_paused() -> bool {
    let path = get_pause_path();
    let Ok(raw) = fs::read_to_string(&path) else {
        return false;
    };
    let Ok(created) = raw.trim().parse::<u64>() else {
        // Unparseable flag — treat as stale and clear it.
        let _ = fs::remove_file(&path);
        return false;
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    now.saturating_sub(created) < PAUSE_TIMEOUT_SECS
}

#[derive(serde::Deserialize)]
struct ConfigFile {
    #[serde(default)]
    shortcuts: std::collections::HashMap<String, String>,
}

/// Retrieves the global shortcut list, creating a default config when missing.
pub fn get_shortcuts() -> Vec<Shortcut> {
    let path = get_config_path();

    if !path.exists() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let defaults: Vec<Shortcut> = DEFAULT_SHORTCUTS
            .iter()
            .enumerate()
            .map(|(i, (mods, key, cmd))| Shortcut {
                id: i + 1,
                modifiers: mods.to_string(),
                key: key.to_string(),
                command: cmd.to_string(),
            })
            .collect();
        let _ = save_shortcuts(&defaults);
        return defaults;
    }

    parse_config(&path)
}

/// Parses the config file into shortcuts; returns an empty list on errors.
fn parse_config(path: &PathBuf) -> Vec<Shortcut> {
    let Ok(raw) = fs::read_to_string(path) else {
        return Vec::new();
    };

    let Ok(file) = toml::from_str::<ConfigFile>(&raw) else {
        return Vec::new();
    };

    let mut shortcuts: Vec<Shortcut> = file
        .shortcuts
        .into_iter()
        .enumerate()
        .map(|(i, (combo, command))| {
            let mut parts = combo.split('-');
            let key = parts.next_back().unwrap_or_default().to_string();
            let modifiers = parts.collect::<Vec<_>>().join("-");
            Shortcut {
                id: i + 1,
                modifiers,
                key,
                command,
            }
        })
        .collect();
    shortcuts.sort_by_key(|a| a.combo());
    shortcuts
}

/// Saves the shortcut list to `keymap.toml` in `[shortcuts]` map form.
pub fn save_shortcuts(shortcuts: &[Shortcut]) -> CoreResult<()> {
    let path = get_config_path();

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut content = String::from(
        "# babydra-keymap — global shortcut configuration\n\
         # Combo format (same as labwc): modifiers then key, separated by '-'.\n\
         #   Modifiers: S = Shift, C = Ctrl, A = Alt, W = Super\n\
         # Changes are hot-reloaded by babydra-keymap.\n\n[shortcuts]\n",
    );
    for sc in shortcuts {
        content.push_str(&format!("\"{}\" = \"{}\"\n", sc.combo(), sc.command));
    }

    fs::write(path, content)?;
    Ok(())
}
