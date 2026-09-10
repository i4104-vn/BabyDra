//! Default Applications settings module.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the Default Applications collapsible card.
pub fn build_default_apps_card() -> GtkBox {
    let (widgets, browsers, file_managers, terminals) = render::render_default_apps_card();
    handler::wire_events(&widgets, browsers, file_managers, terminals);
    widgets.container
}
