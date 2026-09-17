//! Session lifecycle action handlers (start, stop, pause).

use babydra_core::models::recording::RecordingConfig;
use babydra_core::services::recording::{start_recording, stop_recording, toggle_pause};

/// Dispatches starting screen recording.
pub fn start_recording_action(config: &RecordingConfig) {
    let _ = start_recording(config);
}

/// Dispatches stopping screen recording cleanly on a background worker thread.
pub fn stop_recording_action() {
    std::thread::spawn(|| {
        let _ = stop_recording();
    });
}

/// Dispatches pause / resume toggling.
pub fn toggle_pause_action() {
    let _ = toggle_pause();
}
