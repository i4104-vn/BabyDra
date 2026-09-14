//! Row renderers and readers for system and custom shortcut entries.

pub mod custom;
pub mod system;

pub use custom::{create_custom_shortcut_row, read_combo, read_command, DATA_ROW_CSS_CLASS};
pub use system::{create_system_shortcut_row, read_system_row, SYSTEM_ROW_CSS_CLASS};
