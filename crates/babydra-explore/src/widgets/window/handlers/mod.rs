pub mod events;
pub mod navigation;

pub use events::{
    setup_dbus_receiver, setup_file_watcher, setup_key_shortcuts, setup_resize_handler,
};
pub use navigation::setup_navigation;
