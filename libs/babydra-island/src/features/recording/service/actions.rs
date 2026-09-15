//! User action dispatchers for the recording island feature.

use babydra_core::services::recording::{
    is_recording_audio_muted, is_recording_mic_muted, toggle_recording_audio_mute,
    toggle_recording_mic_mute,
};

/// Toggles recording audio mute (only for the video stream being recorded) and returns new state.
pub fn toggle_audio_mute() -> bool {
    toggle_recording_audio_mute()
}

/// Toggles recording microphone mute (only for the video stream being recorded) and returns new state.
pub fn toggle_mic_mute() -> bool {
    toggle_recording_mic_mute()
}

/// Returns whether the recorded video audio stream is muted.
pub fn is_audio_muted() -> bool {
    is_recording_audio_muted()
}

/// Returns whether the recorded video microphone stream is muted.
pub fn is_mic_muted() -> bool {
    is_recording_mic_muted()
}
