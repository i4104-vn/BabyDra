//! Background poller monitoring the core recording service.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub use crate::features::recording::models::IslandRecordingState;

/// Spawns a background thread that polls the in-process recording service.
pub fn spawn_recording_polling() -> Rc<RefCell<IslandRecordingState>> {
    let state_rc = Rc::new(RefCell::new(IslandRecordingState::default()));

    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<IslandRecordingState>();

    std::thread::Builder::new()
        .name("island-recorder-poll".into())
        .spawn(move || loop {
            let mut state = IslandRecordingState::default();

            if babydra_core::services::recording::is_recording() {
                state.is_recording = true;
                state.is_paused = babydra_core::services::recording::is_paused();
                state.elapsed_secs = babydra_core::services::recording::get_elapsed_secs();
                state.is_audio_muted =
                    babydra_core::services::recording::is_recording_audio_muted();
                state.is_mic_muted = babydra_core::services::recording::is_recording_mic_muted();
                if let Some(cfg) = babydra_core::services::recording::get_active_config() {
                    state.audio_enabled = cfg.audio;
                    state.mode_name = match cfg.mode {
                        babydra_core::models::recording::RecordingMode::Fullscreen => {
                            "Fullscreen".to_string()
                        }
                        babydra_core::models::recording::RecordingMode::SingleOutput(ref o) => {
                            format!("Display ({o})")
                        }
                        babydra_core::models::recording::RecordingMode::Area { .. } => {
                            "Area".to_string()
                        }
                        babydra_core::models::recording::RecordingMode::Window(_) => {
                            "Window / Region".to_string()
                        }
                    };
                    state.format = cfg.format.to_uppercase();
                    state.framerate = cfg.framerate;
                }
            }

            if sender.send(state).is_err() {
                break;
            }

            std::thread::sleep(Duration::from_millis(500));
        })
        .ok();

    let cache = state_rc.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Some(st) = receiver.recv().await {
            *cache.borrow_mut() = st;
        }
    });

    state_rc
}
