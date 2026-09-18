//! Category header row component.

use gtk4::prelude::*;

/// Creates a category header row with a clean text label and generous section margin.
pub fn create_category_header_row(title: &str) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.set_selectable(false);
    row.set_activatable(false);
    row.add_css_class("wifi-category-header-row");

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    hbox.set_margin_top(25);
    hbox.set_margin_bottom(8);
    hbox.set_margin_start(12);
    hbox.set_margin_end(12);

    let title_lbl = gtk4::Label::new(Some(title));
    title_lbl.add_css_class("wifi-category-title");
    title_lbl.set_halign(gtk4::Align::Start);
    hbox.append(&title_lbl);

    row.set_child(Some(&hbox));
    row
}
