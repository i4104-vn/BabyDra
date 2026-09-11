//! Keyboard navigation event controller for the power popover.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::gdk::Key;
use gtk4::glib;

use crate::features::power::ui::{execute_power_action, highlight_selection, PowerPopover};
use crate::island::IslandViewHandle;

/// Creates an event controller key that navigates and triggers power options.
pub fn create_keyboard_controller(
    selected: Rc<Cell<usize>>,
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    popover_rc: Rc<RefCell<Option<PowerPopover>>>,
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

        match keyval {
            Key::Left | Key::Up | Key::h | Key::k | Key::ISO_Left_Tab => {
                let cur = selected.get();
                let prev = if cur == 0 { 3 } else { cur - 1 };
                selected.set(prev);
                if let Some(p) = popover_rc.borrow().as_ref() {
                    highlight_selection(p, prev);
                }
                glib::Propagation::Stop
            }
            Key::Right | Key::Down | Key::l | Key::j | Key::Tab => {
                let cur = selected.get();
                let next = (cur + 1) % 4;
                selected.set(next);
                if let Some(p) = popover_rc.borrow().as_ref() {
                    highlight_selection(p, next);
                }
                glib::Propagation::Stop
            }
            Key::Return | Key::KP_Enter | Key::space => {
                let idx = selected.get();
                if let Some(p) = popover_rc.borrow().as_ref() {
                    let handle = handle_rc.borrow();
                    execute_power_action(idx, p, handle.as_ref());
                }
                glib::Propagation::Stop
            }
            Key::_1 | Key::KP_1 => {
                selected.set(0);
                if let Some(p) = popover_rc.borrow().as_ref() {
                    let handle = handle_rc.borrow();
                    execute_power_action(0, p, handle.as_ref());
                }
                glib::Propagation::Stop
            }
            Key::_2 | Key::KP_2 => {
                selected.set(1);
                if let Some(p) = popover_rc.borrow().as_ref() {
                    let handle = handle_rc.borrow();
                    execute_power_action(1, p, handle.as_ref());
                }
                glib::Propagation::Stop
            }
            Key::_3 | Key::KP_3 => {
                selected.set(2);
                if let Some(p) = popover_rc.borrow().as_ref() {
                    let handle = handle_rc.borrow();
                    execute_power_action(2, p, handle.as_ref());
                }
                glib::Propagation::Stop
            }
            Key::_4 | Key::KP_4 => {
                selected.set(3);
                if let Some(p) = popover_rc.borrow().as_ref() {
                    let handle = handle_rc.borrow();
                    execute_power_action(3, p, handle.as_ref());
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
            _ => glib::Propagation::Proceed,
        }
    });

    key_ctrl
}
