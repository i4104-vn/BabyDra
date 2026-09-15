//! Keymap configuration and runtime pause state helpers.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn default_true() -> bool {
    true
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct SystemShortcutEntry {
    #[serde(default)]
    pub modifiers: String,
    #[serde(default)]
    pub key: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub system_shortcuts: HashMap<String, SystemShortcutEntry>,
    #[serde(default)]
    pub custom_shortcuts: HashMap<String, String>,
    #[serde(default)]
    pub shortcuts: HashMap<String, String>,
}

/// Resolves the keymap config path `~/.babydra/keymap.toml`.
pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let babydra_path = PathBuf::from(&home).join(".babydra").join("keymap.toml");
    if babydra_path.exists() {
        return babydra_path;
    }

    // Migration from legacy ~/.config/babydra/keymap.toml if present
    let legacy_path = PathBuf::from(&home)
        .join(".config")
        .join("babydra")
        .join("keymap.toml");
    if legacy_path.exists() {
        let parent = PathBuf::from(&home).join(".babydra");
        let _ = fs::create_dir_all(&parent);
        let _ = fs::copy(&legacy_path, &babydra_path);
        let _ = fs::remove_file(&legacy_path);
        return babydra_path;
    }

    babydra_path
}

/// Maximum age (seconds) of the pause flag before it is considered stale and
/// ignored — protects against a crashed client leaving shortcuts disabled.
pub const PAUSE_TIMEOUT_SECS: u64 = 300;

/// Resolves the pause flag path `~/.babydra/keymap.pause`.
pub fn get_pause_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home).join(".babydra").join("keymap.pause")
}

/// Suppresses global shortcut handling while a key-capture dialog is open.
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
        let _ = fs::remove_file(&path);
        return false;
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    now.saturating_sub(created) < PAUSE_TIMEOUT_SECS
}
