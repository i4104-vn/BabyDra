//! FFprobe JSON schema data models.

use serde::{Deserialize, Serialize};

/// Root output object from `ffprobe -print_format json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FfprobeOutput {
    pub streams: Option<Vec<FfprobeStream>>,
    pub format: Option<FfprobeFormat>,
}

/// Stream metadata descriptor in FFprobe output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FfprobeStream {
    pub codec_type: Option<String>,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub r_frame_rate: Option<String>,
    pub avg_frame_rate: Option<String>,
    pub bit_rate: Option<String>,
    pub pix_fmt: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<u32>,
    pub channel_layout: Option<String>,
}

/// Format metadata descriptor in FFprobe output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FfprobeFormat {
    pub format_name: Option<String>,
    pub format_long_name: Option<String>,
    pub duration: Option<String>,
    pub size: Option<String>,
    pub bit_rate: Option<String>,
}
