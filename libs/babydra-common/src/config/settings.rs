use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomContextItem {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExploreSettings {
    pub view_mode: String,       // "icons" | "list"
    pub preview_visible: bool,   // true | false
    pub show_hidden: bool,       // true | false
    pub double_click_to_open: bool, // true | false
    pub permanent_delete: bool,   // true | false
    pub calculate_dir_size: bool, // true | false
    pub custom_context_items: Vec<CustomContextItem>,
    #[serde(default)]
    pub keybinds: std::collections::HashMap<String, String>,
}

impl ExploreSettings {
    pub fn get_keybind(&self, action: &str) -> String {
        self.keybinds.get(action).cloned().unwrap_or_else(|| {
            match action {
                "toggle_split" => "F3".to_string(),
                "toggle_preview" => "F4".to_string(),
                "toggle_hidden" => "Ctrl + H".to_string(),
                "cut" => "Ctrl + X".to_string(),
                "copy" => "Ctrl + C".to_string(),
                "paste" => "Ctrl + V".to_string(),
                "undo" => "Ctrl + Z".to_string(),
                _ => "".to_string(),
            }
        })
    }
}

impl Default for ExploreSettings {
    fn default() -> Self {
        Self {
            view_mode: "icons".to_string(),
            preview_visible: true,
            show_hidden: false,
            double_click_to_open: true,
            permanent_delete: false,
            calculate_dir_size: true,
            custom_context_items: Vec::new(),
            keybinds: std::collections::HashMap::new(),
        }
    }
}

pub fn load_explore_settings() -> ExploreSettings {
    let dir = super::get_babydra_config_dir();
    let path = dir.join("explore.json");
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(settings) = serde_json::from_str(&content) {
            return settings;
        }
    }
    ExploreSettings::default()
}

pub fn save_explore_settings(settings: &ExploreSettings) {
    let dir = super::get_babydra_config_dir();
    let path = dir.join("explore.json");

    #[cfg(unix)]
    {
        if dir.exists() {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
        }
        if path.exists() {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
    }

    let _ = std::fs::create_dir_all(&dir);

    if let Ok(content) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(&path, content);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400));
        let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o500));
    }
}
