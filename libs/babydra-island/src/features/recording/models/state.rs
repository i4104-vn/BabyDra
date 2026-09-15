//! State model for the recording dynamic island feature.

/// Snapshot of the in-process screen recording state polled by the background thread.
#[derive(Clone, Debug, Default)]
pub struct IslandRecordingState {
    pub is_recording: bool,
    pub is_paused: bool,
    pub elapsed_secs: u64,
    pub mode_name: String,
    pub format: String,
    pub framerate: u32,
}
