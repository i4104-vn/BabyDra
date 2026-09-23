//! Data models and configuration state for screen recording.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Recording capture mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum RecordingMode {
    /// Captures the default / active screen entirely.
    #[default]
    Fullscreen,
    /// Captures a specific display output by name (e.g. "HDMI-A-1").
    SingleOutput(String),
    /// Captures a specific rectangular screen region.
    Area {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Captures a specific application window or custom geometry string.
    Window(String),
}


/// Screen recording configuration parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordingConfig {
    /// Screen capture target mode.
    pub mode: RecordingMode,
    /// Target resolution scaling (width, height), or None for native resolution.
    pub resolution: Option<(u32, u32)>,
    /// Frame rate in fps (e.g. 60, 30, 24).
    pub framerate: u32,
    /// Whether to record audio along with video.
    pub audio: bool,
    /// Specific PulseAudio/PipeWire audio source device name (optional).
    pub audio_device: Option<String>,
    /// Container format extension (e.g. "mp4", "mkv", "webm").
    pub format: String,
    /// Codec override (e.g. "libx264", "h264_vaapi"), None for default.
    pub codec: Option<String>,
    /// Whether to capture in HDR10 (10-bit Rec.2020 PQ).
    pub hdr: bool,
}

impl Default for RecordingConfig {
    fn default() -> Self {
        Self {
            mode: RecordingMode::Fullscreen,
            resolution: None,
            framerate: 60,
            audio: false,
            audio_device: None,
            format: "mp4".to_string(),
            codec: None,
            hdr: false,
        }
    }
}

/// High-level recording state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingStatus {
    /// No active recording is occurring.
    Idle,
    /// A screen recording session is actively running.
    Recording {
        pid: u32,
        output_path: PathBuf,
        elapsed_secs: u64,
        config: RecordingConfig,
        is_paused: bool,
    },
}

impl RecordingStatus {
    pub fn is_recording(&self) -> bool {
        matches!(self, RecordingStatus::Recording { .. })
    }

    pub fn is_paused(&self) -> bool {
        match self {
            RecordingStatus::Recording { is_paused, .. } => *is_paused,
            RecordingStatus::Idle => false,
        }
    }

    pub fn elapsed_secs(&self) -> u64 {
        match self {
            RecordingStatus::Recording { elapsed_secs, .. } => *elapsed_secs,
            RecordingStatus::Idle => 0,
        }
    }
}
