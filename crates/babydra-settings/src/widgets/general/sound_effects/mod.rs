//! System Sound Effects settings module.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the System Sound Effects collapsible card.
pub fn build_sound_effects_card() -> GtkBox {
    let widgets = render::render_sound_effects_card();
    handler::wire_events(&widgets);
    widgets.container
}
