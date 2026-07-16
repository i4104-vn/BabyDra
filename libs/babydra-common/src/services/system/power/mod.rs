//! Power subsystem module wrapper.

pub mod control;

pub use control::{poweroff, reboot, suspend};
