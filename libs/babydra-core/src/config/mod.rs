//! Configuration path resolvers and desktop shell models.

use std::path::PathBuf;

pub use crate::models::{ShellConfig, ThemeConfig};

pub mod settings;
pub mod variant;
pub use settings::{
    get_babydra_conf_path, load_babydra_config, load_explore_settings, save_babydra_config,
    save_explore_settings, BabyDraConfig, ExploreSettings, NotificationConfig, PowerConfig,
    WallpaperConfig,
};

/// Resolves the absolute directory path to the user's config folder: `~/.babydra/configs/`.
pub fn get_babydra_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".babydra")
        .join("configs")
}
