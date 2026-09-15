//! Island recording background service modules.

pub mod actions;
pub mod polling;

pub use actions::{toggle_audio_mute, toggle_mic_mute};
pub use polling::{spawn_recording_polling, IslandRecordingState};

use std::cell::RefCell;

thread_local! {
    static TRIGGER_CALLBACK: RefCell<Option<Box<dyn Fn()>>> = const { RefCell::new(None) };
}

/// Registers the callback used by the global shortcut and island IPC command.
pub fn set_trigger_callback<F>(callback: F)
where
    F: Fn() + 'static,
{
    TRIGGER_CALLBACK.with(|slot| *slot.borrow_mut() = Some(Box::new(callback)));
}

pub(crate) fn fire_trigger() {
    TRIGGER_CALLBACK.with(|slot| {
        if let Ok(callback) = slot.try_borrow() {
            if let Some(callback) = callback.as_ref() {
                callback();
            }
        }
    });
}
