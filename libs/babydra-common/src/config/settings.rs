use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExploreSettings {
    pub view_mode: String,       // "icons" | "list"
    pub preview_visible: bool,   // true | false
    pub show_hidden: bool,       // true | false
}

impl Default for ExploreSettings {
    fn default() -> Self {
        Self {
            view_mode: "icons".to_string(),
            preview_visible: true,
            show_hidden: false,
        }
    }
}

pub fn get_settings_path() -> PathBuf {
    super::get_babydra_config_dir().join("explore_settings.json")
}

pub fn load_explore_settings() -> ExploreSettings {
    let path = get_settings_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str::<ExploreSettings>(&content) {
                return settings;
            }
        }
    }
    ExploreSettings::default()
}

pub fn save_explore_settings(settings: &ExploreSettings) {
    let path = get_settings_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(content) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(path, content);
    }
}
