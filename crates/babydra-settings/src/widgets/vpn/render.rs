//! VPN UI layout generator synchronized with explore settings_dialog.

use gtk4::prelude::*;

pub fn build_vpn_ui() -> (gtk4::Box, gtk4::Button, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);

    let header_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let title_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    let title_lbl = babydra_utils::components::create_title("VPN & Mạng ảo");
    title_lbl.add_css_class("settings-title-label");
    title_box.append(&title_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    title_box.append(&spacer);

    let import_btn = babydra_utils::components::create_accent_button("Nhập file cấu hình");
    import_btn.set_valign(gtk4::Align::Center);
    title_box.append(&import_btn);
    header_box.append(&title_box);

    let desc_lbl = gtk4::Label::new(Some("Quản lý các đường truyền bảo mật VPN và WireGuard/OpenVPN"));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(gtk4::Align::Start);
    header_box.append(&desc_lbl);

    main_box.append(&header_box);

    let list_title = gtk4::Label::new(Some("DANH SÁCH MẠNG VPN KHẢ DỤNG"));
    list_title.add_css_class("settings-section-title");
    list_title.set_halign(gtk4::Align::Start);
    main_box.append(&list_title);

    let list_box = gtk4::ListBox::new();
    list_box.add_css_class("settings-card");
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_child(Some(&list_box));

    main_box.append(&scroll);

    (main_box, import_btn, list_box)
}
