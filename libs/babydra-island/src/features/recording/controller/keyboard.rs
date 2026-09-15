//! Keyboard navigation and shortcut event controller for the recording popover.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk::Key;
use gtk4::glib;

use crate::features::recording::ui::RecordingPopover;
use crate::island::IslandViewHandle;

/// Creates an event controller key that handles Escape and key events on the recording popover.
pub fn create_keyboard_controller(
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    popover_rc: Rc<RefCell<Option<RecordingPopover>>>,
) -> gtk4::EventControllerKey {
    let key_ctrl = gtk4::EventControllerKey::new();

    key_ctrl.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == Key::Escape {
            if let Some(p) = popover_rc.borrow().as_ref() {
                p.popdown();
            }
            if let Some(h) = handle_rc.borrow().as_ref() {
                if !babydra_core::services::recording::is_recording() {
                    h.release_override();
                    h.hide();
                }
            }
            crate::island::dismiss_all_popovers();
            return glib::Propagation::Stop;
        }

        let is_open = popover_rc
            .borrow()
            .as_ref()
            .map(|p| p.is_visible())
            .unwrap_or(false);
        if !is_open {
            return glib::Propagation::Proceed;
        }

        glib::Propagation::Proceed
    });

    key_ctrl
}
