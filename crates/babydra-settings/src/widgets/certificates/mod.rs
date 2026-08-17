mod handler;
mod render;

/// Creates a new `certificates widget`.
pub fn create_certificates_widget() -> gtk4::Widget {
    let (widget, auth_dialog) = render::build_certificates_ui();
    handler::wire_events(&widget, auth_dialog);
    widget.root.into()
}
