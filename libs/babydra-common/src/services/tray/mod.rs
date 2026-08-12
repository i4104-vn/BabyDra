//! StatusNotifierItem system tray server daemon.
//! Implements DBuswatcher daemon specifications to allow client apps to register system tray items.

pub mod watcher;
pub mod client;
pub mod dbusmenu;

pub use watcher::{get_tray_items, spawn_watcher_service};
pub use client::{activate_item, get_dbus_menu, fetch_menu_path, activate_menu_item};
pub use crate::models::MenuItem;
pub use crate::models::TrayItem;
