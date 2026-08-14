//! Bluetooth UI layout generator synchronized with Wi-Fi layout.

use babydra_utils::components::ToggleRow;
use gtk4::prelude::*;

pub fn build_bluetooth_ui() -> (gtk4::Box, ToggleRow, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Row (Bluetooth Title + On Switcher)
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let title_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.bt_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    // Toggle Switch (On)
    let toggle_row = ToggleRow::new(true);
    header_row.append(&toggle_row.container);

    main_box.append(&header_row);

    // Glass Panel List Container (Fixed height extending to bottom line)
    let glass_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    glass_card.append(&scroll);
    main_box.append(&glass_card);

    (main_box, toggle_row, list_box)
}
