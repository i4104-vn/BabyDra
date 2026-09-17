//! Re-exports for recording controller actions.

pub mod area;
pub mod audio;
pub mod lifecycle;

pub use area::{open_recordings_dir_action, select_recording_area_action};
pub use audio::{toggle_audio_mute_action, toggle_mic_mute_action};
pub use lifecycle::{start_recording_action, stop_recording_action, toggle_pause_action};
