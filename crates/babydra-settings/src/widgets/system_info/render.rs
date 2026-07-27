//! System specifications UI layout generator matching reference design Image 1.

use gtk4::prelude::*;

pub fn build_system_ui(
    hostname: &str,
    os_name: &str,
    kernel_version: &str,
    cpu_model: &str,
    gpu_info: &str,
    memory_text: &str,
    uptime_text: &str,
    cpu_arch: &str,
) -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Page Title
    let page_title = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.about_title")));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    main_box.append(&page_title);

    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);

    // ── Card 1: Top Hero Card (Avatar, OS Title, Uptime Badge) ──
    let hero_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 20);
    hero_card.add_css_class("glass-panel");
    hero_card.set_margin_top(4);
    hero_card.set_margin_bottom(4);
    hero_card.set_margin_start(4);
    hero_card.set_margin_end(4);

    // Mascot / Avatar Container
    let avatar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    avatar_box.add_css_class("hero-avatar-box");
    avatar_box.set_size_request(80, 80);
    avatar_box.set_valign(gtk4::Align::Center);

    let avatar_img = babydra_utils::ui::icon::get_icon("logo", 80);
    avatar_img.set_pixel_size(80);
    avatar_img.set_valign(gtk4::Align::Center);
    avatar_img.set_halign(gtk4::Align::Center);
    avatar_box.append(&avatar_img);
    hero_card.append(&avatar_box);

    // Right Side Information
    let info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    info_box.set_hexpand(true);
    info_box.set_valign(gtk4::Align::Center);

    // Top Row: Hostname + Uptime Pill Badge
    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);

    let display_host = if !hostname.is_empty() && hostname != "localhost" {
        hostname
    } else {
        "BabyDra Linux"
    };

    let os_label = gtk4::Label::new(Some(display_host));
    os_label.add_css_class("hero-hostname");
    os_label.set_halign(gtk4::Align::Start);
    os_label.set_hexpand(true);
    title_row.append(&os_label);

    // Sleek Uptime Badge (e.g. "Up 2h 15m")
    let uptime_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    uptime_badge.add_css_class("hero-uptime-badge");
    uptime_badge.set_valign(gtk4::Align::Center);

    let clock_icon = babydra_utils::ui::icon::get_icon("history", 14);
    clock_icon.set_pixel_size(14);
    clock_icon.set_valign(gtk4::Align::Center);
    uptime_badge.append(&clock_icon);

    let formatted_uptime = babydra_common::i18n::t("settings.up_time").replace("{}", uptime_text);
    let uptime_lbl = gtk4::Label::new(Some(&formatted_uptime));
    uptime_lbl.add_css_class("hero-uptime-label");
    uptime_lbl.set_valign(gtk4::Align::Center);
    uptime_badge.append(&uptime_lbl);

    title_row.append(&uptime_badge);
    info_box.append(&title_row);

    // Subtitle Row: OS Name (Architecture) • Kernel Version
    let sub_title = format!("{} ({}) • Kernel {}", os_name, cpu_arch, kernel_version);
    let sub_label = gtk4::Label::new(Some(&sub_title));
    sub_label.add_css_class("hero-subtitle");
    sub_label.set_halign(gtk4::Align::Start);
    info_box.append(&sub_label);

    hero_card.append(&info_box);
    content_box.append(&hero_card);

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

    content_box.append(&grid);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&content_box));

    main_box.append(&scroll);
    main_box
}

