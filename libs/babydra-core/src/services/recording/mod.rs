//! Screen recording service using wf-recorder and slurp.

pub mod capture;
pub mod path;
pub mod slurp;

pub use capture::{
    get_active_config, get_elapsed_secs, get_status, is_paused, is_recording, pause_recording,
    resume_recording, start_recording, stop_recording, toggle_pause, toggle_recording,
};
pub use path::{get_new_recording_path, get_recordings_dir};
pub use slurp::{select_area_with_slurp, select_geometry_str_with_slurp};

// Re-export models from core models
pub use crate::models::recording::{RecordingConfig, RecordingMode, RecordingStatus};
