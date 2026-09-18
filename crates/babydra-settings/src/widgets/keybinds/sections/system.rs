//! BabyDra Crates System Keymaps section builder.

use crate::widgets::keybinds::rows::system::create_system_shortcut_row;
use babydra_core::models::shortcut::SystemShortcut;
use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

/// Builds System Keymaps section: Category header row + table of rows.
pub fn build_system_section(system_shortcuts: &[SystemShortcut]) -> (Box, Box) {
    let header_row = Box::new(Orientation::Horizontal, 0);
    header_row.add_css_class("wifi-category-header-row");
    header_row.set_margin_top(14);
    header_row.set_margin_bottom(8);
    header_row.set_margin_start(12);
    header_row.set_margin_end(12);

    let title_lbl = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_system_title",
    )));
    title_lbl.add_css_class("wifi-category-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let table = Box::new(Orientation::Vertical, 0);
    table.add_css_class("keybinds-table");

    for sc in system_shortcuts {
        let row = create_system_shortcut_row(sc);
        table.append(&row);
    }

    (header_row, table)
}
