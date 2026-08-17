pub mod battery_card;
pub mod handler;
pub mod render;

use gtk4::Widget;

/// Creates a new `power widget`.
pub fn create_power_widget() -> Widget {
    let (widget, auth_dialog) = render::build();
    handler::wire_events(&widget, auth_dialog);
    widget.root.into()
}
