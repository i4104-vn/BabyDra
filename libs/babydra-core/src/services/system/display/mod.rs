//! Monitor and display configuration service.

pub mod apply;
pub mod config;
pub mod query;

pub use apply::apply_display_configs;
pub use config::{apply_saved_displays, save_displays};
pub use query::get_displays;
