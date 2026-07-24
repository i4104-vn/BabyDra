//! System specifications UI layout generator matching reference design Image 1.

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

    // Page Title
    let page_title = gtk4::Label::new(Some("About System"));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    main_box.append(&page_title);

    // ── Card 1: Top Hero Card (Avatar, OS Title, Storage Progress) ──
    let hero_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 24);
    hero_card.add_css_class("glass-panel");
    hero_card.set_margin_top(4);
    hero_card.set_margin_bottom(4);
    hero_card.set_margin_start(4);
    hero_card.set_margin_end(4);

    // Mascot / Avatar Container
    let avatar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    avatar_box.add_css_class("hero-avatar-box");
    avatar_box.set_size_request(100, 100);
    avatar_box.set_valign(gtk4::Align::Center);

    let avatar_img = babydra_utils::ui::icon::get_icon("logo", 96);
    avatar_img.set_pixel_size(96);
    avatar_img.set_valign(gtk4::Align::Center);
    avatar_img.set_halign(gtk4::Align::Center);
    avatar_box.append(&avatar_img);
    hero_card.append(&avatar_box);

    // Right Side Information
    let info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    info_box.set_hexpand(true);
    info_box.set_valign(gtk4::Align::Center);

    // Hostname / OS Header
    let display_title = if !hostname.is_empty() && hostname != "localhost" {
        format!("{} - {} {}", hostname, os_name, kernel_version)
    } else {
        format!("{} - Linux {}", os_name, kernel_version)
    };

    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let os_label = gtk4::Label::new(Some(&display_title));
    os_label.add_css_class("hero-hostname");
    os_label.set_halign(gtk4::Align::Start);
    os_label.set_hexpand(true);
    title_row.append(&os_label);

    let disk_stats_lbl = gtk4::Label::new(Some(disk_text));
    disk_stats_lbl.add_css_class("hero-stats-label");
    disk_stats_lbl.set_halign(gtk4::Align::End);
    title_row.append(&disk_stats_lbl);

    info_box.append(&title_row);

    // Disk Progress Bar
    let progress_bar = gtk4::ProgressBar::new();
    progress_bar.add_css_class("disk-progress");
    progress_bar.set_fraction(disk_percent / 100.0);
    progress_bar.set_hexpand(true);
    info_box.append(&progress_bar);

    hero_card.append(&info_box);
    main_box.append(&hero_card);

    // ── 2x2 Grid of Hardware Spec Cards ─────────────────────────
    let grid = gtk4::Grid::new();
    grid.set_column_spacing(20);
    grid.set_row_spacing(20);
    grid.set_column_homogeneous(true);

    let specs = [
        ("cog", "KERNEL", kernel_version),
        ("sliders", "PROCESSOR", cpu_model),
        ("history", "MEMORY", memory_text),
        ("palette", "GRAPHICS", gpu_info),
    ];

    for (idx, (icon_name, label, value)) in specs.iter().enumerate() {
        let card = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        card.add_css_class("spec-card");
        card.set_halign(gtk4::Align::Fill);
        card.set_valign(gtk4::Align::Fill);

        // Centered Blue Icon Badge
        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge");
        icon_badge.set_halign(gtk4::Align::Center);
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_size_request(44, 44);

        let icon_img = babydra_utils::ui::icon::get_icon(icon_name, 22);
        icon_img.set_pixel_size(22);
        icon_img.set_vexpand(true);
        icon_img.set_hexpand(true);
        icon_img.set_valign(gtk4::Align::Center);
        icon_img.set_halign(gtk4::Align::Center);
        icon_badge.append(&icon_img);
        card.append(&icon_badge);

        // Label (e.g. KERNEL, PROCESSOR, MEMORY, GRAPHICS)
        let label_widget = gtk4::Label::new(Some(*label));
        label_widget.add_css_class("spec-label");
        label_widget.set_halign(gtk4::Align::Center);
        card.append(&label_widget);

        // Value (e.g. Linux 7.1.3-arch1-1, Intel Core i5...)
        let value_widget = gtk4::Label::new(Some(*value));
        value_widget.add_css_class("spec-value");
        value_widget.set_halign(gtk4::Align::Center);
        value_widget.set_justify(gtk4::Justification::Center);
        value_widget.set_wrap(true);
        card.append(&value_widget);

        let col = (idx % 2) as i32;
        let row = (idx / 2) as i32;
        grid.attach(&card, col, row, 1, 1);
    }

    main_box.append(&grid);
    main_box
}

