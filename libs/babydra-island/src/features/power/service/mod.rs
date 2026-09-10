//! Background service for the power island feature.

use std::cell::RefCell;

thread_local! {
    static POWER_TRIGGER_CALLBACK: RefCell<Option<Box<dyn Fn()>>> = const { RefCell::new(None) };
}

/// Registers the trigger callback invoked whenever the power shortcut (Win+F4) or IPC is fired.
pub fn set_trigger_callback<F>(cb: F)
where
    F: Fn() + 'static,
{
    POWER_TRIGGER_CALLBACK.with(|tc| {
        *tc.borrow_mut() = Some(Box::new(cb));
    });
}

pub(crate) fn fire_trigger() {
    POWER_TRIGGER_CALLBACK.with(|tc| {
        if let Some(cb) = tc.borrow().as_ref() {
            cb();
        }
    });
}
