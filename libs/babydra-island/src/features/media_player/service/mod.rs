//! Background services and format helpers for the media player feature.

pub mod art;
pub mod format;
pub mod poll;

pub use art::{load_album_art_from_bytes, spawn_art_receiver, ArtPayload};
pub use format::{format_time, get_player_icon_name};
pub use poll::spawn_playerctl_polling;
