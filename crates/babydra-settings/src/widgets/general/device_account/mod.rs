//! Device & Account Settings module in General Settings.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the Device & Account collapsible card.
pub fn build_device_account_card(overlay: &gtk4::Overlay) -> GtkBox {
    let widgets = render::render_device_account_card();
    overlay.add_overlay(&widgets.change_hostname_dialog.container);
    overlay.add_overlay(&widgets.change_password_dialog.container);
    handler::wire_events(&widgets);
    widgets.container
}
