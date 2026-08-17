//! UI layout renderer for query results sidebars.

use gtk4::prelude::*;

/// Builds a search row button to launch standard browser searches.
pub fn build_browser_search_button(query: &str) -> (gtk4::Button, gtk4::Label) {
    let browser_btn = gtk4::Button::new();
    browser_btn.add_css_class("launcher-list-item");
    browser_btn.set_cursor_from_name(Some("pointer"));

    let content_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    content_box.set_valign(gtk4::Align::Center);

    // Icon wrapper
    let icon_wrapper = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_wrapper.add_css_class("app-icon-wrapper");
    icon_wrapper.set_size_request(42, 42);
    icon_wrapper.set_halign(gtk4::Align::Center);
    icon_wrapper.set_valign(gtk4::Align::Center);

    let web_icon = babydra_ui_kit::ui::icon::get_system_or_file_icon("web-browser", "text-html");
    web_icon.set_pixel_size(24);
    web_icon.set_halign(gtk4::Align::Center);
    web_icon.set_valign(gtk4::Align::Center);
    icon_wrapper.append(&web_icon);

    // Info box
    let info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    info_box.add_css_class("app-info");
    info_box.set_hexpand(true);
    info_box.set_valign(gtk4::Align::Center);

    // Title row
    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    title_row.add_css_class("app-title-row");

    let name_text = format!("Search Google for \"{}\"", query);
    let name_label = gtk4::Label::new(Some(&name_text));
    name_label.set_halign(gtk4::Align::Start);
    name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    name_label.set_max_width_chars(25);
    name_label.add_css_class("app-title");

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    let badge_label = gtk4::Label::new(Some(&babydra_core::i18n::t("launcher.web")));
    badge_label.add_css_class("item-badge");
    badge_label.add_css_class("web");

    title_row.append(&name_label);
    title_row.append(&spacer);
    title_row.append(&badge_label);

    // Description row
    let desc_label = gtk4::Label::new(Some(&babydra_core::i18n::t("launcher.web_desc")));
    desc_label.set_halign(gtk4::Align::Start);
    desc_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    desc_label.add_css_class("app-desc");

    info_box.append(&title_row);
    info_box.append(&desc_label);

    content_box.append(&icon_wrapper);
    content_box.append(&info_box);
    browser_btn.set_child(Some(&content_box));

    (browser_btn, name_label)
}
