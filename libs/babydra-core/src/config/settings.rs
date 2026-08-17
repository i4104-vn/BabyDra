use crate::models::ThemeConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Gets path to `~/.babydra/babydra.conf`
pub fn get_babydra_conf_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".babydra")
        .join("babydra.conf")
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PowerConfig {
    #[serde(default = "default_power_profile")]
    pub profile: String,
    #[serde(default = "default_auto_saver_enabled")]
    pub auto_saver_enabled: bool,
    #[serde(default = "default_saver_threshold")]
    pub saver_threshold: u32,
    #[serde(default = "default_charge_limit")]
    pub charge_limit: u32,
}

fn default_power_profile() -> String {
    "balanced".to_string()
}
fn default_auto_saver_enabled() -> bool {
    true
}
fn default_saver_threshold() -> u32 {
    20
}
fn default_charge_limit() -> u32 {
    80
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            profile: default_power_profile(),
            auto_saver_enabled: default_auto_saver_enabled(),
            saver_threshold: default_saver_threshold(),
            charge_limit: default_charge_limit(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WallpaperConfig {
    #[serde(default)]
    pub current: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NotificationConfig {
    #[serde(default)]
    pub dnd: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomContextItem {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExploreSettings {
    pub view_mode: String,          // "icons" | "list"
    pub preview_visible: bool,      // true | false
    pub show_hidden: bool,          // true | false
    pub double_click_to_open: bool, // true | false
    pub permanent_delete: bool,     // true | false
    pub calculate_dir_size: bool,   // true | false
    pub custom_context_items: Vec<CustomContextItem>,
    #[serde(default)]
    pub keybinds: std::collections::HashMap<String, String>,
}

impl ExploreSettings {
    pub fn get_keybind(&self, action: &str) -> String {
        self.keybinds
            .get(action)
            .cloned()
            .unwrap_or_else(|| match action {
                "toggle_split" => "F3".to_string(),
                "toggle_preview" => "F4".to_string(),
                "toggle_hidden" => "Ctrl + H".to_string(),
                "cut" => "Ctrl + X".to_string(),
                "copy" => "Ctrl + C".to_string(),
                "paste" => "Ctrl + V".to_string(),
                "undo" => "Ctrl + Z".to_string(),
                _ => "".to_string(),
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DisplayMonitorSetting {
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_width")]
    pub resolution_width: u32,
    #[serde(default = "default_height")]
    pub resolution_height: u32,
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: f64,
    #[serde(default)]
    pub position_x: i32,
    #[serde(default)]
    pub position_y: i32,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_scale")]
    pub scale: f64,
}

fn default_width() -> u32 {
    1920
}
fn default_height() -> u32 {
    1080
}
fn default_refresh_rate() -> f64 {
    60.0
}
fn default_orientation() -> String {
    "normal".to_string()
}
fn default_enabled() -> bool {
    true
}
fn default_scale() -> f64 {
    1.0
}

impl Default for DisplayMonitorSetting {
    fn default() -> Self {
        Self {
            name: String::new(),
            resolution_width: default_width(),
            resolution_height: default_height(),
            refresh_rate: default_refresh_rate(),
            position_x: 0,
            position_y: 0,
            orientation: default_orientation(),
            enabled: default_enabled(),
            scale: default_scale(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DisplayConfig {
    #[serde(default)]
    pub monitors: Vec<DisplayMonitorSetting>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LockscreenConfig {
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BabyDraConfig {
    #[serde(default)]
    pub power: PowerConfig,
    #[serde(default)]
    pub explore: ExploreSettings,
    #[serde(default)]
    pub wallpaper: WallpaperConfig,
    #[serde(default)]
    pub notification: NotificationConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub lockscreen: LockscreenConfig,
    /// Theme package selection (id + dark preference).
    /// Empty `id` = engine default (`babydra-default`).
    #[serde(default)]
    pub theme: ThemeConfig,
}

static CONFIG_CACHE: std::sync::OnceLock<std::sync::RwLock<BabyDraConfig>> =
    std::sync::OnceLock::new();

fn load_from_disk() -> BabyDraConfig {
    let path = get_babydra_conf_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str::<BabyDraConfig>(&content) {
                return config;
            }
        }
    }

    let mut config = BabyDraConfig::default();

    // Migration logic from legacy standalone files if present

    if let Some(home) = dirs::home_dir() {
        let legacy_explore = super::get_babydra_config_dir().join("explore.json");
        if legacy_explore.exists() {
            if let Ok(content) = std::fs::read_to_string(&legacy_explore) {
                if let Ok(exp) = serde_json::from_str(&content) {
                    config.explore = exp;
                }
            }
        }

        let legacy_perf = home.join(".babydra/perf_profile");
        if legacy_perf.exists() {
            if let Ok(prof) = std::fs::read_to_string(&legacy_perf) {
                config.power.profile = prof.trim().to_string();
            }
        }

        let legacy_wp = home.join(".babydra/current_wallpaper");
        if legacy_wp.exists() {
            if let Ok(wp) = std::fs::read_to_string(&legacy_wp) {
                config.wallpaper.current = wp.trim().to_string();
            }
        }

        let legacy_dnd = home.join(".babydra/dnd");
        if legacy_dnd.exists() {
            config.notification.dnd = true;
        }
    }

    // Don't call save_babydra_config here to avoid deadlock, just write to disk
    let path = get_babydra_conf_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(content) = toml::to_string_pretty(&config) {
        let _ = std::fs::write(&path, content);
    }
    config
}

/// Loads `babydra config`.
pub fn load_babydra_config() -> BabyDraConfig {
    let cache = CONFIG_CACHE.get_or_init(|| std::sync::RwLock::new(load_from_disk()));
    cache.read().unwrap().clone()
}

/// Persists `babydra config`.
pub fn save_babydra_config(config: &BabyDraConfig) {
    if let Some(cache) = CONFIG_CACHE.get() {
        if let Ok(mut guard) = cache.write() {
            *guard = config.clone();
        }
    }

    let path = get_babydra_conf_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(content) = toml::to_string_pretty(config) {
        let _ = std::fs::write(&path, content);
    }
}

/// Invalidate config cache.
pub fn invalidate_config_cache() {
    if let Some(cache) = CONFIG_CACHE.get() {
        if let Ok(mut guard) = cache.write() {
            *guard = load_from_disk();
        }
    }
}

/// Loads `explore settings`.
pub fn load_explore_settings() -> ExploreSettings {
    load_babydra_config().explore
}

/// Persists `explore settings`.
pub fn save_explore_settings(settings: &ExploreSettings) {
    let mut config = load_babydra_config();
    config.explore = settings.clone();
    save_babydra_config(&config);
}
