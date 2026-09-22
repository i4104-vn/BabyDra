use crate::widgets::editor::render::EditorUi;
use babydra_core::models::notepad::CursorPosition;
use gtk4::prelude::*;

/// Registers notifications on cursor movement to update line/col numbers and line highlight.
pub fn setup_cursor_handler(ui: &EditorUi) {
    let status_cursor_lbl = ui.status_cursor_lbl.clone();
    let line_numbers = ui.line_numbers.clone();
    let text_buffer = ui.text_buffer.clone();

    text_buffer.connect_notify_local(Some("cursor-position"), move |tb, _| {
        let cursor_iter = tb.iter_at_mark(&tb.get_insert());
        let pos = CursorPosition::new(
            cursor_iter.line() as usize + 1,
            cursor_iter.line_offset() as usize + 1,
        );
        status_cursor_lbl.set_text(&pos.display_string());
        line_numbers.queue_draw();
    });
}
