//! Custom Shortcuts section builder with editable rows and add button.

use crate::widgets::keybinds::rows::custom::create_custom_shortcut_row;
use babydra_core::models::shortcut::Shortcut;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation};

/// Builds Area 2: Custom Shortcuts card container with "+ Add" button.
pub fn build_custom_section(custom_shortcuts: &[Shortcut]) -> (Box, Box, Button) {
    let section = Box::new(Orientation::Vertical, 8);

    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_vbox = Box::new(Orientation::Vertical, 2);
    title_vbox.set_hexpand(true);

    let title = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_custom_title",
    )));
    title.add_css_class("settings-section-title");
    title.set_halign(gtk4::Align::Start);

    let desc = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_custom_desc",
    )));
    desc.add_css_class("settings-page-subtitle");
    desc.set_halign(gtk4::Align::Start);

    title_vbox.append(&title);
    title_vbox.append(&desc);

    let add_btn = Button::with_label(&babydra_core::i18n::trans("settings.startup_add_new"));
    add_btn.add_css_class("connect-pill-btn");
    add_btn.set_valign(gtk4::Align::Center);

    header_box.append(&title_vbox);
    header_box.append(&add_btn);
    section.append(&header_box);

    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");

    let table = Box::new(Orientation::Vertical, 4);
    table.add_css_class("keybinds-table");

    for sc in custom_shortcuts {
        let row = create_custom_shortcut_row(sc, table.clone());
        table.append(&row);
    }

    glass_card.append(&table);
    section.append(&glass_card);

    (section, table, add_btn)
}
