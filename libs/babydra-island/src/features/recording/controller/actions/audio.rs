//! Audio and microphone mute action handlers.

use babydra_core::i18n::trans;
use crate::features::recording::service::{toggle_audio_mute, toggle_mic_mute};
use crate::features::recording::ui::RecordingButtonWidget;

/// Toggles audio mute state and updates the button visual.
pub fn toggle_audio_mute_action(button: &RecordingButtonWidget) {
    let muted = toggle_audio_mute();
    button.set_icon_and_title(
        if muted {
            "audio-volume-muted"
        } else {
            "audio-volume-high"
        },
        &trans(if muted {
            "recorder.audio_unmute"
        } else {
            "recorder.audio_mute"
        }),
    );
    button.set_alert(muted);
}

/// Toggles microphone mute state and updates the button visual.
pub fn toggle_mic_mute_action(button: &RecordingButtonWidget) {
    let muted = toggle_mic_mute();
    button.set_icon_and_title(
        if muted {
            "microphone-disabled"
        } else {
            "audio-input-microphone"
        },
        &trans(if muted {
            "recorder.mic_unmute"
        } else {
            "recorder.mic_mute"
        }),
    );
    button.set_alert(muted);
}
