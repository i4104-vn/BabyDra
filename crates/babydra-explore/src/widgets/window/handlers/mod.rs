pub mod events;
pub mod navigation;

pub use events::{setup_key_shortcuts, setup_window_resize_handler, setup_file_watcher_receiver, setup_dbus_receiver};
pub use navigation::setup_navigation;
