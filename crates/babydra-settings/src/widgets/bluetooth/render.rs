//! Bluetooth UI layout generator synchronized with Wi-Fi layout.

use gtk4::prelude::*;

pub fn build_bluetooth_ui() -> (gtk4::Box, gtk4::Switch, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);

    // Header Row (Bluetooth Title + On Switcher)
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let title_lbl = gtk4::Label::new(Some("Bluetooth"));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    // Toggle Switch (On)
    let switch_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    switch_box.set_valign(gtk4::Align::Center);

    let switch_lbl = gtk4::Label::new(Some("On"));
    switch_lbl.add_css_class("settings-page-subtitle");
    switch_lbl.set_valign(gtk4::Align::Center);

    let bt_switch = gtk4::Switch::new();
    bt_switch.set_active(true);
    bt_switch.set_valign(gtk4::Align::Center);

    switch_box.append(&switch_lbl);
    switch_box.append(&bt_switch);
    header_row.append(&switch_box);

    main_box.append(&header_row);

    // Glass Panel List Container (Fills Full Height)
    let list_box = gtk4::ListBox::new();
    list_box.add_css_class("glass-panel");
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    list_box.set_vexpand(true);
    list_box.set_valign(gtk4::Align::Fill);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    main_box.append(&scroll);

    (main_box, bt_switch, list_box)
}
