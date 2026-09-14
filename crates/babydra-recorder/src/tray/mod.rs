//! System tray StatusNotifierItem integration.

pub mod service;

pub use service::{register_with_watcher, StatusNotifierItemService};
