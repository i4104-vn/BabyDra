//! Context menu dispatcher for desktop background and file entries.

pub mod empty_menu;
pub mod file_menu;

pub use empty_menu::show_desktop_empty_menu;
pub use file_menu::show_desktop_file_menu;
