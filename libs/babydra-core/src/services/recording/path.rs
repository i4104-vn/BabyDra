//! Filesystem path resolvers for screen recording artifacts.

use std::path::PathBuf;

/// Resolves the default recording storage folder in `~/Videos/Recordings/`.
pub fn get_recordings_dir() -> PathBuf {
    let videos_dir = dirs::video_dir().unwrap_or_else(|| {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        home.join("Videos")
    });
    let recordings_dir = videos_dir.join("Recordings");
    let _ = std::fs::create_dir_all(&recordings_dir);
    recordings_dir
}

/// Generates a timestamped recording file path for the requested format.
pub fn get_new_recording_path(format: &str) -> PathBuf {
    let datetime = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let clean_fmt = format.trim_start_matches('.');
    get_recordings_dir().join(format!("Recording_{}.{}", datetime, clean_fmt))
}
