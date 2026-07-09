//! Audio volume subsystem module wrapper.

pub mod state;
pub mod control;
pub mod device;
pub mod helper;

pub use state::{is_muted, get_current_volume};
pub use control::{set_volume, set_muted, select_audio_device};
pub use device::{get_audio_devices, AudioDevice};
