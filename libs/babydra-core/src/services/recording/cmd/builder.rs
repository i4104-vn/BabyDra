//! Construction and flag generation for wf-recorder process invocations.

use std::path::Path;
use std::process::Command;

use crate::models::recording::{RecordingConfig, RecordingMode};

/// Constructs the `Command` to launch `wf-recorder` based on the specified recording configuration.
pub fn build_wf_recorder_command(config: &RecordingConfig, output_path: &Path) -> Command {
    let mut cmd = Command::new("wf-recorder");
    cmd.arg("-f").arg(output_path);
    cmd.arg("-y"); // overwrite confirmation bypass

    // Framerate
    if config.framerate > 0 {
        cmd.arg("-r").arg(config.framerate.to_string());
    }

    // Audio capture
    if config.audio {
        if let Some(ref device) = config.audio_device {
            let source = if device.trim().is_empty()
                || device.starts_with("profile:")
                || device.starts_with("route:")
            {
                "default"
            } else {
                device.as_str()
            };
            cmd.arg(format!("--audio={source}"));
        } else {
            cmd.arg("--audio=default");
        }
        cmd.arg("--audio-backend=pipewire");
    }

    let is_vaapi = config
        .codec
        .as_deref()
        .map(|c| c.contains("vaapi"))
        .unwrap_or(false);

    if config.hdr {
        // 10-bit High Dynamic Range / Deep Color mode
        if !is_vaapi {
            if let Some((width, height)) = config.resolution {
                cmd.arg("-F").arg(format!(
                    "scale={}:{}:flags=lanczos+accurate_rnd,format=yuv420p10le",
                    width, height
                ));
            }
            cmd.arg("-x").arg("yuv420p10le");
        } else if let Some((width, height)) = config.resolution {
            cmd.arg("-F").arg(format!("scale_vaapi=w={}:h={}:format=p010", width, height));
            cmd.arg("-x").arg("p010le");
        } else {
            cmd.arg("-x").arg("p010le");
        }

        let codec = config.codec.as_deref().unwrap_or_else(|| {
            if config.format == "webm" {
                "libvpx-vp9"
            } else {
                "libx264"
            }
        });
        cmd.arg("-c").arg(codec);

        if codec == "libx264" || codec.contains("x264") {
            cmd.arg("-p").arg("color_range=pc");
            cmd.arg("-p").arg("colorspace=bt709");
            cmd.arg("-p").arg("color_primaries=bt709");
            cmd.arg("-p").arg("color_trc=bt709");
            cmd.arg("-p").arg("crf=16");
            cmd.arg("-p").arg("preset=veryfast");
        } else if codec == "libx265" || codec.contains("x265") || codec.contains("hevc") {
            cmd.arg("-p").arg("color_range=pc");
            cmd.arg("-p").arg("colorspace=bt709");
            cmd.arg("-p").arg("color_primaries=bt709");
            cmd.arg("-p").arg("color_trc=bt709");
            cmd.arg("-p").arg("crf=16");
            cmd.arg("-p").arg("preset=superfast");
        } else if codec.contains("vaapi") {
            cmd.arg("-p").arg("color_range=pc");
        } else if codec.contains("vp9") {
            cmd.arg("-p").arg("crf=16");
            cmd.arg("-p").arg("profile=2");
        }
    } else {
        if !is_vaapi {
            if let Some((width, height)) = config.resolution {
                cmd.arg("-F").arg(format!(
                    "scale={}:{}:flags=lanczos+accurate_rnd,format=yuv420p",
                    width, height
                ));
            }
            cmd.arg("-x").arg("yuv420p");
        } else if let Some((width, height)) = config.resolution {
            cmd.arg("-F").arg(format!("scale_vaapi=w={}:h={}:format=nv12", width, height));
        }

        let codec = config.codec.as_deref().unwrap_or_else(|| {
            if config.format == "webm" {
                "libvpx-vp9"
            } else {
                "libx264"
            }
        });
        cmd.arg("-c").arg(codec);

        if codec == "libx264" || codec.contains("x264") {
            cmd.arg("-p").arg("color_range=pc");
            cmd.arg("-p").arg("colorspace=bt709");
            cmd.arg("-p").arg("color_primaries=bt709");
            cmd.arg("-p").arg("color_trc=bt709");
            cmd.arg("-p").arg("crf=18");
            cmd.arg("-p").arg("preset=veryfast");
        }
    }

    // Capture mode specific args
    match &config.mode {
        RecordingMode::Fullscreen => {}
        RecordingMode::SingleOutput(output_name) => {
            cmd.arg("-o").arg(output_name);
        }
        RecordingMode::Area {
            x,
            y,
            width,
            height,
        } => {
            cmd.arg("-g")
                .arg(format!("{},{} {}x{}", x, y, width, height));
        }
        RecordingMode::Window(geometry_str) => {
            if !geometry_str.trim().is_empty() {
                cmd.arg("-g").arg(geometry_str.trim());
            }
        }
    }

    cmd
}
