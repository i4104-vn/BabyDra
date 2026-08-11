//! Power subsystem module wrapper.

pub mod control;
pub mod profile;

pub use control::{poweroff, reboot, suspend};
pub use profile::{get_current_profile, set_performance_profile, set_performance_profile_with_password, apply_saved_profile};
