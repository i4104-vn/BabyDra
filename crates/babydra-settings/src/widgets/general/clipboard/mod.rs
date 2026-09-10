//! Clipboard Settings module in General Settings.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the Clipboard collapsible card.
pub fn build_clipboard_card() -> GtkBox {
    let widgets = render::render_clipboard_card();
    handler::wire_events(&widgets);
    widgets.container
}
