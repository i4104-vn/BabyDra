//! Custom Shortcuts section builder with editable rows.

use crate::widgets::keybinds::rows::custom::create_custom_shortcut_row;
use babydra_core::models::shortcut::Shortcut;
use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

/// Builds Custom Shortcuts section: Category header row (with generous 22px margin-top) + table of rows.
pub fn build_custom_section(custom_shortcuts: &[Shortcut]) -> (Box, Box) {
    let header_row = Box::new(Orientation::Horizontal, 0);
    header_row.add_css_class("wifi-category-header-row");
    header_row.set_margin_top(22);
    header_row.set_margin_bottom(8);
    header_row.set_margin_start(12);
    header_row.set_margin_end(12);

    let title_lbl = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_custom_title",
    )));
    title_lbl.add_css_class("wifi-category-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let table = Box::new(Orientation::Vertical, 0);
    table.add_css_class("keybinds-table");

    for sc in custom_shortcuts {
        let row = create_custom_shortcut_row(sc, table.clone());
        table.append(&row);
    }

    (header_row, table)
}
