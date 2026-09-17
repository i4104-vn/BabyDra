//! Screen recording service using wf-recorder and slurp.

pub mod audio;
pub mod cmd;
pub mod geo;
pub mod path;
pub mod session;

pub use audio::{
    init_recording_audio, is_recording_audio_muted, is_recording_mic_muted, reset_recording_audio,
    set_recording_audio_muted, set_recording_mic_muted, toggle_recording_audio_mute,
    toggle_recording_mic_mute,
};
pub use cmd::build_wf_recorder_command;
pub use geo::{select_area_with_slurp, select_geometry_str_with_slurp};
pub use path::{get_new_recording_path, get_recordings_dir};
pub use session::{
    get_active_config, get_elapsed_secs, get_status, is_paused, is_recording, pause_recording,
    resume_recording, start_recording, stop_recording, toggle_pause, toggle_recording,
};

// Re-export models from core models
pub use crate::models::recording::{RecordingConfig, RecordingMode, RecordingStatus};
