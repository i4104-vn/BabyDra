//! Keyboard shortcut parsing and registration utilities.

use gtk4::prelude::*;
use std::rc::Rc;
pub struct KeyShortcut {
    pub keyval: gtk4::gdk::Key,
    pub modifiers: gtk4::gdk::ModifierType,
    pub callback: Rc<dyn Fn()>,
}

pub use babydra_core::parse_shortcut;

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

        let clean_state = babydra_core::clean_modifiers(state);

        for shortcut in &shortcuts {
            if babydra_core::matches_key(keyval, clean_state, (shortcut.keyval, shortcut.modifiers)) {
                (shortcut.callback)();
                return glib::Propagation::Stop;
            }
        }
        glib::Propagation::Proceed
    });
    window.add_controller(key_controller.clone());
    key_controller
}
