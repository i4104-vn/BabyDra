//! Factory Reset / Recovery settings card.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds the Factory Reset collapsible card and mounts its modal overlays.
pub fn build_recovery_card(overlay: &gtk4::Overlay) -> GtkBox {
    let widgets = render::render_recovery_card();

    // Mount Auth Modal and Console Modal overlays on top-level Overlay
    overlay.add_overlay(&widgets.auth_modal_overlay);
    overlay.add_overlay(&widgets.console_modal_overlay);

    handler::wire_events(&widgets);

    widgets.container
}
