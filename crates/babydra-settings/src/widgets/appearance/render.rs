//! Appearance UI layout generator matching design tokens.

use gtk4::prelude::*;

pub fn build_appearance_ui(
    current_wallpaper_path: &str,
    is_dark: bool,
    themes: &[String],
    current_theme: &str,
) -> (
    gtk4::Box,
    gtk4::Image,
    gtk4::Button,
    gtk4::Button,
    gtk4::Button,
    gtk4::DropDown,
) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 18);

    // Breadcrumb Header (System > Personalization)
    let breadcrumb_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let bc_parent = gtk4::Label::new(Some("System"));
    bc_parent.add_css_class("settings-breadcrumb-parent");
    let bc_arrow = gtk4::Label::new(Some("›"));
    bc_arrow.add_css_class("settings-breadcrumb-arrow");
    let bc_current = gtk4::Label::new(Some("Personalization"));
    bc_current.add_css_class("settings-breadcrumb-current");

    breadcrumb_box.append(&bc_parent);
    breadcrumb_box.append(&bc_arrow);
    breadcrumb_box.append(&bc_current);
    breadcrumb_box.set_halign(gtk4::Align::Start);
    main_box.append(&breadcrumb_box);

    // ── Section 1: System Theme Mode (Light / Dark Cards) ─────
    let theme_section_lbl = gtk4::Label::new(Some("CHỦ ĐỀ HỆ THỐNG"));
    theme_section_lbl.add_css_class("settings-section-title");
    theme_section_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&theme_section_lbl);

    let theme_card = babydra_utils::components::create_card(gtk4::Orientation::Horizontal, 12);
    theme_card.add_css_class("settings-card");

    let light_card = gtk4::Button::new();
    light_card.add_css_class("theme-option-card");
    light_card.set_hexpand(true);
    light_card.set_cursor_from_name(Some("pointer"));

    let light_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    light_content.set_halign(gtk4::Align::Center);
    light_content.set_valign(gtk4::Align::Center);
    let light_icon = babydra_utils::ui::icon::get_icon("brightness", 18);
    light_content.append(&light_icon);
    let light_lbl = gtk4::Label::new(Some("Chế độ Sáng"));
    light_lbl.add_css_class("settings-row-title");
    light_content.append(&light_lbl);
    light_card.set_child(Some(&light_content));
    theme_card.append(&light_card);

    let dark_card = gtk4::Button::new();
    dark_card.add_css_class("theme-option-card");
    dark_card.set_hexpand(true);
    dark_card.set_cursor_from_name(Some("pointer"));

    let dark_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    dark_content.set_halign(gtk4::Align::Center);
    dark_content.set_valign(gtk4::Align::Center);
    let dark_icon = babydra_utils::ui::icon::get_icon("dark-mode", 18);
    dark_content.append(&dark_icon);
    let dark_lbl = gtk4::Label::new(Some("Chế độ Tối"));
    dark_lbl.add_css_class("settings-row-title");
    dark_content.append(&dark_lbl);
    dark_card.set_child(Some(&dark_content));
    theme_card.append(&dark_card);

    if is_dark {
        dark_card.add_css_class("active");
        light_card.remove_css_class("active");
    } else {
        light_card.add_css_class("active");
        dark_card.remove_css_class("active");
    }

    main_box.append(&theme_card);

    // ── Section 2: Desktop Wallpaper ───────────────────────────
    let wallpaper_section_lbl = gtk4::Label::new(Some("HÌNH NỀN MÁY TÍNH"));
    wallpaper_section_lbl.add_css_class("settings-section-title");
    wallpaper_section_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&wallpaper_section_lbl);

    let wallpaper_card = babydra_utils::components::create_card(gtk4::Orientation::Horizontal, 16);
    wallpaper_card.add_css_class("settings-card");

    // Preview thumbnail
    let preview_col = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    preview_col.add_css_class("wallpaper-preview-frame");
    preview_col.set_size_request(140, 80);

    let preview_img = gtk4::Image::new();
    let clean_path = current_wallpaper_path.replace("file://", "");
    if !clean_path.is_empty() && std::path::Path::new(&clean_path).exists() {
        preview_img.set_from_file(Some(&clean_path));
        preview_img.set_pixel_size(80);
    } else {
        let display_icon = babydra_utils::ui::icon::get_icon("display", 36);
        display_icon.set_pixel_size(36);
        display_icon.set_valign(gtk4::Align::Center);
        display_icon.set_halign(gtk4::Align::Center);
        preview_col.append(&display_icon);
    }
    preview_img.set_valign(gtk4::Align::Center);
    preview_img.set_halign(gtk4::Align::Center);
    preview_col.append(&preview_img);
    wallpaper_card.append(&preview_col);

    // Info text in middle
    let wp_info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    wp_info_box.set_valign(gtk4::Align::Center);
    wp_info_box.set_hexpand(true);

    let wp_title_lbl = gtk4::Label::new(Some("Hình nền hiện tại"));
    wp_title_lbl.add_css_class("settings-row-title");
    wp_title_lbl.set_halign(gtk4::Align::Start);

    let wp_path_lbl = gtk4::Label::new(Some(if clean_path.is_empty() { "Mặc định hệ thống" } else { &clean_path }));
    wp_path_lbl.add_css_class("settings-row-desc");
    wp_path_lbl.set_halign(gtk4::Align::Start);
    wp_path_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);

    wp_info_box.append(&wp_title_lbl);
    wp_info_box.append(&wp_path_lbl);
    wallpaper_card.append(&wp_info_box);

    // Pick file button on right
    let pick_btn = babydra_utils::components::create_accent_button("Chọn hình nền...");
    pick_btn.set_valign(gtk4::Align::Center);
    pick_btn.set_cursor_from_name(Some("pointer"));
    wallpaper_card.append(&pick_btn);

    main_box.append(&wallpaper_card);

    // ── Section 3: GTK Application Theme ─────────────────────
    let app_theme_lbl = gtk4::Label::new(Some("GIAO DIỆN ỨNG DỤNG"));
    app_theme_lbl.add_css_class("settings-section-title");
    app_theme_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&app_theme_lbl);

    let gtk_listbox = gtk4::ListBox::new();
    gtk_listbox.set_selection_mode(gtk4::SelectionMode::None);
    gtk_listbox.add_css_class("settings-card");

    let gtk_row = gtk4::ListBoxRow::new();
    gtk_row.add_css_class("settings-card-row");

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    hbox.set_margin_top(10);
    hbox.set_margin_bottom(10);
    hbox.set_margin_start(16);
    hbox.set_margin_end(16);

    let palette_icon = babydra_utils::ui::icon::get_icon("display", 18);
    palette_icon.set_valign(gtk4::Align::Center);
    palette_icon.add_css_class("settings-row-icon");
    hbox.append(&palette_icon);

    let gtk_lbl = gtk4::Label::new(Some("Giao diện GTK Theme"));
    gtk_lbl.add_css_class("settings-row-title");
    gtk_lbl.set_halign(gtk4::Align::Start);
    gtk_lbl.set_valign(gtk4::Align::Center);
    hbox.append(&gtk_lbl);

    let spacer2 = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer2.set_hexpand(true);
    hbox.append(&spacer2);

    let dropdown = gtk4::DropDown::from_strings(&themes.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    dropdown.set_valign(gtk4::Align::Center);
    dropdown.set_cursor_from_name(Some("pointer"));
    if let Some(pos) = themes.iter().position(|t| t == current_theme) {
        dropdown.set_selected(pos as u32);
    }
    hbox.append(&dropdown);

    gtk_row.set_child(Some(&hbox));
    gtk_listbox.append(&gtk_row);
    main_box.append(&gtk_listbox);

    (
        main_box,
        preview_img,
        pick_btn,
        light_card,
        dark_card,
        dropdown,
    )
}
