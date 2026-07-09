//! System specifications UI layout generator.

use gtk4::prelude::*;

pub fn build_system_ui(
    hostname: &str,
    os_name: &str,
    kernel_version: &str,
    cpu_model: &str,
    gpu_info: &str,
    memory_text: &str,
    disk_text: &str,
) -> (gtk4::Box, gtk4::Button) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = gtk4::Label::new(Some("Thông tin hệ thống"));
    title_lbl.add_css_class("settings-title");
    title_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&title_lbl);

    let stats_card = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    stats_card.add_css_class("settings-card");

    let add_info_row = |label: &str, value: &str| {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.set_margin_top(4);
        row.set_margin_bottom(4);

        let lbl = gtk4::Label::new(Some(label));
        lbl.add_css_class("settings-label");
        lbl.set_halign(gtk4::Align::Start);
        row.append(&lbl);

        let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        row.append(&spacer);

        let val = gtk4::Label::new(Some(value));
        val.add_css_class("settings-desc");
        val.set_halign(gtk4::Align::End);
        row.append(&val);

        stats_card.append(&row);
    };

    add_info_row("Tên máy (Hostname)", hostname);
    add_info_row("Hệ điều hành (OS)", os_name);
    add_info_row("Nhân Kernel", kernel_version);
    add_info_row("Bộ vi xử lý (CPU)", cpu_model);
    add_info_row("Card đồ họa (GPU)", gpu_info);
    add_info_row("Bộ nhớ RAM", memory_text);
    add_info_row("Ổ đĩa hệ thống (/)", disk_text);
    main_box.append(&stats_card);

    let update_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    update_card.add_css_class("settings-card");
    update_card.set_valign(gtk4::Align::Center);

    let update_lbl_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let update_title = gtk4::Label::new(Some("Cập nhật hệ thống"));
    update_title.add_css_class("settings-label");
    update_title.set_halign(gtk4::Align::Start);
    let update_desc = gtk4::Label::new(Some("Kiểm tra và cài đặt các bản nâng cấp Arch Linux mới nhất"));
    update_desc.add_css_class("settings-desc");
    update_desc.set_halign(gtk4::Align::Start);
    update_lbl_box.append(&update_title);
    update_lbl_box.append(&update_desc);
    update_card.append(&update_lbl_box);

    let update_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    update_spacer.set_hexpand(true);
    update_card.append(&update_spacer);

    let update_btn = gtk4::Button::with_label("Cập nhật ngay");
    update_btn.set_valign(gtk4::Align::Center);
    update_btn.add_css_class("suggested-action");
    update_card.append(&update_btn);
    main_box.append(&update_card);

    (main_box, update_btn)
}
