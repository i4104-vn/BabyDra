use gtk4::prelude::*;

/// Shorthand helper to set tooltip text on a widget.
pub fn set_tooltip(widget: &impl IsA<gtk4::Widget>, text: &str) {
    widget.set_tooltip_text(Some(text));
}
