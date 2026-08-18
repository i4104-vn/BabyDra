mod handler;
mod render;

/// Creates a new `certificates widget`.
pub fn create_cert_widget() -> gtk4::Widget {
    let (widget, auth_dialog) = render::build_certificates();
    handler::wire_events(&widget, auth_dialog);
    widget.root.into()
}
