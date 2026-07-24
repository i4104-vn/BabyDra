//! Power subsystem module wrapper.

pub mod control;
pub mod profile;

pub use control::{poweroff, reboot, suspend};
pub use profile::{get_current_profile, set_performance_profile, get_battery_info, get_profile_config_path};
