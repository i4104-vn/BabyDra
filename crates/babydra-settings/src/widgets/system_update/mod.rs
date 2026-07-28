pub mod handler;
pub mod render;

use gtk4::Widget;

pub fn create_system_update_widget() -> Widget {
    // Build initial UI layout
    let widget = render::build(&[]);

    // Wire events, async update checks, and streaming console
    handler::wire_events(&widget);

    widget.root.into()
}
