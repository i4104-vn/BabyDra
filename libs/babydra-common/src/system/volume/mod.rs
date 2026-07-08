//! Audio volume subsystem module wrapper.

pub mod state;
pub mod control;
pub mod device;

pub use state::{is_muted, get_current_volume};
pub use control::set_volume;
pub use device::{get_audio_devices, AudioDevice};
