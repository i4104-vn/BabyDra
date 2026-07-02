//! UI layout renderer for query results sidebars.

use gtk4::prelude::*;

/// Builds a placeholder container showing the initial greeting/welcome message when query is empty.
pub fn build_welcome_layout() -> gtk4::Box {
    let right_content = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    right_content.set_hexpand(true);
    right_content.set_vexpand(true);

    let welcome_label = gtk4::Label::new(Some(&babydra_common::i18n::t("launcher.welcome")));
    welcome_label.add_css_class("launcher-no-results");
    welcome_label.set_halign(gtk4::Align::Center);
    welcome_label.set_valign(gtk4::Align::Center);
    welcome_label.set_vexpand(true);
    welcome_label.set_hexpand(true);
    right_content.append(&welcome_label);

    right_content
}

/// Builds a search row button to launch standard browser searches.
pub fn build_browser_search_button(query: &str) -> (gtk4::Button, gtk4::Label) {
    let browser_btn = gtk4::Button::new();
    browser_btn.add_css_class("launcher-browser-search-row");

    let browser_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    browser_box.set_valign(gtk4::Align::Center);

    let web_icon = babydra_common::icon::get_system_or_file_icon("web-browser", "text-html");
    web_icon.set_pixel_size(20);
    web_icon.set_valign(gtk4::Align::Center);

    let search_fmt = babydra_common::i18n::t("launcher.google_search");
    let browser_lbl = gtk4::Label::new(Some(&search_fmt.replace("{}", query)));
    browser_lbl.set_halign(gtk4::Align::Start);
    browser_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    browser_lbl.set_valign(gtk4::Align::Center);

    browser_box.append(&web_icon);
    browser_box.append(&browser_lbl);
    browser_btn.set_child(Some(&browser_box));

    (browser_btn, browser_lbl)
}

/// Builds a vertical layout box containing files search result entries.
pub fn build_results_layout() -> (gtk4::Box, gtk4::Box) {
    let right_content = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    right_content.set_hexpand(true);
    right_content.set_vexpand(true);

    let files_placeholder = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    right_content.append(&files_placeholder);

    (right_content, files_placeholder)
}

/// Builds a label title block for the files result group.
pub fn build_files_title() -> gtk4::Label {
    let files_title = gtk4::Label::new(Some(&babydra_common::i18n::t("launcher.files")));
    files_title.add_css_class("launcher-section-title");
    files_title.set_halign(gtk4::Align::Start);
    files_title
}

