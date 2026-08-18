//! StatusNotifierItem system tray server daemon.
//! Implements DBuswatcher daemon specifications to allow client apps to register system tray items.

pub mod client;
pub mod dbus_menu;
pub mod watcher;

pub use crate::models::MenuItem;
pub use crate::models::TrayItem;
pub use client::{activate_item, activate_menu_item, fetch_menu_path, get_dbus_menu};
pub use watcher::{get_tray_items, spawn_watcher_service};
