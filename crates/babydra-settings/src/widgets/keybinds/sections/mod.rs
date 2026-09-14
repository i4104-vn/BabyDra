//! UI sections of the keybindings settings page.

pub mod custom;
pub mod header;
pub mod system;

pub use custom::build_custom_section;
pub use header::build_page_header;
pub use system::build_system_section;
