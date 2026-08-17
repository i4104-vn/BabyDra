//! Dynamic system status "island" overlay widget.
//!
//! The island is an extensible notch capsule that displays exactly one view at
//! a time. Register views either through the [`island::IslandFeature`] trait
//! (stateful features) or the [`island::IslandView`] descriptor +
//! [`island::IslandViewHandle`] API, and control them with `show` / `hide` /
//! `override_show_for`.
//!
//! Built-in features: media player (playerctl + visualizer + control popover),
//! desktop notifications, and an optional idle logo pill.

pub mod features;
pub mod island;
pub mod models;
pub mod render;
pub mod widgets;

pub use island::{
    default_island, Island, IslandBuilder, IslandConfig, IslandFeature, IslandView,
    IslandViewHandle,
};
pub use render::{build_default_island, create_system_island};
