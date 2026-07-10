use gtk4::prelude::*;

/// Creates a sidebar navigation row box.
pub fn create_sidebar_row(label: &str, icon_name: &str) -> gtk4::Box {
    let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    row_box.add_css_class("settings-sidebar-row");
    let icon = gtk4::Image::from_icon_name(icon_name);
    row_box.append(&icon);
    let lbl = gtk4::Label::new(Some(label));
    row_box.append(&lbl);
    row_box
}
