//! Session state tracking and lifecycle coordination.

pub mod manager;
pub mod state;

pub use manager::{
    pause_recording, resume_recording, start_recording, stop_recording, toggle_pause,
    toggle_recording,
};
pub use state::{get_active_config, get_elapsed_secs, get_status, is_paused, is_recording};
