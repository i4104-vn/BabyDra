//! Notepad configuration model and persistence logic.

use serde::{Deserialize, Serialize};
use std::sync::{OnceLock, RwLock};

static NOTEPAD_CFG_CACHE: OnceLock<RwLock<Option<NotepadSettings>>> = OnceLock::new();

/// Notepad configuration options covering typography, editor behavior, and autosave.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NotepadSettings {
    // ── Typography & Appearance ─────────────────────────────
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,
    #[serde(default = "default_false")]
    pub word_wrap: bool,
    #[serde(default = "default_dark_theme")]
    pub dark_theme: String,
    #[serde(default = "default_light_theme")]
    pub light_theme: String,

    // ── Editor Behavior ──────────────────────────────────────
    #[serde(default = "default_tab_size")]
    pub tab_size: u32,
    #[serde(default = "default_true")]
    pub indent_with_spaces: bool,
    #[serde(default = "default_true")]
    pub auto_indent: bool,

    // ── Saving & Formatting ──────────────────────────────────
    #[serde(default = "default_false")]
    pub auto_save: bool,
    #[serde(default = "default_auto_save_delay")]
    pub auto_save_delay_seconds: u32,
    #[serde(default = "default_false")]
    pub trim_trailing_whitespace: bool,
    #[serde(default = "default_true")]
    pub insert_final_newline: bool,
}

fn default_font_family() -> String {
    "Monospace".to_string()
}

fn default_font_size() -> u32 {
    13
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_tab_size() -> u32 {
    4
}

fn default_auto_save_delay() -> u32 {
    5
}

fn default_dark_theme() -> String {
    "base16-ocean.dark".to_string()
}

fn default_light_theme() -> String {
    "InspiredGitHub".to_string()
}

impl Default for NotepadSettings {
    fn default() -> Self {
        Self {
            font_family: default_font_family(),
            font_size: default_font_size(),
            show_line_numbers: default_true(),
            word_wrap: default_false(),
            dark_theme: default_dark_theme(),
            light_theme: default_light_theme(),
            tab_size: default_tab_size(),
            indent_with_spaces: default_true(),
            auto_indent: default_true(),
            auto_save: default_false(),
            auto_save_delay_seconds: default_auto_save_delay(),
            trim_trailing_whitespace: default_false(),
            insert_final_newline: default_true(),
        }
    }
}

/// Loads notepad settings from disk (`~/.babydra/configs/notepad.json`), falling back to defaults.
fn load_notepad_cfg_from_disk() -> NotepadSettings {
    let path = crate::config::get_config_dir().join("notepad.json");
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<NotepadSettings>(&content) {
            return cfg;
        }
    }
    NotepadSettings::default()
}

/// Loads notepad settings (cached in memory).
pub fn load_notepad_cfg() -> NotepadSettings {
    let cache = NOTEPAD_CFG_CACHE.get_or_init(|| RwLock::new(None));
    if let Ok(guard) = cache.read() {
        if let Some(settings) = guard.as_ref() {
            return settings.clone();
        }
    }

    let settings = load_notepad_cfg_from_disk();
    if let Ok(mut guard) = cache.write() {
        *guard = Some(settings.clone());
    }
    settings
}

/// Persists notepad settings to disk (`~/.babydra/configs/notepad.json`) and updates in-memory cache.
pub fn save_notepad_cfg(settings: &NotepadSettings) {
    let cache = NOTEPAD_CFG_CACHE.get_or_init(|| RwLock::new(None));
    if let Ok(mut guard) = cache.write() {
        *guard = Some(settings.clone());
    }

    let path = crate::config::get_config_dir().join("notepad.json");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(path, json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notepad_settings_default() {
        let def = NotepadSettings::default();
        assert_eq!(def.font_family, "Monospace");
        assert_eq!(def.font_size, 13);
        assert!(def.show_line_numbers);
        assert!(!def.word_wrap);
        assert_eq!(def.tab_size, 4);
        assert!(def.indent_with_spaces);
        assert!(def.auto_indent);
        assert!(!def.auto_save);
        assert_eq!(def.auto_save_delay_seconds, 5);
        assert!(!def.trim_trailing_whitespace);
        assert!(def.insert_final_newline);
    }

    #[test]
    fn test_notepad_settings_serialization() {
        let def = NotepadSettings {
            font_size: 18,
            auto_save: true,
            font_family: "JetBrains Mono".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&def).expect("serialization must succeed");
        let parsed: NotepadSettings =
            serde_json::from_str(&json).expect("deserialization must succeed");

        assert_eq!(parsed.font_size, 18);
        assert!(parsed.auto_save);
        assert_eq!(parsed.font_family, "JetBrains Mono");
        assert_eq!(parsed, def);
    }
}
