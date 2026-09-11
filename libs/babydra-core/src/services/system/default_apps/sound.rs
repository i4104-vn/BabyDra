//! System alert and event sound settings.

use crate::services::utils::gsettings;
use std::process::Command;

/// Returns whether system event sounds are enabled.
pub fn get_event_sounds_enabled() -> bool {
    gsettings::get_bool("org.gnome.desktop.sound", "event-sounds", true)
}

/// Sets whether system event sounds are enabled.
pub fn set_event_sounds_enabled(enabled: bool) {
    gsettings::set_bool("org.gnome.desktop.sound", "event-sounds", enabled);
}

/// Returns whether input feedback sounds are enabled.
pub fn get_input_feedback_sounds_enabled() -> bool {
    gsettings::get_bool("org.gnome.desktop.sound", "input-feedback-sounds", false)
}

/// Sets whether input feedback sounds are enabled.
pub fn set_input_feedback_sounds_enabled(enabled: bool) {
    gsettings::set_bool("org.gnome.desktop.sound", "input-feedback-sounds", enabled);
}

/// Plays a system alert sound for testing audio output.
pub fn play_test_alert_sound() {
    let sound_paths = [
        "/usr/share/sounds/freedesktop/stereo/bell.oga",
        "/usr/share/sounds/freedesktop/stereo/audio-volume-change.oga",
        "/usr/share/sounds/freedesktop/stereo/message.oga",
    ];

    let found_path = sound_paths.iter().find(|p| std::path::Path::new(p).exists());

    if let Some(path) = found_path {
        // Try pw-play first, then paplay
        if Command::new("pw-play").arg(path).spawn().is_err() {
            let _ = Command::new("paplay").arg(path).spawn();
        }
    }
}
