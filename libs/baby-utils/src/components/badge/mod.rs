use gtk4::prelude::*;

/// Creates a status indicator label/badge.
pub fn create_status_badge(text: &str, is_success: bool) -> gtk4::Label {
    let lbl = gtk4::Label::new(Some(text));
    if is_success {
        lbl.add_css_class("success-text");
    } else {
        lbl.add_css_class("settings-desc");
    }
    lbl.set_valign(gtk4::Align::Center);
    lbl
}
