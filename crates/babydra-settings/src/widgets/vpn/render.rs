//! VPN UI layout generator.

use gtk4::prelude::*;

pub fn build_vpn_ui() -> (gtk4::Box, gtk4::Button, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    let title_lbl = baby_utils::components::create_title("VPN & Mạng ảo");
    title_box.append(&title_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    title_box.append(&spacer);

    let import_btn = baby_utils::components::create_accent_button("Nhập file cấu hình (.ovpn/.conf)");
    import_btn.set_valign(gtk4::Align::Center);
    title_box.append(&import_btn);
    main_box.append(&title_box);

    let list_container = baby_utils::components::create_card(gtk4::Orientation::Vertical, 8);
    list_container.set_vexpand(true);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    scroll.set_child(Some(&list_box));
    list_container.append(&scroll);
    main_box.append(&list_container);

    (main_box, import_btn, list_box)
}
