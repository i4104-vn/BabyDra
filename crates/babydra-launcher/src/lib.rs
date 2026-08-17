//! Library for the application launcher module.
//! Exposes layouts, builders, and app search filtering utilities.

pub mod render;
pub mod results;
pub mod widgets;

pub use render::build_launcher_ui;
pub use results::{repopulate_results, update_highlight};
