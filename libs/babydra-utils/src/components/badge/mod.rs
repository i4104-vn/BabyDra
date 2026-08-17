use gtk4::prelude::*;

/// Creates a status indicator label/badge.
#[deprecated(note = "unused — remove in v2; use success-text / settings-desc classes directly")]
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

/// Creates a standard icon badge containing a GTK icon widget.
pub fn create_icon_badge(icon_name: &str, icon_size: i32, is_small: bool) -> gtk4::Box {
    let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    if is_small {
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_halign(gtk4::Align::Start);
    } else {
        icon_badge.add_css_class("blue-icon-badge");
        icon_badge.set_size_request(44, 44);
        icon_badge.set_halign(gtk4::Align::Center);
    }
    icon_badge.set_valign(gtk4::Align::Center);

    let icon = crate::ui::icon::get_icon(icon_name, icon_size);
    icon.set_pixel_size(icon_size);
    icon.set_valign(gtk4::Align::Center);
    icon.set_halign(gtk4::Align::Center);
    icon.set_vexpand(true);
    icon_badge.append(&icon);
    icon_badge
}
