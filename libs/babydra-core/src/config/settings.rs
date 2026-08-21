//! Configuration load/save logic for `~/.babydra/babydra.conf`.
//!
//! Legacy bridge: data models now live in `models::config` and are re-exported
//! here only so existing `config::settings::*` imports keep working. Do NOT add
//! new struct definitions to this file.

pub use crate::models::config::{
    BabyDraConfig, CustomContextItem, DesktopConfig, DisplayConfig, DisplayMonitorSetting,
    ExploreSettings, LockscreenConfig, NotificationConfig, PowerConfig, WallpaperConfig,
};
use std::path::PathBuf;

/// Gets path to `~/.babydra/babydra.conf`
pub fn get_conf_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".babydra")
        .join("babydra.conf")
}

static CONFIG_CACHE: std::sync::OnceLock<std::sync::RwLock<BabyDraConfig>> =
    std::sync::OnceLock::new();

fn load_from_disk() -> BabyDraConfig {
    let path = get_conf_path();
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
    let path = get_conf_path();
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

    let path = get_conf_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(content) = toml::to_string_pretty(config) {
        let _ = std::fs::write(&path, content);
    }
}

/// Invalidate config cache.
pub fn invalidate_cache() {
    if let Some(cache) = CONFIG_CACHE.get() {
        if let Ok(mut guard) = cache.write() {
            *guard = load_from_disk();
        }
    }
}

/// Loads `explore settings` from `~/.babydra/configs/explore.json`.
pub fn load_explore_cfg() -> ExploreSettings {
    let path = crate::config::get_config_dir().join("explore.json");
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(mut settings) = serde_json::from_str::<ExploreSettings>(&content) {
            if settings.sidebar_items.is_empty() {
                settings.sidebar_items = crate::config::sidebar_layout::default_sidebar_items();
            }
            return settings;
        } else if let Ok(items) =
            serde_json::from_str::<Vec<crate::models::config::SidebarItem>>(&content)
        {
            let mut s = ExploreSettings::default();
            s.sidebar_items = items;
            return s;
        }
    }

    let mut s = ExploreSettings::default();
    s.sidebar_items = crate::config::sidebar_layout::default_sidebar_items();
    s
}

/// Persists `explore settings` directly to `~/.babydra/configs/explore.json`.
pub fn save_explore_cfg(settings: &ExploreSettings) {
    let path = crate::config::get_config_dir().join("explore.json");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(path, json);
    }
}

/// Loads `desktop config`.
pub fn load_desktop_config() -> DesktopConfig {
    load_babydra_config().desktop
}

/// Persists `desktop config`.
pub fn save_desktop_config(desktop: &DesktopConfig) {
    let mut config = load_babydra_config();
    config.desktop = desktop.clone();
    save_babydra_config(&config);
}
