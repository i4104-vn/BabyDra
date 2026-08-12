//! Configuration path resolvers and desktop shell models.

use std::path::PathBuf;

pub use crate::models::{ThemeConfig, ShellConfig};

pub mod settings;
pub use settings::{
    BabyDraConfig, PowerConfig, WallpaperConfig, NotificationConfig, ExploreSettings,
    load_babydra_config, save_babydra_config, get_babydra_conf_path,
    load_explore_settings, save_explore_settings,
};

/// Resolves the absolute directory path to the user's config folder: `~/.babydra/configs/`.
pub fn get_babydra_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".babydra")
        .join("configs")
}

