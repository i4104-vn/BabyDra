//! Page header with title, add button, refresh button, and save changes button.

use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation};

/// Builds the top header bar:
/// Title on Left, Add (circular icon) + Refresh (circular icon) + Save on Right.
pub fn build_page_header() -> (Box, Button, Button, Button) {
    let header_box = Box::new(Orientation::Horizontal, 10);
    header_box.set_margin_bottom(4);

    let title_label = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_title_page",
    )));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);
    header_box.append(&title_label);

    let add_btn = Button::new();
    add_btn.add_css_class("icon-btn");
    add_btn.add_css_class("circular");
    add_btn.set_cursor_from_name(Some("pointer"));
    add_btn.set_valign(gtk4::Align::Center);
    add_btn.set_size_request(34, 34);
    let add_icon = babydra_ui_kit::ui::icon::get_icon("plus", 16);
    add_icon.set_pixel_size(16);
    add_btn.set_child(Some(&add_icon));
    add_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.startup_add_new")));
    header_box.append(&add_btn);

    let refresh_btn = Button::new();
    refresh_btn.add_css_class("icon-btn");
    refresh_btn.add_css_class("circular");
    refresh_btn.set_cursor_from_name(Some("pointer"));
    refresh_btn.set_valign(gtk4::Align::Center);
    refresh_btn.set_size_request(34, 34);
    let refresh_icon = babydra_ui_kit::ui::icon::get_icon("refresh", 16);
    refresh_icon.set_pixel_size(16);
    refresh_btn.set_child(Some(&refresh_icon));
    refresh_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.refresh")));
    header_box.append(&refresh_btn);

    let save_btn = Button::with_label(&babydra_core::i18n::trans("settings.save_changes"));
    save_btn.add_css_class("suggested-action");
    save_btn.set_valign(gtk4::Align::Center);
    header_box.append(&save_btn);

    (header_box, add_btn, refresh_btn, save_btn)
}
