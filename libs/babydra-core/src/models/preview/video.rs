//! Video metadata, stream specifications, and playback runtime state.

use serde::{Deserialize, Serialize};

/// Video stream metadata extracted via FFprobe.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoStreamInfo {
    pub width: u32,
    pub height: u32,
    pub codec_name: String,
    pub codec_long_name: Option<String>,
    pub fps: Option<f64>,
    pub bit_rate: Option<u64>,
    pub pix_fmt: Option<String>,
}

/// Audio stream metadata extracted via FFprobe.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioStreamInfo {
    pub codec_name: String,
    pub codec_long_name: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<u32>,
    pub channel_layout: Option<String>,
    pub bit_rate: Option<u64>,
}

/// Aggregated video metadata container.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub width: u32,
    pub height: u32,
    pub duration_secs: f64,
    pub format_name: String,
    pub format_long_name: String,
    pub file_size: u64,
    pub bit_rate: Option<u64>,
    pub video_stream: Option<VideoStreamInfo>,
    pub audio_stream: Option<AudioStreamInfo>,
}

/// Video playback runtime state.
#[derive(Debug, Clone)]
pub struct VideoState {
    pub duration_us: i64,
    pub position_us: i64,
    pub is_playing: bool,
    pub is_seeking: bool,
    pub volume: f64,
    pub is_muted: bool,
    pub speed: f64,
}

impl Default for VideoState {
    fn default() -> Self {
        Self {
            duration_us: 0,
            position_us: 0,
            is_playing: false,
            is_seeking: false,
            volume: 1.0,
            is_muted: false,
            speed: 1.0,
        }
    }
}
