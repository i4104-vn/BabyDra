use gtk4::prelude::*;
use gtk4::TextView;

pub use babydra_core::services::explore::shell_quote;

/// Scroll TextView buffer to the bottom.
pub fn scroll_to_end(text_view: &TextView) {
    let buffer = text_view.buffer();
    let mark = buffer.create_mark(None, &buffer.end_iter(), false);
    text_view.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
    buffer.delete_mark(&mark);
}
