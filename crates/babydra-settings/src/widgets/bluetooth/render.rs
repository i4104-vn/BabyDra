//! Bluetooth UI layout generator.

use gtk4::prelude::*;

pub fn build_bluetooth_ui() -> (gtk4::Box, gtk4::Switch, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = gtk4::Label::new(Some("Bluetooth"));
    title_lbl.add_css_class("settings-title");
    title_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&title_lbl);

    let switch_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    switch_card.add_css_class("settings-card");
    switch_card.set_valign(gtk4::Align::Center);

    let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let status_title = gtk4::Label::new(Some("Bật/Tắt Bluetooth"));
    status_title.add_css_class("settings-label");
    status_title.set_halign(gtk4::Align::Start);
    let status_desc = gtk4::Label::new(Some("Quản lý kết nối tai nghe, chuột, bàn phím và thiết bị không dây khác"));
    status_desc.add_css_class("settings-desc");
    status_desc.set_halign(gtk4::Align::Start);
    label_box.append(&status_title);
    label_box.append(&status_desc);
    switch_card.append(&label_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    switch_card.append(&spacer);

    let bt_switch = gtk4::Switch::new();
    bt_switch.set_valign(gtk4::Align::Center);
    switch_card.append(&bt_switch);
    main_box.append(&switch_card);

    let list_title = gtk4::Label::new(Some("Danh sách thiết bị ghép nối"));
    list_title.add_css_class("settings-subtitle");
    list_title.set_halign(gtk4::Align::Start);
    main_box.append(&list_title);

    let list_container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    list_container.add_css_class("settings-card");
    list_container.set_vexpand(true);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    scroll.set_child(Some(&list_box));
    list_container.append(&scroll);
    main_box.append(&list_container);

    (main_box, bt_switch, list_box)
}
