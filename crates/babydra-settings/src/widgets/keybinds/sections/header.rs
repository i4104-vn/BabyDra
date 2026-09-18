//! Page header with title, search box, add button, refresh button, and save changes button.

use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation};

/// Builds the top header bar matching Wi-Fi layout:
/// Title on Left, Search + Add + Refresh + Save on Right.
pub fn build_page_header() -> (Box, Entry, Button, Button, Button) {
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let title_label = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_title_page",
    )));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);
    header_box.append(&title_label);

    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some(&babydra_core::i18n::trans(
        "settings.keybinds_search_placeholder",
    )));
    search_entry.add_css_class("sidebar-search-entry");
    search_entry.set_width_request(200);
    header_box.append(&search_entry);

    let add_btn = Button::with_label(&babydra_core::i18n::trans("settings.startup_add_new"));
    add_btn.add_css_class("connect-pill-btn");
    add_btn.set_valign(gtk4::Align::Center);
    header_box.append(&add_btn);

    let refresh_btn = Button::new();
    refresh_btn.add_css_class("icon-btn");
    refresh_btn.add_css_class("circular");
    refresh_btn.set_cursor_from_name(Some("pointer"));
    let refresh_icon = babydra_ui_kit::ui::icon::get_icon("refresh", 16);
    refresh_icon.set_pixel_size(16);
    refresh_btn.set_child(Some(&refresh_icon));
    refresh_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.refresh")));
    header_box.append(&refresh_btn);

    let save_btn = Button::with_label(&babydra_core::i18n::trans("settings.save_changes"));
    save_btn.add_css_class("suggested-action");
    save_btn.set_valign(gtk4::Align::Center);
    header_box.append(&save_btn);

    (header_box, search_entry, add_btn, refresh_btn, save_btn)
}
