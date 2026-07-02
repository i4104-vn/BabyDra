//! UI layout renderer for launcher application rows and grids.

use crate::models::DesktopApp;
use gtk4::prelude::*;

/// Builds a grid item button widget displaying an application icon and truncated label.
pub fn build_grid_app_ui(app: &DesktopApp) -> (gtk4::Button, gtk4::Box, gtk4::Label) {
    let btn = gtk4::Button::new();
    btn.add_css_class("launcher-grid-item");
    btn.set_tooltip_text(Some(&app.name));

    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    content_box.set_halign(gtk4::Align::Center);

    let icon_widget = babydra_common::icon::get_system_or_file_icon(
        app.icon.as_deref().unwrap_or(""),
        "application-x-executable",
    );
    icon_widget.set_pixel_size(40);
    icon_widget.set_halign(gtk4::Align::Center);

    let name_label = gtk4::Label::new(Some(&app.name));
    name_label.set_halign(gtk4::Align::Center);
    name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    name_label.set_max_width_chars(10);
    name_label.add_css_class("launcher-grid-label");

    content_box.append(&icon_widget);
    content_box.append(&name_label);
    btn.set_child(Some(&content_box));

    (btn, content_box, name_label)
}

/// Builds a horizontal list row item button widget displaying an application icon and label.
pub fn build_list_app_ui(app: &DesktopApp) -> (gtk4::Button, gtk4::Box, gtk4::Label) {
    let btn = gtk4::Button::new();
    btn.add_css_class("launcher-list-item");
    btn.set_tooltip_text(Some(&app.name));

    let content_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    content_box.set_valign(gtk4::Align::Center);

    let icon_widget = babydra_common::icon::get_system_or_file_icon(
        app.icon.as_deref().unwrap_or(""),
        "application-x-executable",
    );
    icon_widget.set_pixel_size(24);
    icon_widget.set_valign(gtk4::Align::Center);

    let name_label = gtk4::Label::new(Some(&app.name));
    name_label.set_halign(gtk4::Align::Start);
    name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    name_label.set_max_width_chars(25);
    name_label.add_css_class("launcher-list-label");
    name_label.set_valign(gtk4::Align::Center);

    content_box.append(&icon_widget);
    content_box.append(&name_label);
    btn.set_child(Some(&content_box));

    (btn, content_box, name_label)
}

