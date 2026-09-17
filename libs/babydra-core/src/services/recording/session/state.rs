//! Recording session state management and query functions.

use std::path::PathBuf;
use std::process::Child;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::models::recording::{RecordingConfig, RecordingStatus};

pub(crate) struct ActiveSession {
    pub(crate) child: Child,
    pub(crate) pid: u32,
    pub(crate) start_time: Instant,
    pub(crate) output_path: PathBuf,
    pub(crate) config: RecordingConfig,
    pub(crate) is_paused: bool,
    pub(crate) paused_at: Option<Instant>,
    pub(crate) total_paused_duration: Duration,
}

impl ActiveSession {
    pub(crate) fn elapsed_secs(&self) -> u64 {
        if self.is_paused {
            if let Some(paused_at) = self.paused_at {
                return (paused_at - self.start_time - self.total_paused_duration).as_secs();
            }
        }
        (Instant::now() - self.start_time - self.total_paused_duration).as_secs()
    }
}

pub(crate) static ACTIVE_RECORDING: Mutex<Option<ActiveSession>> = Mutex::new(None);

/// Returns `true` if a screen recording process is actively running.
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

/// Returns `true` if recording is currently paused.
pub fn is_paused() -> bool {
    let lock = match ACTIVE_RECORDING.lock() {
        Ok(guard) => guard,
        Err(_) => return false,
    };
    lock.as_ref().map(|s| s.is_paused).unwrap_or(false)
}

/// Returns the elapsed recording time in seconds (excluding paused periods).
pub fn get_elapsed_secs() -> u64 {
    let lock = match ACTIVE_RECORDING.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };
    lock.as_ref().map(|s| s.elapsed_secs()).unwrap_or(0)
}

/// Returns the configuration of the current recording session, if active.
pub fn get_active_config() -> Option<RecordingConfig> {
    let lock = ACTIVE_RECORDING.lock().ok()?;
    lock.as_ref().map(|s| s.config.clone())
}

/// Returns the current high-level status of screen recording.
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
