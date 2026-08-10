pub mod handler;
pub mod render;

use gtk4::Widget;

pub fn create_system_update_widget() -> Widget {
    // Build initial UI layout and PasswordDialog overlay
    let (widget, auth_dialog) = render::build(&[]);

    // Wire events using reusable PasswordDialog
    handler::wire_events(&widget, auth_dialog);

    widget.root.into()
}
