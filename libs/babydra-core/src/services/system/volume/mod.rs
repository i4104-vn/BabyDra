//! Audio volume subsystem module wrapper.

pub mod control;
pub mod device;
pub mod helper;
pub mod state;

pub use crate::models::shell::volume::AudioBackendType;
pub use control::{
    get_audio_backend, select_audio_device, select_audio_source, set_microphone_muted,
    set_microphone_volume, set_muted, set_volume,
};
pub use device::{get_audio_devices, AudioDevice};
pub use state::{get_current_microphone_volume, get_current_volume, is_microphone_muted, is_muted};
