//! Library for the application launcher module.
//! Exposes layouts, builders, and app search filtering utilities.

pub mod core;
pub mod models;
pub mod widgets;
pub mod render;

pub use render::build_launcher_ui;

