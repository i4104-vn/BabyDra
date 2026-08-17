use gtk4::prelude::*;

/// Creates a spinner.
#[deprecated(note = "unused — remove in v2; prefer gtk4::Spinner directly")]
pub fn create_spinner(size: i32) -> gtk4::Spinner {
    let spinner = gtk4::Spinner::new();
    spinner.set_size_request(size, size);
    spinner.start();
    spinner
}

/// Creates a loading layout with spinner and label.
#[deprecated(note = "unused — remove in v2; use PlaceholderState::Loading")]
pub fn create_loading_box(text: &str) -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    container.set_halign(gtk4::Align::Center);
    container.set_valign(gtk4::Align::Center);

    let spinner = create_spinner(16);
    let label = gtk4::Label::new(Some(text));
    label.add_css_class("settings-desc");

    container.append(&spinner);
    container.append(&label);
    container
}
