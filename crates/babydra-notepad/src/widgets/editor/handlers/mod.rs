//! Event handler coordinator for BabyDra Notepad editor.

use crate::widgets::editor::render::EditorUi;
use crate::widgets::editor::EditorState;
use gtk4::Application;
use std::cell::RefCell;
use std::rc::Rc;

pub mod cursor;
pub mod gutter;
pub mod keys;
pub mod text_change;

/// Attaches all UI and keyboard event handlers to the editor session.
pub fn setup_editor_handlers(state: &Rc<RefCell<EditorState>>, ui: &EditorUi, app: &Application) {
    gutter::setup_gutter_drawing(
        &ui.line_numbers,
        &ui.text_view,
        &ui.text_buffer,
        &ui.scrolled_window,
    );
    cursor::setup_cursor_handler(ui);
    text_change::setup_text_change_handler(state, ui);
    keys::setup_key_controller(state, ui, app);
}
