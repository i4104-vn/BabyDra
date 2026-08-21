//! Keyboard shortcut parsing and registration utilities.

use gtk4::prelude::*;
use std::rc::Rc;

pub struct KeyShortcut {
    pub keyval: gtk4::gdk::Key,
    pub modifiers: gtk4::gdk::ModifierType,
    pub callback: Rc<dyn Fn()>,
}

/// Parses a shortcut string like "Ctrl+Shift+T" into a (Key, ModifierType) pair.
pub fn parse_shortcut(shortcut_str: &str) -> Option<(gtk4::gdk::Key, gtk4::gdk::ModifierType)> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();
    let mut modifiers = gtk4::gdk::ModifierType::empty();
    let mut key = None;

    for part in parts {
        let part_lower = part.to_lowercase();
        if part_lower == "ctrl" || part_lower == "control" {
            modifiers |= gtk4::gdk::ModifierType::CONTROL_MASK;
        } else if part_lower == "shift" {
            modifiers |= gtk4::gdk::ModifierType::SHIFT_MASK;
        } else if part_lower == "alt" {
            modifiers |= gtk4::gdk::ModifierType::ALT_MASK;
        } else {
            let k = match part_lower.as_str() {
                "f1" => Some(gtk4::gdk::Key::F1),
                "f2" => Some(gtk4::gdk::Key::F2),
                "f3" => Some(gtk4::gdk::Key::F3),
                "f4" => Some(gtk4::gdk::Key::F4),
                "f5" => Some(gtk4::gdk::Key::F5),
                "f6" => Some(gtk4::gdk::Key::F6),
                "f7" => Some(gtk4::gdk::Key::F7),
                "f8" => Some(gtk4::gdk::Key::F8),
                "f9" => Some(gtk4::gdk::Key::F9),
                "f10" => Some(gtk4::gdk::Key::F10),
                "f11" => Some(gtk4::gdk::Key::F11),
                "f12" => Some(gtk4::gdk::Key::F12),
                "enter" => Some(gtk4::gdk::Key::Return),
                "space" => Some(gtk4::gdk::Key::space),
                "escape" | "esc" => Some(gtk4::gdk::Key::Escape),
                "delete" | "del" => Some(gtk4::gdk::Key::Delete),
                "backspace" => Some(gtk4::gdk::Key::BackSpace),
                s if s.len() == 1 => {
                    let c = s.chars().next().unwrap();
                    gtk4::gdk::Key::from_name(&c.to_string())
                }
                s => gtk4::gdk::Key::from_name(s),
            };
            if let Some(keyval) = k {
                key = Some(keyval);
            }
        }
    }

    key.map(|k| (k, modifiers))
}

/// Registers a list of shortcuts to the window, returns the created controller.
pub fn setup_key_shortcuts(
    window: &gtk4::ApplicationWindow,
    shortcuts: Vec<KeyShortcut>,
) -> gtk4::EventControllerKey {
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    let win = window.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        // If an entry or editable text field is currently focused, allow normal typing
        if let Some(focus) = gtk4::prelude::GtkWindowExt::focus(&win) {
            if focus.downcast_ref::<gtk4::Editable>().is_some() {
                return glib::Propagation::Proceed;
            }
        }

        let clean_state = state
            & (gtk4::gdk::ModifierType::CONTROL_MASK
                | gtk4::gdk::ModifierType::SHIFT_MASK
                | gtk4::gdk::ModifierType::ALT_MASK);

        for shortcut in &shortcuts {
            let key_matches = shortcut.keyval == keyval
                || shortcut.keyval.to_lower() == keyval.to_lower()
                || (shortcut.keyval == gtk4::gdk::Key::Delete
                    && keyval == gtk4::gdk::Key::KP_Delete)
                || (shortcut.keyval == gtk4::gdk::Key::Return
                    && keyval == gtk4::gdk::Key::KP_Enter);

            if key_matches && shortcut.modifiers == clean_state {
                (shortcut.callback)();
                return glib::Propagation::Stop;
            }
        }
        glib::Propagation::Proceed
    });
    window.add_controller(key_controller.clone());
    key_controller
}
