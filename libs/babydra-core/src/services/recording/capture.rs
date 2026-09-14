//! Screen recording process lifecycle management via wf-recorder.

use crate::models::recording::{RecordingConfig, RecordingMode, RecordingStatus};
use crate::services::notification::service::send_notification;
use super::path::get_new_recording_path;
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
        let elapsed = session.start_time.elapsed();
        let current_paused = if let Some(p) = session.paused_at {
            p.elapsed()
        } else {
            Duration::ZERO
        };
        let active = elapsed.saturating_sub(session.total_paused_duration + current_paused);
        active.as_secs()
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
            Ok(None) => {
                let elapsed = session.start_time.elapsed();
                let current_paused = if let Some(p) = session.paused_at {
                    p.elapsed()
                } else {
                    Duration::ZERO
                };
                let active =
                    elapsed.saturating_sub(session.total_paused_duration + current_paused);
                RecordingStatus::Recording {
                    pid: session.pid,
                    output_path: session.output_path.clone(),
                    elapsed_secs: active.as_secs(),
                    config: session.config.clone(),
                    is_paused: session.is_paused,
                }
            }
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
            cmd.arg(format!("-a{}", device));
        } else {
            cmd.arg("-a");
        }
    }

    // Resolution scaling filter
    if let Some((width, height)) = config.resolution {
        cmd.arg("-F").arg(format!("scale={}:{}", width, height));
    }

    // Codec override
    if let Some(ref codec) = config.codec {
        cmd.arg("-c").arg(codec);
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
            cmd.arg("-g").arg(format!("{},{} {}x{}", x, y, width, height));
        }
        RecordingMode::Window(geometry_str) => {
            if !geometry_str.trim().is_empty() {
                cmd.arg("-g").arg(geometry_str.trim());
            }
        }
    }

    let child = cmd.spawn().map_err(|e| {
        format!(
            "Failed to launch wf-recorder: {e}. Make sure wf-recorder is installed."
        )
    })?;

    let pid = child.id();
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

    let filename = output_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Recording".to_string());

    let notif_title = crate::i18n::trans("recorder.title");
    let notif_msg = format!("{} ({})", crate::i18n::trans("recorder.notif_started"), filename);
    send_notification(&notif_title, &notif_msg);

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
    let notif_msg = format!("{} {}", crate::i18n::trans("recorder.notif_saved"), session.output_path.display());
    send_notification(&notif_title, &notif_msg);

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
