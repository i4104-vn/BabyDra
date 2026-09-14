//! Shared helpers for the panel's mutually-exclusive auxiliary windows.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) type WindowSlot = Rc<RefCell<Option<gtk4::ApplicationWindow>>>;

/// Closes the window currently stored in `slot`, if one exists.
pub(crate) fn close_window(slot: &WindowSlot) {
    if let Some(window) = slot.borrow().clone() {
        window.close();
    }
}

/// Closes an existing window or creates and stores its replacement.
///
/// The factory owns presentation because some panel windows animate themselves
/// as part of their construction while others need an explicit `present` call.
pub(crate) fn toggle_window(slot: &WindowSlot, create: impl FnOnce() -> gtk4::ApplicationWindow) {
    if slot.borrow().is_some() {
        close_window(slot);
        return;
    }

    let window = create();
    if let Ok(mut slot) = slot.try_borrow_mut() {
        *slot = Some(window);
    }
}
