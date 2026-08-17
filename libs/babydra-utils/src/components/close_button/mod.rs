use gtk4::prelude::*;

/// Creates a standardized icon-only close button.
#[deprecated(note = "unused — remove in v2; use create_icon_button")]
pub fn create_close_button(css_class: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
    } else {
        btn.add_css_class("close-btn");
    }
    btn.set_cursor_from_name(Some("pointer"));
    let icon = crate::ui::icon::get_system_or_file_icon("window-close", "window-close");
    icon.set_pixel_size(12);
    btn.set_child(Some(&icon));
    btn
}

/// Creates a close button with a label and an icon.
#[deprecated(note = "unused — remove in v2; use create_icon_button")]
pub fn create_close_button_with_label(label_text: &str, css_class: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
    } else {
        btn.add_css_class("close-btn");
    }
    btn.set_cursor_from_name(Some("pointer"));

    let close_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let close_icon = crate::ui::icon::get_system_or_file_icon("window-close", "window-close");
    close_icon.set_pixel_size(12);
    let close_label = gtk4::Label::new(Some(label_text));
    close_content.append(&close_icon);
    close_content.append(&close_label);
    btn.set_child(Some(&close_content));
    btn
}
