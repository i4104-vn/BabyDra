mod render;
mod handler;

pub fn create_certificates_widget() -> gtk4::Overlay {
    let (widget, auth_dialog) = render::build_certificates_ui();
    handler::wire_events(&widget, auth_dialog);
    widget.root
}
