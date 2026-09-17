//! File and directory path management for recording outputs.

pub mod resolver;

pub use resolver::{get_new_recording_path, get_recordings_dir};
