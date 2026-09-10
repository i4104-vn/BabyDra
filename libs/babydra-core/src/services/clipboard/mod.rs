//! Clipboard service module.

pub mod service;

pub use service::{
    copy_to_system_clipboard, get_entries, get_shortcut, is_clipboard_enabled, push_entry,
    spawn_clipboard_watcher, trigger_clipboard, ClipboardEntry, DBUS_TRIGGER_CMD,
};
