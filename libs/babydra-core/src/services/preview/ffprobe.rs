//! FFprobe metadata extraction service for video and audio properties.

use crate::models::preview::{AudioStreamInfo, FfprobeOutput, VideoMetadata, VideoStreamInfo};
use std::path::Path;
use std::process::Command;

/// Parses a fractional frame rate string such as "30/1" or "24000/1001" into an f64.
fn parse_fractional_rate(rate_str: &str) -> Option<f64> {
    let parts: Vec<&str> = rate_str.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].trim().parse().ok()?;
        let den: f64 = parts[1].trim().parse().ok()?;
        if den > 0.0 {
            return Some(num / den);
        }
    } else if let Ok(val) = rate_str.trim().parse::<f64>() {
        if val > 0.0 {
            return Some(val);
        }
    }
    None
}

/// Probes the specified video file using `ffprobe` and extracts full stream metadata.
pub fn probe_video(path: &Path) -> VideoMetadata {
    let mut meta = VideoMetadata {
        width: 1280,
        height: 720,
        duration_secs: 0.0,
        format_name: "Video".to_string(),
        format_long_name: "Video File".to_string(),
        file_size: std::fs::metadata(path).map(|m| m.len()).unwrap_or(0),
        bit_rate: None,
        video_stream: None,
        audio_stream: None,
    };

    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(path)
        .output();

    let Ok(output) = output else {
        return meta;
    };

    if !output.status.success() {
        return meta;
    }

    let parsed: Result<FfprobeOutput, _> = serde_json::from_slice(&output.stdout);
    let Ok(data) = parsed else {
        return meta;
    };

    // Parse format section
    if let Some(ref fmt) = data.format {
        if let Some(ref fn_name) = fmt.format_name {
            meta.format_name = fn_name.clone();
        }
        if let Some(ref fn_long) = fmt.format_long_name {
            meta.format_long_name = fn_long.clone();
        }
        if let Some(ref dur_str) = fmt.duration {
            if let Ok(d) = dur_str.trim().parse::<f64>() {
                meta.duration_secs = d;
            }
        }
        if let Some(ref sz_str) = fmt.size {
            if let Ok(sz) = sz_str.trim().parse::<u64>() {
                meta.file_size = sz;
            }
        }
        if let Some(ref br_str) = fmt.bit_rate {
            if let Ok(br) = br_str.trim().parse::<u64>() {
                meta.bit_rate = Some(br);
            }
        }
    }

    // Parse streams
    if let Some(streams) = data.streams {
        for s in streams {
            match s.codec_type.as_deref() {
                Some("video") if meta.video_stream.is_none() => {
                    let w = s.width.unwrap_or(0);
                    let h = s.height.unwrap_or(0);
                    if w > 0 && h > 0 {
                        meta.width = w;
                        meta.height = h;
                    }

                    let fps = s
                        .avg_frame_rate
                        .as_deref()
                        .and_then(parse_fractional_rate)
                        .or_else(|| {
                            s.r_frame_rate
                                .as_deref()
                                .and_then(parse_fractional_rate)
                        });

                    let br = s
                        .bit_rate
                        .as_deref()
                        .and_then(|b| b.trim().parse::<u64>().ok());

                    meta.video_stream = Some(VideoStreamInfo {
                        width: w,
                        height: h,
                        codec_name: s.codec_name.unwrap_or_else(|| "Unknown".to_string()),
                        codec_long_name: s.codec_long_name,
                        fps,
                        bit_rate: br,
                        pix_fmt: s.pix_fmt,
                    });
                }
                Some("audio") if meta.audio_stream.is_none() => {
                    let br = s
                        .bit_rate
                        .as_deref()
                        .and_then(|b| b.trim().parse::<u64>().ok());

                    meta.audio_stream = Some(AudioStreamInfo {
                        codec_name: s.codec_name.unwrap_or_else(|| "Unknown".to_string()),
                        codec_long_name: s.codec_long_name,
                        sample_rate: s.sample_rate,
                        channels: s.channels,
                        channel_layout: s.channel_layout,
                        bit_rate: br,
                    });
                }
                _ => {}
            }
        }
    }

    meta
}
