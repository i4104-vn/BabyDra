//! Keyboard navigation event controller for the clipboard history popover.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::gdk::Key;
use gtk4::glib;

use crate::features::clipboard::ui::{render_popover, ClipboardPopover};
use crate::island::IslandViewHandle;

/// Creates an event controller key that navigates and selects clipboard entries.
pub fn create_keyboard_controller(
    selected: Rc<Cell<usize>>,
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    popover_rc: Rc<RefCell<Option<ClipboardPopover>>>,
) -> gtk4::EventControllerKey {
    let key_ctrl = gtk4::EventControllerKey::new();

    key_ctrl.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == Key::Escape {
            if let Some(p) = popover_rc.borrow().as_ref() {
                p.popdown();
            }
            if let Some(h) = handle_rc.borrow().as_ref() {
                h.release_override();
                h.hide();
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

        let entries = babydra_core::get_entries();
        let total = entries.len();

        let copy_and_dismiss = |idx: usize| {
            if let Some(entry) = entries.get(idx) {
                let _ = babydra_core::copy_to_system_clipboard(entry);
            }
            if let Some(p) = popover_rc.borrow().as_ref() {
                p.popdown();
            }
            if let Some(h) = handle_rc.borrow().as_ref() {
                h.release_override();
                h.hide();
            }
        };

        match keyval {
            Key::Up | Key::k | Key::ISO_Left_Tab => {
                if total > 0 {
                    let cur = selected.get();
                    let next = if cur == 0 { total - 1 } else { cur - 1 };
                    selected.set(next);
                    if let Some(p) = popover_rc.borrow().as_ref() {
                        render_popover(p, &entries, next);
                    }
                }
                glib::Propagation::Stop
            }
            Key::Down | Key::j | Key::Tab => {
                if total > 0 {
                    let cur = selected.get();
                    let next = (cur + 1) % total;
                    selected.set(next);
                    if let Some(p) = popover_rc.borrow().as_ref() {
                        render_popover(p, &entries, next);
                    }
                }
                glib::Propagation::Stop
            }
            Key::Return | Key::KP_Enter | Key::space => {
                if total > 0 {
                    copy_and_dismiss(selected.get());
                } else {
                    if let Some(p) = popover_rc.borrow().as_ref() {
                        p.popdown();
                    }
                    if let Some(h) = handle_rc.borrow().as_ref() {
                        h.release_override();
                        h.hide();
                    }
                }
                glib::Propagation::Stop
            }
            Key::Escape => {
                if let Some(p) = popover_rc.borrow().as_ref() {
                    p.popdown();
                }
                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.release_override();
                    h.hide();
                }
                glib::Propagation::Stop
            }
            Key::_1 | Key::KP_1 if total > 0 => {
                selected.set(0);
                copy_and_dismiss(0);
                glib::Propagation::Stop
            }
            Key::_2 | Key::KP_2 if total > 1 => {
                selected.set(1);
                copy_and_dismiss(1);
                glib::Propagation::Stop
            }
            Key::_3 | Key::KP_3 if total > 2 => {
                selected.set(2);
                copy_and_dismiss(2);
                glib::Propagation::Stop
            }
            Key::_4 | Key::KP_4 if total > 3 => {
                selected.set(3);
                copy_and_dismiss(3);
                glib::Propagation::Stop
            }
            Key::_5 | Key::KP_5 if total > 4 => {
                selected.set(4);
                copy_and_dismiss(4);
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
        }
    });

    key_ctrl
}
