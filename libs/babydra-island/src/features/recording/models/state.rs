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
    pub hdr: bool,
    /// Whether the active recording session was started with audio capture enabled.
    pub audio_enabled: bool,
    pub is_audio_muted: bool,
    pub is_mic_muted: bool,
}
