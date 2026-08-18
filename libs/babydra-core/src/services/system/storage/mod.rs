//! Storage disk query subsystem module wrapper.

pub mod helper;
pub mod queries;

pub use queries::{get_disk_list, DiskInfo};
