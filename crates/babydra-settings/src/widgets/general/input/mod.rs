//! Microphone / Audio Input settings module.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the Microphone collapsible card.
pub fn build_input_card() -> GtkBox {
    let (widgets, devices) = render::render_input_card();
    handler::wire_events(&widgets, devices);
    widgets.container
}
