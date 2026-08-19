//! Power subsystem module wrapper.

pub mod control;
pub mod profile;

pub use control::{poweroff, reboot, suspend};
pub use profile::{
    apply_saved_profile, get_current_profile, set_perf_profile, set_perf_profile_pw,
};
