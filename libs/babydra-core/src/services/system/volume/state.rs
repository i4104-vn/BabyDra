//! Volume and mute status querying using the cached active backend.

use super::control::get_audio_backend;
use crate::models::shell::volume::AudioBackendType;
use std::process::Command;

/// Returns both current volume percentage (0.0 to 100.0+) and muted status in a single query.
pub fn get_volume_state() -> (f64, bool) {
    match get_audio_backend() {
        AudioBackendType::Wpctl => {
            if let Ok(output) = Command::new("wpctl")
                .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let is_m = stdout.contains("[MUTED]");
                let mut vol = 80.0;
                if let Some(vol_str) = stdout.split_whitespace().nth(1) {
                    if let Ok(parsed) = vol_str.parse::<f64>() {
                        vol = parsed * 100.0;
                    }
                }
                return (vol, is_m);
            }
            (80.0, false)
        }
        AudioBackendType::Pactl => (get_current_volume(), is_muted()),
        AudioBackendType::Amixer => (get_current_volume(), is_muted()),
    }
}

/// Returns `true` when `muted` holds, `false` otherwise.
pub fn is_muted() -> bool {
    match get_audio_backend() {
        AudioBackendType::Wpctl => get_volume_state().1,
        AudioBackendType::Pactl => {
            if let Ok(output) = Command::new("pactl")
                .args(["get-sink-mute", "@DEFAULT_SINK@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("Mute: yes");
            }
            false
        }
        AudioBackendType::Amixer => {
            if let Ok(output) = Command::new("amixer").args(["get", "Master"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("[off]");
            }
            false
        }
    }
}

/// Returns the current volume level as a percentage (0.0 to 100.0+).
pub fn get_current_volume() -> f64 {
    match get_audio_backend() {
        AudioBackendType::Wpctl => get_volume_state().0,
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
            80.0
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
            80.0
        }
    }
}

/// Returns both current microphone volume percentage (0.0 to 100.0+) and muted status in a single query.
pub fn get_microphone_volume_state() -> (f64, bool) {
    match get_audio_backend() {
        AudioBackendType::Wpctl => {
            if let Ok(output) = Command::new("wpctl")
                .args(["get-volume", "@DEFAULT_AUDIO_SOURCE@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let is_m = stdout.contains("[MUTED]");
                let mut vol = 100.0;
                if let Some(vol_str) = stdout.split_whitespace().nth(1) {
                    if let Ok(parsed) = vol_str.parse::<f64>() {
                        vol = parsed * 100.0;
                    }
                }
                return (vol, is_m);
            }
            (100.0, false)
        }
        AudioBackendType::Pactl => (get_current_microphone_volume(), is_microphone_muted()),
        AudioBackendType::Amixer => (get_current_microphone_volume(), is_microphone_muted()),
    }
}

/// Returns `true` when microphone is muted, `false` otherwise.
pub fn is_microphone_muted() -> bool {
    match get_audio_backend() {
        AudioBackendType::Wpctl => get_microphone_volume_state().1,
        AudioBackendType::Pactl => {
            if let Ok(output) = Command::new("pactl")
                .args(["get-source-mute", "@DEFAULT_SOURCE@"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("Mute: yes");
            }
            false
        }
        AudioBackendType::Amixer => {
            if let Ok(output) = Command::new("amixer").args(["get", "Capture"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("[off]");
            }
            false
        }
    }
}

/// Returns the current microphone volume level as a percentage (0.0 to 100.0+).
pub fn get_current_microphone_volume() -> f64 {
    match get_audio_backend() {
        AudioBackendType::Wpctl => get_microphone_volume_state().0,
        AudioBackendType::Pactl => {
            if let Ok(output) = Command::new("pactl")
                .args(["get-source-volume", "@DEFAULT_SOURCE@"])
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
            100.0
        }
        AudioBackendType::Amixer => {
            if let Ok(output) = Command::new("amixer").args(["get", "Capture"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(pos) = stdout.find('%') {
                    let start = stdout[..pos].rfind('[').unwrap_or(0);
                    if let Ok(vol) = stdout[start + 1..pos].trim().parse::<f64>() {
                        return vol;
                    }
                }
            }
            100.0
        }
    }
}
