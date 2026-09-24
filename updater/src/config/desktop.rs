use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaConfig {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
}

impl Default for MetaConfig {
    fn default() -> Self {
        Self {
            name: "BabyDra".to_string(),
            version: "1.0.0".to_string(),
            description: "Modern Wayland Desktop Shell for Arch Linux".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GSettingsConfig {
    pub font_name: String,
    pub document_font_name: String,
    pub monospace_font_name: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub cursor_size: u32,
}

impl Default for GSettingsConfig {
    fn default() -> Self {
        Self {
            font_name: "Quicksand 12".to_string(),
            document_font_name: "Quicksand 12".to_string(),
            monospace_font_name: "CaskaydiaCove Nerd Font 11".to_string(),
            icon_theme: "We10X".to_string(),
            cursor_theme: "Twilight-cursors".to_string(),
            cursor_size: 24,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetdConfig {
    pub vt: u32,
    pub command: String,
    pub user: String,
    #[serde(default)]
    pub mask_vt_range: Vec<u32>,
}

impl Default for GreetdConfig {
    fn default() -> Self {
        Self {
            vt: 1,
            command: "sh -c 'clear 2>/dev/null; setterm -cursor off 2>/dev/null; exec cage -s -- /usr/bin/babydra-greeter'".to_string(),
            user: "greeter".to_string(),
            mask_vt_range: vec![2, 3, 4, 5, 6],
        }
    }
}
