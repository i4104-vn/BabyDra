//! Recording session lifecycle orchestration: start, stop, pause, resume.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::models::recording::RecordingConfig;
use crate::services::notification::service::send_app_notif_with_cmd;

use super::state::{ActiveSession, ACTIVE_RECORDING, is_recording};

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

    let output_path = crate::services::recording::path::get_new_recording_path(&config.format);
    let mut cmd = crate::services::recording::cmd::build_wf_recorder_command(config, &output_path);

    let child = cmd.spawn().map_err(|e| {
        format!("Failed to launch wf-recorder: {e}. Make sure wf-recorder is installed.")
    })?;

    let pid = child.id();
    crate::services::recording::audio::init_recording_audio(pid, config.audio);
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

/// Pauses the active recording session by sending SIGSTOP to the recorder process.
pub fn pause_recording() -> Result<(), String> {
    let mut lock = ACTIVE_RECORDING
        .lock()
        .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;

    if let Some(ref mut session) = *lock {
        if session.is_paused {
            return Ok(());
        }
        #[cfg(unix)]
        {
            let res = unsafe { libc::kill(session.pid as i32, libc::SIGSTOP) };
            if res != 0 {
                return Err(format!(
                    "Failed to pause recording process: errno {}",
                    std::io::Error::last_os_error()
                ));
            }
        }
        session.is_paused = true;
        session.paused_at = Some(Instant::now());
        Ok(())
    } else {
        Err("No active recording session to pause".to_string())
    }
}

/// Resumes a paused recording session by sending SIGCONT to the recorder process.
pub fn resume_recording() -> Result<(), String> {
    let mut lock = ACTIVE_RECORDING
        .lock()
        .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;

    if let Some(ref mut session) = *lock {
        if !session.is_paused {
            return Ok(());
        }
        #[cfg(unix)]
        {
            let res = unsafe { libc::kill(session.pid as i32, libc::SIGCONT) };
            if res != 0 {
                return Err(format!(
                    "Failed to resume recording process: errno {}",
                    std::io::Error::last_os_error()
                ));
            }
        }
        if let Some(paused_at) = session.paused_at.take() {
            session.total_paused_duration += Instant::now().saturating_duration_since(paused_at);
        }
        session.is_paused = false;
        Ok(())
    } else {
        Err("No active recording session to resume".to_string())
    }
}

/// Toggles pause/resume state of the active recording.
pub fn toggle_pause() -> Result<bool, String> {
    let is_currently_paused = {
        let lock = ACTIVE_RECORDING
            .lock()
            .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;
        match *lock {
            Some(ref s) => s.is_paused,
            None => return Err("No active recording session".to_string()),
        }
    };

    if is_currently_paused {
        resume_recording()?;
        Ok(false)
    } else {
        pause_recording()?;
        Ok(true)
    }
}

/// Stops the active recording process gracefully (SIGINT), closes audio nodes, and sends notification.
pub fn stop_recording() -> Result<Option<PathBuf>, String> {
    crate::services::recording::audio::reset_recording_audio();

    let mut lock = ACTIVE_RECORDING
        .lock()
        .map_err(|e| format!("Failed to lock recording mutex: {e}"))?;

    let mut session = match lock.take() {
        Some(s) => s,
        None => return Ok(None),
    };

    if session.is_paused {
        #[cfg(unix)]
        unsafe {
            libc::kill(session.pid as i32, libc::SIGCONT);
        }
    }

    #[cfg(unix)]
    unsafe {
        libc::kill(session.pid as i32, libc::SIGINT);
    }

    let start_wait = Instant::now();
    let timeout = Duration::from_millis(5000);
    loop {
        match session.child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if start_wait.elapsed() > timeout {
                    let _ = session.child.kill();
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
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
    send_app_notif_with_cmd(&notif_title, &notif_title, &notif_msg, "camera", &open_cmd);

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
