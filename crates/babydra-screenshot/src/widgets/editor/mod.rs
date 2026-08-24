//! Screenshot editor window: assembly, keyboard shortcuts, and UI construction.

pub mod canvas;
pub mod clipboard;
pub mod color_popover;
pub mod shape_popover;
mod render;

pub use render::build_editor_ui;

use babydra_core::models::EditorState;
use babydra_core::services::screenshot::trigger_save;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use self::canvas::delete_selected;
use self::clipboard::copy_to_clipboard;

/// Sets up keyboard event controllers to handle global shortcuts like Escape (cancel),
/// Return (copy to clipboard), Ctrl+S (save to file), and Delete (remove selected drawing).
pub fn setup_editor_keys(
    window: &gtk4::ApplicationWindow,
    state: Rc<RefCell<EditorState>>,
    canvas: &gtk4::DrawingArea,
) {
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let state_key = state.clone();
    let win_key = window.clone();
    let canvas_key = canvas.clone();
    key_controller.connect_key_pressed(move |_, key, _, state_flags| match key {
        gtk4::gdk::Key::Escape => {
            win_key.close();
            gtk4::glib::Propagation::Stop
        }
        gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
            if copy_to_clipboard(&state_key.borrow(), &win_key) {
                win_key.close();
            }
            gtk4::glib::Propagation::Stop
        }
        gtk4::gdk::Key::s | gtk4::gdk::Key::S => {
            if state_flags.contains(gtk4::gdk::ModifierType::CONTROL_MASK) {
                if trigger_save(&state_key.borrow()) {
                    win_key.close();
                }
                gtk4::glib::Propagation::Stop
            } else {
                gtk4::glib::Propagation::Proceed
            }
        }
        gtk4::gdk::Key::Delete | gtk4::gdk::Key::BackSpace => {
            let mut s = state_key.borrow_mut();
            delete_selected(&mut s);
            drop(s);
            canvas_key.queue_draw();
            gtk4::glib::Propagation::Stop
        }
        _ => gtk4::glib::Propagation::Proceed,
    });

    window.add_controller(key_controller);
}
