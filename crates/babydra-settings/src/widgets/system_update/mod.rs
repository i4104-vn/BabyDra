pub mod handler;
pub mod render;

use gtk4::Widget;

/// Creates a new `system update widget`.
pub fn create_update_widget() -> Widget {
    let (widget, auth_dialog) = render::build(&[]);

    // Wire events using reusable PasswordDialog
    handler::wire_events(&widget, auth_dialog);

    widget.root.into()
}
