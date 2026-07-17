//! Bluetooth UI layout generator.

use gtk4::prelude::*;

pub fn build_bluetooth_ui() -> (gtk4::Box, gtk4::Switch, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = babydra_utils::components::create_title("Bluetooth");
    main_box.append(&title_lbl);

    let (switch_card, bt_switch) = babydra_utils::components::create_switch_card(
        "Bật/Tắt Bluetooth",
        "Quản lý kết nối tai nghe, chuột, bàn phím và thiết bị không dây khác"
    );
    main_box.append(&switch_card);

    let list_title = babydra_utils::components::create_subtitle("Danh sách thiết bị ghép nối");
    main_box.append(&list_title);

    let list_container = babydra_utils::components::create_card(gtk4::Orientation::Vertical, 8);
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
