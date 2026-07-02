//! Dynamic system status "island" overlay widget.
//! Manages popup notifications, volume/brightness overlays, and media players.

pub mod player;
pub mod models;
pub mod widgets;
pub mod render;

pub use render::create_system_island;

