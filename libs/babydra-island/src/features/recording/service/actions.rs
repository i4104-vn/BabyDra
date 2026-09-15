//! User action dispatchers for the recording island feature.

/// Toggles system audio mute and returns new state.
pub fn toggle_audio_mute() -> bool {
    let muted = babydra_core::services::system::volume::is_muted();
    babydra_core::services::system::volume::set_muted(!muted);
    !muted
}

/// Toggles microphone mute and returns new state.
pub fn toggle_mic_mute() -> bool {
    let muted = babydra_core::services::system::volume::is_microphone_muted();
    babydra_core::services::system::volume::set_microphone_muted(!muted);
    !muted
}
