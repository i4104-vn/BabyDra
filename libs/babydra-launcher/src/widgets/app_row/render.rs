//! UI layout renderer for launcher application rows and grids.

use crate::models::DesktopApp;
use gtk4::prelude::*;

/// Builds a grid item button widget displaying an application icon and truncated label.
pub fn build_grid_app_ui(app: &DesktopApp) -> (gtk4::Button, gtk4::Box, gtk4::Label) {
    let btn = gtk4::Button::new();
    btn.add_css_class("launcher-grid-item");
    btn.set_tooltip_text(Some(&app.name));
    btn.set_cursor_from_name(Some("pointer"));

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
    btn.set_cursor_from_name(Some("pointer"));

    let content_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    content_box.set_valign(gtk4::Align::Center);

    // Icon wrapper
    let icon_wrapper = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_wrapper.add_css_class("app-icon-wrapper");
    icon_wrapper.set_size_request(42, 42);
    icon_wrapper.set_halign(gtk4::Align::Center);
    icon_wrapper.set_valign(gtk4::Align::Center);

    let icon_widget = babydra_common::icon::get_system_or_file_icon(
        app.icon.as_deref().unwrap_or(""),
        "application-x-executable",
    );
    icon_widget.set_pixel_size(24);
    icon_widget.set_halign(gtk4::Align::Center);
    icon_widget.set_valign(gtk4::Align::Center);
    icon_wrapper.append(&icon_widget);

    // App info box
    let info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    info_box.add_css_class("app-info");
    info_box.set_hexpand(true);
    info_box.set_valign(gtk4::Align::Center);

    // Title row
    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    title_row.add_css_class("app-title-row");

    let name_label = gtk4::Label::new(Some(&app.name));
    name_label.set_halign(gtk4::Align::Start);
    name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    name_label.set_max_width_chars(25);
    name_label.add_css_class("app-title");

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    let is_flatpak = app.exec.contains("flatpak");
    let badge_text = if is_flatpak { "Flatpak" } else { "System" };
    let badge_label = gtk4::Label::new(Some(badge_text));
    badge_label.add_css_class("item-badge");
    if is_flatpak {
        badge_label.add_css_class("flatpak");
    } else {
        badge_label.add_css_class("system");
    }

    title_row.append(&name_label);
    title_row.append(&spacer);
    title_row.append(&badge_label);

    // Description row
    let exec_bin = app.exec.split_whitespace().next().unwrap_or("").split('/').last().unwrap_or("");
    let desc_text = if is_flatpak {
        format!("Flatpak App • {}", exec_bin)
    } else {
        format!("System App • {}", exec_bin)
    };
    let desc_label = gtk4::Label::new(Some(&desc_text));
    desc_label.set_halign(gtk4::Align::Start);
    desc_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    desc_label.add_css_class("app-desc");

    info_box.append(&title_row);
    info_box.append(&desc_label);

    content_box.append(&icon_wrapper);
    content_box.append(&info_box);
    btn.set_child(Some(&content_box));

    (btn, content_box, name_label)
}
