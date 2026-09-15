//! Media preview services for video inspection and playback control.

pub mod ffprobe;
pub mod playback;

pub use ffprobe::probe_video;
pub use playback::{set_gst_play_rate, set_media_file_speed_raw, SPEED_PRESETS};
