//! Volume and mute status querying using the cached active backend.

use super::control::get_audio_backend;
use crate::models::shell::volume::AudioBackendType;
use std::process::Command;

/// Returns `true` when `muted` holds, `false` otherwise.
pub fn is_muted() -> bool {
    match get_audio_backend() {
        AudioBackendType::Wpctl => {
            if let Ok(output) = Command::new("wpctl")
                .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("[MUTED]");
            }
        }
        AudioBackendType::Pactl => {
            if let Ok(output) = Command::new("pactl")
                .args(["get-sink-mute", "@DEFAULT_SINK@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("Mute: yes");
            }
        }
        AudioBackendType::Amixer => {
            if let Ok(output) = Command::new("amixer").args(["get", "Master"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("[off]");
            }
        }
    }
    false
}

/// Returns the current volume level as a percentage (0.0 to 100.0+).
pub fn get_current_volume() -> f64 {
    match get_audio_backend() {
        AudioBackendType::Wpctl => {
            if let Ok(output) = Command::new("wpctl")
                .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(vol_str) = stdout.split_whitespace().nth(1) {
                    if let Ok(vol) = vol_str.parse::<f64>() {
                        return vol * 100.0;
                    }
                }
            }
        }
        AudioBackendType::Pactl => {
            if let Ok(output) = Command::new("pactl")
                .args(["get-sink-volume", "@DEFAULT_SINK@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(pos) = stdout.find('%') {
                    let start = stdout[..pos].rfind(' ').unwrap_or(0);
                    if let Ok(vol) = stdout[start..pos].trim().parse::<f64>() {
                        return vol;
                    }
                }
            }
        }
        AudioBackendType::Amixer => {
            if let Ok(output) = Command::new("amixer").args(["get", "Master"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(pos) = stdout.find('%') {
                    let start = stdout[..pos].rfind('[').unwrap_or(0);
                    if let Ok(vol) = stdout[start + 1..pos].trim().parse::<f64>() {
                        return vol;
                    }
                }
            }
        }
    }
    80.0
}
