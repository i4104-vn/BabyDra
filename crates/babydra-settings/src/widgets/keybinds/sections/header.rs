//! Page header with title, refresh button, and save changes button.

use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation};

/// Builds the top header bar: Title, Refresh button, and Save button.
pub fn build_page_header() -> (Box, Button, Button) {
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_title_page",
    )));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label(&babydra_core::i18n::trans("settings.refresh"));
    refresh_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label(&babydra_core::i18n::trans("settings.save_changes"));
    save_btn.add_css_class("suggested-action");

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    header_box.append(&save_btn);

    (header_box, refresh_btn, save_btn)
}
