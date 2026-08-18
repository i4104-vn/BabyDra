//! Volume control action triggers with one-time backend detection and zero-overhead caching.

use crate::models::shell::volume::AudioBackendType;
use std::process::Command;
use std::sync::OnceLock;

static ACTIVE_AUDIO_BACKEND: OnceLock<AudioBackendType> = OnceLock::new();

/// Returns the detected active audio backend, evaluating only once on first access.
pub fn get_audio_backend() -> AudioBackendType {
    *ACTIVE_AUDIO_BACKEND.get_or_init(|| {
        // 1. PipeWire / WirePlumber (wpctl) — preferred default on modern Arch Linux
        if Command::new("which")
            .arg("wpctl")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return AudioBackendType::Wpctl;
        }

        // 2. PulseAudio (pactl)
        if Command::new("which")
            .arg("pactl")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return AudioBackendType::Pactl;
        }

        // 3. Fallback ALSA (amixer)
        AudioBackendType::Amixer
    })
}

/// Sets `volume` to the given percentage value using the active audio backend.
pub fn set_volume(val: f64) {
    let percent = val.clamp(0.0, 150.0) as i32;

    match get_audio_backend() {
        AudioBackendType::Wpctl => {
            let _ = Command::new("wpctl")
                .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &format!("{}%", percent)])
                .spawn();
            if percent > 0 {
                let _ = Command::new("wpctl")
                    .args(["set-mute", "@DEFAULT_AUDIO_SINK@", "0"])
                    .spawn();
            }
        }
        AudioBackendType::Pactl => {
            let _ = Command::new("pactl")
                .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", percent)])
                .spawn();
            if percent > 0 {
                let _ = Command::new("pactl")
                    .args(["set-sink-mute", "@DEFAULT_SINK@", "0"])
                    .spawn();
            }
        }
        AudioBackendType::Amixer => {
            let _ = Command::new("amixer")
                .args(["set", "Master", &format!("{}%", percent)])
                .spawn();
            if percent > 0 {
                let _ = Command::new("amixer")
                    .args(["set", "Master", "unmute"])
                    .spawn();
            }
        }
    }
}

/// Toggles the master mute status using the active audio backend.
pub fn set_muted(muted: bool) {
    let mute_val = if muted { "1" } else { "0" };

    match get_audio_backend() {
        AudioBackendType::Wpctl => {
            let _ = Command::new("wpctl")
                .args(["set-mute", "@DEFAULT_AUDIO_SINK@", mute_val])
                .spawn();
        }
        AudioBackendType::Pactl => {
            let _ = Command::new("pactl")
                .args(["set-sink-mute", "@DEFAULT_SINK@", mute_val])
                .spawn();
        }
        AudioBackendType::Amixer => {
            let _ = Command::new("amixer")
                .args(["set", "Master", if muted { "mute" } else { "unmute" }])
                .spawn();
        }
    }
}

/// Commands the audio subsystem to activate/default a specific audio device or route configuration.
pub fn select_audio_device(name: &str) {
    match get_audio_backend() {
        AudioBackendType::Wpctl => {
            if name.starts_with("route:") {
                let parts: Vec<&str> = name.split(':').collect();
                if parts.len() == 4 {
                    let card_id = parts[1];
                    let route_index = parts[2];
                    let profile_index = parts[3];
                    let _ = Command::new("wpctl")
                        .args(["set-profile", card_id, profile_index])
                        .status();
                    let _ = Command::new("wpctl")
                        .args(["set-route", card_id, route_index])
                        .status();
                }
            } else if name.starts_with("profile:") {
                let parts: Vec<&str> = name.split(':').collect();
                if parts.len() == 3 {
                    let card_id = parts[1];
                    let profile_index = parts[2];
                    let _ = Command::new("wpctl")
                        .args(["set-profile", card_id, profile_index])
                        .status();
                }
            } else {
                let _ = Command::new("wpctl")
                    .args(["set-default", name])
                    .status();
            }
        }
        AudioBackendType::Pactl => {
            let _ = Command::new("pactl")
                .args(["set-default-sink", name])
                .status();
        }
        AudioBackendType::Amixer => {
            // ALSA does not support dynamic sink switching through simple name
        }
    }
}
