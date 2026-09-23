pub mod clipboard;
pub mod events;
pub mod navigation;
pub mod shortcuts;

pub use clipboard::{create_clipboard_callbacks, ClipboardCallbacks};
pub use events::{
    setup_dbus_receiver, setup_file_watcher, setup_resize_handler, setup_shortcuts,
    setup_status_wiring, ShortcutCallbacks, StatusWiringArgs,
};
pub use navigation::{setup_navigation, NavigationSetupArgs};
pub use shortcuts::{parse_shortcut, setup_key_shortcuts, KeyShortcut};
