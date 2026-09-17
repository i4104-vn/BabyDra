//! Screen recording process lifecycle management via wf-recorder.

use super::path::get_new_recording_path;
use crate::models::recording::{RecordingConfig, RecordingMode, RecordingStatus};
use crate::services::notification::service::send_app_notif_with_cmd;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct ActiveSession {
    child: Child,
    pid: u32,
    start_time: Instant,
    output_path: PathBuf,
    config: RecordingConfig,
    is_paused: bool,
    paused_at: Option<Instant>,
    total_paused_duration: Duration,
}

impl ActiveSession {
    fn elapsed_secs(&self) -> u64 {
        let current_paused = self
            .paused_at
            .map(|paused_at| paused_at.elapsed())
            .unwrap_or_default();
        self.start_time
            .elapsed()
            .saturating_sub(self.total_paused_duration + current_paused)
            .as_secs()
    }
}

static ACTIVE_RECORDING: Mutex<Option<ActiveSession>> = Mutex::new(None);

/// Checks whether an active screen recording is currently taking place.
pub fn is_recording() -> bool {
    let mut lock = match ACTIVE_RECORDING.lock() {
        Ok(guard) => guard,
        Err(_) => return false,
    };

    if let Some(ref mut session) = *lock {
        match session.child.try_wait() {
            Ok(None) => true,
            _ => {
                *lock = None;
                false
            }
        }
    } else {
        false
    }
}

/// Returns whether the active recording is currently paused.
pub fn is_paused() -> bool {
    let lock = match ACTIVE_RECORDING.lock() {
        Ok(guard) => guard,
        Err(_) => return false,
    };

    if let Some(ref session) = *lock {
        session.is_paused
    } else {
        false
    }
}

/// Returns elapsed active seconds since recording started (excluding paused duration).
pub fn get_elapsed_secs() -> u64 {
    let lock = match ACTIVE_RECORDING.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };

    if let Some(ref session) = *lock {
        session.elapsed_secs()
    } else {
        0
    }
}

/// Returns the current active configuration if recording is ongoing.
pub fn get_active_config() -> Option<RecordingConfig> {
    let lock = ACTIVE_RECORDING.lock().ok()?;
    lock.as_ref().map(|s| s.config.clone())
}

/// Returns the current high-level recording status.
pub fn get_status() -> RecordingStatus {
    let mut lock = match ACTIVE_RECORDING.lock() {
        Ok(guard) => guard,
        Err(_) => return RecordingStatus::Idle,
    };

    if let Some(ref mut session) = *lock {
        match session.child.try_wait() {
            Ok(None) => RecordingStatus::Recording {
                pid: session.pid,
                output_path: session.output_path.clone(),
                elapsed_secs: session.elapsed_secs(),
                config: session.config.clone(),
                is_paused: session.is_paused,
            },
            _ => {
                *lock = None;
                RecordingStatus::Idle
            }
        }
    } else {
        RecordingStatus::Idle
    }
}

/// Launches `wf-recorder` with the given configuration.
pub fn start_recording(config: &RecordingConfig) -> Result<PathBuf, String> {
    let mut lock = ACTIVE_RECORDING
        .lock()
        .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;

    // Check if already running
    if let Some(ref mut session) = *lock {
        if let Ok(None) = session.child.try_wait() {
            return Err("A screen recording is already in progress".to_string());
        }
    }

    let output_path = get_new_recording_path(&config.format);

    let mut cmd = Command::new("wf-recorder");
    cmd.arg("-f").arg(&output_path);
    cmd.arg("-y"); // overwrite confirmation bypass

    // Framerate
    if config.framerate > 0 {
        cmd.arg("-r").arg(config.framerate.to_string());
    }

    // Audio capture
    if config.audio {
        if let Some(ref device) = config.audio_device {
            // The volume service exposes PipeWire profile identifiers such as
            // `profile:12:3`; those are not PulseAudio source names and make
            // wf-recorder exit immediately.  Use the real default source for
            // those legacy values; the PipeWire links in audio.rs add the
            // desktop monitor as well.
            let source = if device.trim().is_empty()
                || device.starts_with("profile:")
                || device.starts_with("route:")
            {
                "default"
            } else {
                device.as_str()
            };
            // `-a` has an optional value in wf-recorder.  Passing the value
            // as a second argv item is parsed as an unexpected positional
            // argument, so use the documented `--audio=<device>` form.
            cmd.arg(format!("--audio={source}"));
        } else {
            cmd.arg("--audio=default");
        }
        // The recording audio linker below operates on PipeWire ports.  Make
        // the backend explicit instead of relying on the distro default
        // (which may be PulseAudio and expose no wf-recorder PipeWire node).
        cmd.arg("--audio-backend=pipewire");
    }

    let is_vaapi = config
        .codec
        .as_deref()
        .map(|c| c.contains("vaapi"))
        .unwrap_or(false);

    if !is_vaapi {
        let filter_str = if let Some((width, height)) = config.resolution {
            format!(
                "scale={}:{}:in_range=full:out_range=full:out_color_matrix=bt709:flags=accurate_rnd+full_chroma_int,format=yuv420p",
                width, height
            )
        } else {
            "scale=in_range=full:out_range=full:out_color_matrix=bt709:flags=accurate_rnd+full_chroma_int,format=yuv420p".to_string()
        };
        cmd.arg("-F").arg(filter_str);
    } else if let Some((width, height)) = config.resolution {
        cmd.arg("-F").arg(format!("scale_vaapi=w={}:h={}:format=nv12", width, height));
    }

    // Codec override and color metadata tagging
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

    let child = cmd.spawn().map_err(|e| {
        format!("Failed to launch wf-recorder: {e}. Make sure wf-recorder is installed.")
    })?;

    let pid = child.id();
    super::audio::init_recording_audio(pid, config.audio);
    *lock = Some(ActiveSession {
        child,
        pid,
        start_time: Instant::now(),
        output_path: output_path.clone(),
        config: config.clone(),
        is_paused: false,
        paused_at: None,
        total_paused_duration: Duration::ZERO,
    });

    Ok(output_path)
}

/// Pauses the active recording using SIGSTOP.
pub fn pause_recording() -> Result<bool, String> {
    let mut lock = ACTIVE_RECORDING
        .lock()
        .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;

    let Some(ref mut session) = *lock else {
        return Err("No active recording session to pause".to_string());
    };

    if session.is_paused {
        return Ok(false);
    }

    unsafe {
        libc::kill(session.pid as i32, libc::SIGSTOP);
    }

    session.is_paused = true;
    session.paused_at = Some(Instant::now());
    Ok(true)
}

/// Resumes the active recording using SIGCONT.
pub fn resume_recording() -> Result<bool, String> {
    let mut lock = ACTIVE_RECORDING
        .lock()
        .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;

    let Some(ref mut session) = *lock else {
        return Err("No active recording session to resume".to_string());
    };

    if !session.is_paused {
        return Ok(false);
    }

    unsafe {
        libc::kill(session.pid as i32, libc::SIGCONT);
    }

    if let Some(p) = session.paused_at.take() {
        session.total_paused_duration += p.elapsed();
    }
    session.is_paused = false;
    Ok(true)
}

/// Toggles pause state between paused and running.
pub fn toggle_pause() -> Result<bool, String> {
    if is_paused() {
        resume_recording()?;
        Ok(false)
    } else {
        pause_recording()?;
        Ok(true)
    }
}

/// Stops the current recording cleanly by sending SIGINT to `wf-recorder`.
pub fn stop_recording() -> Result<Option<PathBuf>, String> {
    let session_opt = {
        let mut lock = ACTIVE_RECORDING
            .lock()
            .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;
        lock.take()
    };

    super::audio::reset_recording_audio();

    let Some(mut session) = session_opt else {
        crate::services::utils::pkill_signal("-SIGINT", "wf-recorder", true);
        return Ok(None);
    };

    // If was paused, resume first so it can receive SIGINT and process exit cleanly
    if session.is_paused {
        unsafe {
            libc::kill(session.pid as i32, libc::SIGCONT);
        }
    }

    // Send SIGINT so wf-recorder finalizes headers (crucial for MP4 moov atom)
    unsafe {
        libc::kill(session.pid as i32, libc::SIGINT);
    }

    // Wait up to 6 seconds for clean process termination
    let start_wait = Instant::now();
    loop {
        match session.child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if start_wait.elapsed().as_secs() >= 6 {
                    let _ = session.child.kill();
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(_) => break,
        }
    }

    let notif_title = crate::i18n::trans("recorder.title");
    let notif_msg = format!(
        "{} {}",
        crate::i18n::trans("recorder.notif_saved"),
        session.output_path.display()
    );
    let open_cmd = format!("babydra-explore \"{}\"", session.output_path.display());
    send_app_notif_with_cmd(&notif_title, &notif_title, &notif_msg, "babydra", &open_cmd);

    Ok(Some(session.output_path))
}

/// Toggles recording state: starts if idle, stops if recording.
pub fn toggle_recording(config: Option<RecordingConfig>) -> Result<bool, String> {
    if is_recording() {
        stop_recording()?;
        Ok(false)
    } else {
        start_recording(&config.unwrap_or_default())?;
        Ok(true)
    }
}
