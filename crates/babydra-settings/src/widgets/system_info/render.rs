//! System specifications UI layout generator matching Windows 11 / About page layout.

use gtk4::prelude::*;

pub fn build_system_ui(
    hostname: &str,
    os_name: &str,
    kernel_version: &str,
    cpu_model: &str,
    gpu_info: &str,
    memory_text: &str,
    disk_text: &str,
    _disk_percent: f64,
) -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 24);

    // Breadcrumb Header (System > About)
    let breadcrumb_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    breadcrumb_box.set_margin_bottom(4);

    let bc_parent = gtk4::Label::new(Some("System"));
    bc_parent.add_css_class("settings-breadcrumb-parent");
    let bc_arrow = gtk4::Label::new(Some("›"));
    bc_arrow.add_css_class("settings-breadcrumb-arrow");
    let bc_current = gtk4::Label::new(Some("About"));
    bc_current.add_css_class("settings-breadcrumb-current");

    breadcrumb_box.append(&bc_parent);
    breadcrumb_box.append(&bc_arrow);
    breadcrumb_box.append(&bc_current);
    breadcrumb_box.set_halign(gtk4::Align::Start);
    main_box.append(&breadcrumb_box);

    // ── Card 1: Top Host Header Card ───────────────────────────
    let host_card = babydra_utils::components::create_card(gtk4::Orientation::Horizontal, 20);
    host_card.add_css_class("settings-card");

    let logo_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    logo_container.add_css_class("os-logo");
    logo_container.set_valign(gtk4::Align::Center);

    let logo_img = babydra_utils::ui::icon::get_icon("logo", 48);
    logo_img.set_pixel_size(48);
    logo_img.set_valign(gtk4::Align::Center);
    logo_img.set_halign(gtk4::Align::Center);
    logo_container.append(&logo_img);
    host_card.append(&logo_container);

    let host_info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    host_info_box.set_hexpand(true);
    host_info_box.set_valign(gtk4::Align::Center);

    let hostname_lbl = gtk4::Label::new(Some(hostname));
    hostname_lbl.add_css_class("hero-hostname");
    hostname_lbl.set_halign(gtk4::Align::Start);
    host_info_box.append(&hostname_lbl);

    let os_sub_lbl = gtk4::Label::new(Some(&format!("{} • {}", os_name, kernel_version)));
    os_sub_lbl.add_css_class("settings-row-desc");
    os_sub_lbl.set_halign(gtk4::Align::Start);
    host_info_box.append(&os_sub_lbl);
    host_card.append(&host_info_box);

    let rename_btn = gtk4::Button::with_label("Rename this PC");
    rename_btn.add_css_class("connect-pill-btn");
    rename_btn.set_valign(gtk4::Align::Center);
    rename_btn.set_cursor_from_name(Some("pointer"));
    host_card.append(&rename_btn);

    main_box.append(&host_card);

    // ── Card 2: Device Specifications Group Card ───────────────
    let dev_group_card = babydra_utils::components::create_card(gtk4::Orientation::Vertical, 0);
    dev_group_card.add_css_class("settings-card");

    // Group Header Row
    let dev_header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    dev_header_row.set_margin_top(2);
    dev_header_row.set_margin_bottom(18);

    let dev_icon = babydra_utils::ui::icon::get_icon("info", 20);
    dev_icon.set_valign(gtk4::Align::Center);
    dev_icon.add_css_class("settings-row-icon");
    dev_header_row.append(&dev_icon);

    let dev_header_title = gtk4::Label::new(Some("Device specifications"));
    dev_header_title.add_css_class("settings-group-header-title");
    dev_header_title.set_halign(gtk4::Align::Start);
    dev_header_title.set_valign(gtk4::Align::Center);
    dev_header_row.append(&dev_header_title);

    dev_group_card.append(&dev_header_row);

    // Device Specs Grid (Key-Value aligned with generous padding & spacing)
    let dev_specs_grid = gtk4::Grid::new();
    dev_specs_grid.set_column_spacing(48);
    dev_specs_grid.set_row_spacing(14);
    dev_specs_grid.set_margin_start(32);
    dev_specs_grid.set_margin_bottom(8);

    let dev_rows = [
        ("Device name", hostname.to_string()),
        ("Processor", cpu_model.to_string()),
        ("Installed RAM", memory_text.to_string()),
        ("Graphics", gpu_info.to_string()),
        ("Storage", disk_text.to_string()),
    ];

    for (idx, (key, val)) in dev_rows.iter().enumerate() {
        let key_lbl = gtk4::Label::new(Some(*key));
        key_lbl.add_css_class("settings-row-desc");
        key_lbl.set_halign(gtk4::Align::Start);
        key_lbl.set_valign(gtk4::Align::Center);

        let val_lbl = gtk4::Label::new(Some(val));
        val_lbl.add_css_class("settings-row-title");
        val_lbl.set_halign(gtk4::Align::Start);
        val_lbl.set_valign(gtk4::Align::Center);
        val_lbl.set_wrap(true);
        val_lbl.set_selectable(true);

        dev_specs_grid.attach(&key_lbl, 0, idx as i32, 1, 1);
        dev_specs_grid.attach(&val_lbl, 1, idx as i32, 1, 1);
    }

    dev_group_card.append(&dev_specs_grid);
    main_box.append(&dev_group_card);

    // ── Card 3: OS Specifications Group Card ───────────────────
    let os_group_card = babydra_utils::components::create_card(gtk4::Orientation::Vertical, 0);
    os_group_card.add_css_class("settings-card");

    // Group Header Row
    let os_header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    os_header_row.set_margin_top(2);
    os_header_row.set_margin_bottom(18);

    let os_icon = babydra_utils::ui::icon::get_icon("display", 20);
    os_icon.set_valign(gtk4::Align::Center);
    os_icon.add_css_class("settings-row-icon");
    os_header_row.append(&os_icon);

    let os_header_title = gtk4::Label::new(Some("Arch Linux specifications"));
    os_header_title.add_css_class("settings-group-header-title");
    os_header_title.set_halign(gtk4::Align::Start);
    os_header_title.set_valign(gtk4::Align::Center);
    os_header_row.append(&os_header_title);

    os_group_card.append(&os_header_row);

    // OS Specs Grid
    let os_specs_grid = gtk4::Grid::new();
    os_specs_grid.set_column_spacing(48);
    os_specs_grid.set_row_spacing(14);
    os_specs_grid.set_margin_start(32);
    os_specs_grid.set_margin_bottom(8);

    let os_rows = [
        ("Edition", os_name.to_string()),
        ("Kernel version", kernel_version.to_string()),
        ("System type", "64-bit operating system, x86_64".to_string()),
        ("Desktop environment", "BabyDra Desktop System".to_string()),
    ];

    for (idx, (key, val)) in os_rows.iter().enumerate() {
        let key_lbl = gtk4::Label::new(Some(*key));
        key_lbl.add_css_class("settings-row-desc");
        key_lbl.set_halign(gtk4::Align::Start);
        key_lbl.set_valign(gtk4::Align::Center);

        let val_lbl = gtk4::Label::new(Some(val));
        val_lbl.add_css_class("settings-row-title");
        val_lbl.set_halign(gtk4::Align::Start);
        val_lbl.set_valign(gtk4::Align::Center);
        val_lbl.set_selectable(true);

        os_specs_grid.attach(&key_lbl, 0, idx as i32, 1, 1);
        os_specs_grid.attach(&val_lbl, 1, idx as i32, 1, 1);
    }

    os_group_card.append(&os_specs_grid);
    main_box.append(&os_group_card);

    main_box
}
