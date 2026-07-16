use gtk4::prelude::*;

/// Creates a placeholder label.
pub fn create_placeholder_message(text: &str) -> gtk4::Label {
    let placeholder = gtk4::Label::new(Some(text));
    placeholder.add_css_class("settings-desc");
    placeholder.set_margin_top(20);
    placeholder.set_margin_bottom(20);
    placeholder
}
