//! Audio Output settings module.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the Audio Output collapsible card.
pub fn build_output_card() -> GtkBox {
    let (widgets, devices) = render::render_output_card();
    handler::wire_events(&widgets, devices);
    widgets.container
}
