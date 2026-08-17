//! Dynamic system status "island" overlay widget.
//! Manages popup notifications, volume/brightness overlays, and media players.

pub mod models;
pub mod player;
pub mod render;
pub mod widgets;

pub use render::create_system_island;
