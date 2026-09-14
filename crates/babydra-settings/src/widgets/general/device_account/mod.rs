//! Device & Account Settings module in General Settings.

pub mod handler;
pub mod render;

use gtk4::Box as GtkBox;

/// Builds and wires the Device & Account collapsible card.
pub fn build_device_account_card() -> GtkBox {
    let widgets = render::render_device_account_card();
    handler::wire_events(&widgets);
    widgets.container
}
