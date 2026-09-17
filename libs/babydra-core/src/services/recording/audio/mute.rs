//! Audio and microphone mute controls for screen recording sessions.

use std::sync::atomic::Ordering;
use super::pipewire::{
    get_mic_output_ports, get_monitor_output_ports, get_recorder_input_ports, link_channel_pairs,
    pactl_set_recorder_mute, ACTIVE_PID, AUDIO_ENABLED, IS_AUDIO_MUTED, IS_MIC_MUTED,
};

/// Checks whether the recorded video audio stream is muted.
/// Returns the actual stored mute state regardless of whether audio is enabled.
pub fn is_recording_audio_muted() -> bool {
    IS_AUDIO_MUTED.load(Ordering::SeqCst)
}

/// Checks whether the recorded video microphone stream is muted.
/// Returns the actual stored mute state regardless of whether audio is enabled.
pub fn is_recording_mic_muted() -> bool {
    IS_MIC_MUTED.load(Ordering::SeqCst)
}

/// Sets whether system desktop audio is recorded in the current video.
/// The atomic mute state is always updated; PipeWire operations are only
/// attempted when the recording session has audio enabled.
pub fn set_recording_audio_muted(muted: bool) -> bool {
    if AUDIO_ENABLED.load(Ordering::SeqCst) {
        let inputs = get_recorder_input_ports();
        let monitors = get_monitor_output_ports();

        if !inputs.is_empty() && !monitors.is_empty() {
            link_channel_pairs(&monitors, &inputs, muted);
        } else {
            let pid = ACTIVE_PID.load(Ordering::SeqCst);
            if pid > 0 {
                pactl_set_recorder_mute(pid, muted);
            }
        }
    }

    IS_AUDIO_MUTED.store(muted, Ordering::SeqCst);
    muted
}

/// Sets whether microphone voice is recorded in the current video.
/// The atomic mute state is always updated; PipeWire operations are only
/// attempted when the recording session has audio enabled.
pub fn set_recording_mic_muted(muted: bool) -> bool {
    if AUDIO_ENABLED.load(Ordering::SeqCst) {
        let inputs = get_recorder_input_ports();
        let mics = get_mic_output_ports();

        if !inputs.is_empty() && !mics.is_empty() {
            link_channel_pairs(&mics, &inputs, muted);
        }
    }

    IS_MIC_MUTED.store(muted, Ordering::SeqCst);
    muted
}

/// Toggles system desktop audio recording for the active video.
pub fn toggle_recording_audio_mute() -> bool {
    let curr = is_recording_audio_muted();
    set_recording_audio_muted(!curr)
}

/// Toggles microphone recording for the active video.
pub fn toggle_recording_mic_mute() -> bool {
    let curr = is_recording_mic_muted();
    set_recording_mic_muted(!curr)
}
