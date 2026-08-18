//! Volume/Audio device configuration and backend models.

use serde::{Deserialize, Serialize};

/// Audio subsystem backend type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AudioBackendType {
    /// PipeWire / WirePlumber controller (`wpctl`) — default recommendation
    #[default]
    Wpctl,
    /// PulseAudio controller (`pactl`)
    Pactl,
    /// ALSA controller (`amixer`)
    Amixer,
}

impl AudioBackendType {
    /// Returns the human-readable display name of the audio backend.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Wpctl => "PipeWire (wpctl)",
            Self::Pactl => "PulseAudio (pactl)",
            Self::Amixer => "ALSA (amixer)",
        }
    }

    /// Returns the binary command name associated with this backend.
    pub fn command_name(&self) -> &'static str {
        match self {
            Self::Wpctl => "wpctl",
            Self::Pactl => "pactl",
            Self::Amixer => "amixer",
        }
    }
}

/// Represents an audio output/input device endpoint.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AudioDevice {
    pub name: String,
    pub description: String,
    pub is_default: bool,
}
