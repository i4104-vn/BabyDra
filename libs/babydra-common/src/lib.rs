//! Common helper utilities shared across BabyDra desktop environment components.
//! Exposes shared config, theme engines, animations, power controllers, window layouts, and i18n hooks.

pub mod system;
pub mod desktop;
pub mod notification;
pub mod i18n;

// Re-export core modules for backward compatibility
pub mod core {
    pub use crate::desktop::config;
    pub use crate::desktop::apps as desktop;
    pub use crate::system::power;
}

// Re-export helper submodules for backward compatibility
pub mod helper {
    pub use crate::notification::service as notification;
    pub use crate::desktop::window;
    pub use crate::system::wifi;
    pub use crate::system::volume;
    pub use crate::system::backlight;
    pub use crate::system::storage;
    pub use crate::system::clean;
}

pub mod models;

// Flat re-exports at root for backward compatibility
pub use desktop::config::{ThemeConfig, ShellConfig, get_babydra_config_dir};
pub use notification::island::{IslandState, update_island_state, clear_island_state, get_island_state_path};
pub use system::volume::AudioDevice;
pub use system::storage::DiskInfo;
pub use notification::service::{ActiveNotification, NotificationMsg};
pub use desktop::icon::get_logo_path;
pub use desktop::theme::{init_theme, apply_theme_class};
pub use desktop::apps::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp};
pub use system::power::{poweroff, reboot, suspend};
pub use desktop::window::{init_layer_window, setup_click_outside_dismiss};

pub use desktop::theme;
pub use desktop::animation;
pub use desktop::icon;
pub use notification::island;
pub use desktop::window;
