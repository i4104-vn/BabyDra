//! BabyDra Crates System Keymaps section builder.

use crate::widgets::keybinds::rows::system::create_system_shortcut_row;
use babydra_core::models::shortcut::SystemShortcut;
use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

/// Builds Area 1: BabyDra Crates System Keymaps card container.
pub fn build_system_section(system_shortcuts: &[SystemShortcut]) -> (Box, Box) {
    let section = Box::new(Orientation::Vertical, 8);

    let header_box = Box::new(Orientation::Vertical, 2);
    let title = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_system_title",
    )));
    title.add_css_class("settings-section-title");
    title.set_halign(gtk4::Align::Start);

    let desc = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_system_desc",
    )));
    desc.add_css_class("settings-page-subtitle");
    desc.set_halign(gtk4::Align::Start);

    header_box.append(&title);
    header_box.append(&desc);
    section.append(&header_box);

    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");

    let table = Box::new(Orientation::Vertical, 4);
    table.add_css_class("keybinds-table");

    for sc in system_shortcuts {
        let row = create_system_shortcut_row(sc);
        table.append(&row);
    }

    glass_card.append(&table);
    section.append(&glass_card);

    (section, table)
}
