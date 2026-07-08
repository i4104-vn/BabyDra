//! UI shell and desktop helper interfaces namespace.

pub mod config;
pub mod theme;
pub mod icon;
pub mod animation;
pub mod window;
pub mod apps;
pub mod tray;

pub use apps::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp};

