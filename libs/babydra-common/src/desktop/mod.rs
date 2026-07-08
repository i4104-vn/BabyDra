//! UI shell and desktop helper interfaces namespace.

pub mod config;
pub mod theme;
pub mod icon;
pub mod animation;
pub mod window;
pub mod apps;
pub mod tray;

pub use config::{ThemeConfig, ShellConfig, get_babydra_config_dir};
pub use theme::{init_theme, apply_theme_class};
pub use icon::{get_logo_png, get_logo_path, get_icon, get_system_or_file_icon, get_icon_from_svg, get_icon_colored, is_dark_mode};
pub use window::{init_layer_window, setup_click_outside_dismiss, get_running_windows, get_active_window, focus_app};
pub use apps::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp, get_window_hash};

