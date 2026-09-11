//! Extensible Dynamic Island manager.
//!
//! The island is an extensible notch capsule that displays exactly one *view* at a time.
//! Features register views either through the [`IslandFeature`] trait or
//! through the lightweight [`IslandView`] descriptor + [`IslandViewHandle`]
//! API. A single controller loop arbitrates between requested views by
//! priority (explicit overrides always win), animates transitions, and
//! dispatches hover, click, and scroll events.

pub mod controller;
pub mod manager;
pub mod models;
pub mod ui;

pub use models::view;

pub use controller::set_layer_keyboard_mode;
pub use manager::{
    default_island, dismiss_all_popovers, tick_default_island, Island,
};
pub use models::{
    IslandConfig, IslandCtx, IslandDisplay, IslandFeature, IslandView, IslandViewHandle,
};
pub use ui::IslandBuilder;
