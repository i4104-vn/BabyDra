//! Background services for the clipboard island feature.

pub mod dbus;

use std::cell::RefCell;

thread_local! {
    static TRIGGER_CALLBACK: RefCell<Option<Box<dyn Fn()>>> = const { RefCell::new(None) };
}

/// Registers the trigger callback invoked whenever the clipboard shortcut / IPC is triggered.
pub fn set_trigger_callback<F>(cb: F)
where
    F: Fn() + 'static,
{
    TRIGGER_CALLBACK.with(|tc| {
        *tc.borrow_mut() = Some(Box::new(cb));
    });
}

pub(crate) fn fire_trigger() {
    TRIGGER_CALLBACK.with(|tc| {
        if let Ok(guard) = tc.try_borrow() {
            if let Some(cb) = guard.as_ref() {
                cb();
            }
        }
    });
}

/// Initializes clipboard background services (watcher + D-Bus server).
pub fn init_clipboard_services() {
    // 1. Spawns the system clipboard watcher thread
    babydra_core::spawn_clipboard_watcher();

    // 2. Starts the D-Bus service org.babydra.Island
    dbus::spawn_island_dbus();
}
