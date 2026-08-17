//! Storage disk query subsystem module wrapper.

pub mod helper;
pub mod query;

pub use query::{get_disk_list, DiskInfo};
