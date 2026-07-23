use gtk4::prelude::*;

/// Creates a sidebar navigation row box.
pub fn create_sidebar_row(label: &str, icon_name: &str) -> gtk4::Box {
    create_sidebar_row_with_badge(label, icon_name, "badge-slate")
}

/// Creates a sidebar navigation row box with custom icon badge class.
pub fn create_sidebar_row_with_badge(label: &str, icon_name: &str, badge_class: &str) -> gtk4::Box {
    let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    row_box.add_css_class("settings-sidebar-row");

    let badge_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    badge_box.add_css_class("sidebar-icon-badge");
    badge_box.add_css_class(badge_class);
    badge_box.set_valign(gtk4::Align::Center);

    let icon = crate::ui::icon::get_icon(icon_name, 16);
    icon.set_pixel_size(16);
    badge_box.append(&icon);
    row_box.append(&badge_box);

    let lbl = gtk4::Label::new(Some(label));
    lbl.add_css_class("sidebar-row-label");
    lbl.set_valign(gtk4::Align::Center);
    row_box.append(&lbl);

    row_box
}

