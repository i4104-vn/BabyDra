//! Common helper utilities shared across BabyDra desktop environment components.
//! Exposes shared config, theme engines, animations, power controllers, window layouts, and i18n hooks.

pub mod core;
pub mod theme;
pub mod animation;
pub mod icon;
pub mod island;
pub mod models;
pub mod window;
pub mod helper;

pub use models::{ThemeConfig, ShellConfig, IslandState, AudioDevice, DiskInfo, ActiveNotification, NotificationMsg};
pub use core::config::get_babydra_config_dir;
pub use icon::get_logo_path;
pub use theme::init_theme;
pub use theme::apply_theme_class;
pub use island::{update_island_state, clear_island_state, get_island_state_path};
pub use core::desktop::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp};
pub use core::desktop;
pub use core::power::{poweroff, reboot, suspend};
pub use window::{init_layer_window, setup_click_outside_dismiss};
pub use babydra_i18n as i18n;

