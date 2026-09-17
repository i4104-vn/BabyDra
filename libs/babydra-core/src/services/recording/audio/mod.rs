//! Isolated audio and microphone stream management for screen recording sessions.

pub mod mute;
pub mod pipewire;

pub use mute::{
    is_recording_audio_muted, is_recording_mic_muted, set_recording_audio_muted,
    set_recording_mic_muted, toggle_recording_audio_mute, toggle_recording_mic_mute,
};
pub use pipewire::{init_recording_audio, reset_recording_audio};
