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
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
    main_box.set_margin_start(16);
    main_box.set_margin_end(16);

    // Title
    let title_lbl = baby_utils::components::create_title("About System");
    main_box.append(&title_lbl);

    // Hero Section glass-panel
    let hero_section = baby_utils::components::create_card(gtk4::Orientation::Horizontal, 20);
    hero_section.set_margin_bottom(8);

    // OS Logo/Avatar
    let logo_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    logo_container.add_css_class("os-logo");
    logo_container.set_size_request(100, 100);
    
    let logo_img = gtk4::Image::from_icon_name("computer-symbolic");
    logo_img.set_pixel_size(72);
    logo_img.set_valign(gtk4::Align::Center);
    logo_img.set_halign(gtk4::Align::Center);
    logo_container.append(&logo_img);
    hero_section.append(&logo_container);

    // OS Title and Disk Usage
    let os_title_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    os_title_box.set_hexpand(true);

    let hostname_lbl = gtk4::Label::new(Some(hostname));
    hostname_lbl.add_css_class("hero-hostname");
    hostname_lbl.set_halign(gtk4::Align::Start);
    os_title_box.append(&hostname_lbl);

    let disk_info_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    let os_kernel_lbl = gtk4::Label::new(Some(&format!("{} - {}", os_name, kernel_version)));
    os_kernel_lbl.add_css_class("settings-desc");
    os_kernel_lbl.set_halign(gtk4::Align::Start);
    disk_info_row.append(&os_kernel_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    disk_info_row.append(&spacer);

    let disk_stats_lbl = gtk4::Label::new(Some(disk_text));
    disk_stats_lbl.add_css_class("settings-label");
    disk_stats_lbl.set_halign(gtk4::Align::End);
    disk_info_row.append(&disk_stats_lbl);
    os_title_box.append(&disk_info_row);

    // Progress bar for disk usage
    let progress_bar = baby_utils::components::create_disk_progress(disk_percent / 100.0, "");
    os_title_box.append(&progress_bar);

    hero_section.append(&os_title_box);
    main_box.append(&hero_section);

    // Grid of cards
    let grid = gtk4::Grid::new();
    grid.set_column_spacing(16);
    grid.set_row_spacing(16);
    grid.set_column_homogeneous(true);

    let create_info_card = |icon_name: &str, label: &str, value: &str| -> gtk4::Box {
        let card = baby_utils::components::create_card(gtk4::Orientation::Horizontal, 16);

        let icon_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_box.add_css_class("card-icon-wrapper");
        icon_box.set_size_request(46, 46);
        icon_box.set_valign(gtk4::Align::Center);
        
        let icon_img = gtk4::Image::from_icon_name(icon_name);
        icon_img.set_pixel_size(20);
        icon_box.append(&icon_img);
        card.append(&icon_box);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        let label_lbl = gtk4::Label::new(Some(label));
        label_lbl.add_css_class("settings-desc");
        label_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&label_lbl);

        let value_lbl = gtk4::Label::new(Some(value));
        value_lbl.add_css_class("settings-label");
        value_lbl.set_halign(gtk4::Align::Start);
        value_lbl.set_wrap(true);
        text_box.append(&value_lbl);
        card.append(&text_box);

        card
    };

    let card_kernel = create_info_card("preferences-system-symbolic", "Kernel", kernel_version);
    let card_cpu = create_info_card("cpu-symbolic", "Processor", cpu_model);
    let card_mem = create_info_card("media-flash-symbolic", "Memory", memory_text);
    let card_gpu = create_info_card("video-display-symbolic", "Graphics", gpu_info);

    grid.attach(&card_kernel, 0, 0, 1, 1);
    grid.attach(&card_cpu, 1, 0, 1, 1);
    grid.attach(&card_mem, 0, 1, 1, 1);
    grid.attach(&card_gpu, 1, 1, 1, 1);

    main_box.append(&grid);

    main_box
}
