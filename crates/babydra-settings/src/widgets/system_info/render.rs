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
    disk_percent: f64,
) -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);

    // Header
    let header_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    let title_lbl = babydra_utils::components::create_title("Thông tin Hệ thống");
    let desc_lbl = gtk4::Label::new(Some("Xem chi tiết thông số phần cứng, dung lượng đĩa và hệ điều hành"));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(gtk4::Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    main_box.append(&header_box);

    // Hero Section Card (Hostname & OS Info & Disk Usage)
    let hero_section = babydra_utils::components::create_card(gtk4::Orientation::Horizontal, 16);
    hero_section.add_css_class("settings-card");

    let logo_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    logo_container.add_css_class("os-logo");
    logo_container.set_valign(gtk4::Align::Center);
    
    let logo_img = babydra_utils::ui::icon::get_icon("logo", 48);
    logo_img.set_pixel_size(48);
    logo_img.set_valign(gtk4::Align::Center);
    logo_img.set_halign(gtk4::Align::Center);
    logo_container.append(&logo_img);
    hero_section.append(&logo_container);

    let os_title_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    os_title_box.set_hexpand(true);
    os_title_box.set_valign(gtk4::Align::Center);

    let hostname_lbl = gtk4::Label::new(Some(hostname));
    hostname_lbl.add_css_class("hero-hostname");
    hostname_lbl.set_halign(gtk4::Align::Start);
    os_title_box.append(&hostname_lbl);

    let disk_info_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    let os_kernel_lbl = gtk4::Label::new(Some(&format!("{} • {}", os_name, kernel_version)));
    os_kernel_lbl.add_css_class("settings-row-desc");
    os_kernel_lbl.set_halign(gtk4::Align::Start);
    disk_info_row.append(&os_kernel_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    disk_info_row.append(&spacer);

    let disk_stats_lbl = gtk4::Label::new(Some(disk_text));
    disk_stats_lbl.add_css_class("settings-row-title");
    disk_stats_lbl.set_halign(gtk4::Align::End);
    disk_info_row.append(&disk_stats_lbl);
    os_title_box.append(&disk_info_row);

    let progress_bar = babydra_utils::components::create_disk_progress(disk_percent / 100.0, "");
    os_title_box.append(&progress_bar);

    hero_section.append(&os_title_box);
    main_box.append(&hero_section);

    // Section Title
    let specs_title = gtk4::Label::new(Some("THÔNG SỐ PHẦN CỨNG"));
    specs_title.add_css_class("settings-section-title");
    specs_title.set_halign(gtk4::Align::Start);
    main_box.append(&specs_title);

    // Hardware Specs Card ListBox
    let specs_listbox = gtk4::ListBox::new();
    specs_listbox.set_selection_mode(gtk4::SelectionMode::None);
    specs_listbox.add_css_class("settings-card");

    let specs = [
        ("performance", "Bộ vi xử lý (CPU)", cpu_model),
        ("display", "Đồ họa (GPU)", gpu_info),
        ("activity", "Bộ nhớ RAM", memory_text),
        ("info", "Phiên bản Kernel", kernel_version),
    ];

    for (icon_name, label_text, val_text) in &specs {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        hbox.set_margin_top(10);
        hbox.set_margin_bottom(10);
        hbox.set_margin_start(16);
        hbox.set_margin_end(16);

        let icon = babydra_utils::ui::icon::get_icon(icon_name, 18);
        icon.set_valign(gtk4::Align::Center);
        icon.add_css_class("settings-row-icon");
        hbox.append(&icon);

        let lbl = gtk4::Label::new(Some(*label_text));
        lbl.add_css_class("settings-row-title");
        lbl.set_halign(gtk4::Align::Start);
        lbl.set_valign(gtk4::Align::Center);
        hbox.append(&lbl);

        let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        hbox.append(&spacer);

        let val_lbl = gtk4::Label::new(Some(*val_text));
        val_lbl.add_css_class("settings-row-desc");
        val_lbl.set_halign(gtk4::Align::End);
        val_lbl.set_valign(gtk4::Align::Center);
        val_lbl.set_selectable(true);
        hbox.append(&val_lbl);

        row.set_child(Some(&hbox));
        specs_listbox.append(&row);
    }

    main_box.append(&specs_listbox);

    main_box
}
