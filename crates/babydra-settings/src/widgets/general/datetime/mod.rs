//! Date & Time Settings module in General Settings.

pub mod controls;
pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the Date & Time collapsible card.
pub fn build_datetime_card() -> GtkBox {
    let widgets = render::render_datetime_card();
    handler::wire_events(&widgets);
    widgets.container
}
