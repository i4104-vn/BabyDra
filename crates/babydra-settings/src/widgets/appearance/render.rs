<<<<<<< HEAD
//! Appearance UI layout generator with local ~/.babydra/wallpaper grid.
=======
//! Appearance UI layout generator matching design tokens.
>>>>>>> hard-develop

use gtk4::prelude::*;

pub fn build_appearance_ui(
    current_wallpaper_path: &str,
    is_dark: bool,
    gtk_themes: &[String],
    icon_themes: &[String],
    cursor_themes: &[String],
    cursor_sizes: &[u32],
) -> (
    gtk4::Box,
    gtk4::Picture,
    gtk4::Button,
    gtk4::Button,
    gtk4::DropDown,
    gtk4::DropDown,
    gtk4::DropDown,
    gtk4::DropDown,
    gtk4::Box,
) {
<<<<<<< HEAD
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Title: Wallpaper & Colors
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let page_title = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.appearance_title")));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    header_box.append(&page_title);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_box.append(&spacer);

    main_box.append(&header_box);

    // Dashboard Main Glass Panel
    let dashboard_panel = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    dashboard_panel.add_css_class("glass-panel");
    dashboard_panel.set_vexpand(true);
    dashboard_panel.set_valign(gtk4::Align::Fill);

    // Top 2-Column Configuration Grid
    let top_grid = gtk4::Grid::new();
    top_grid.set_column_spacing(20);
    top_grid.set_row_spacing(16);
    top_grid.set_column_homogeneous(true);

    // Column 0 (Left): Narrowed Wallpaper Preview Overlay with Floating Controls
    let preview_overlay = gtk4::Overlay::new();
    preview_overlay.add_css_class("wallpaper-preview-overlay");
    preview_overlay.set_size_request(-1, 160);

    let preview_pic = gtk4::Picture::new();
    preview_pic.set_size_request(-1, 160);
    preview_pic.set_content_fit(gtk4::ContentFit::Cover);
    preview_pic.add_css_class("wallpaper-preview-picture");

    let clean_path = current_wallpaper_path.replace("file://", "");
    if !clean_path.is_empty() && std::path::Path::new(&clean_path).exists() {
        preview_pic.set_filename(Some(&clean_path));
    }
    preview_overlay.set_child(Some(&preview_pic));

    // Vertical Overlay Actions Column on the Right Edge (Plus Button + Theme Toggle Button)
    let actions_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    actions_box.set_valign(gtk4::Align::End);
    actions_box.set_halign(gtk4::Align::End);
    actions_box.set_margin_end(10);
    actions_box.set_margin_bottom(10);

    // 1. Shorter Add Wallpaper Button (Plus icon only)
    let pick_btn = gtk4::Button::new();
    pick_btn.add_css_class("wallpaper-action-btn");
    pick_btn.set_cursor_from_name(Some("pointer"));
    pick_btn.set_size_request(38, 38);

    let plus_icon = babydra_utils::ui::icon::get_icon("plus", 18);
    plus_icon.set_pixel_size(18);
    plus_icon.set_valign(gtk4::Align::Center);
    plus_icon.set_halign(gtk4::Align::Center);
    pick_btn.set_child(Some(&plus_icon));
    actions_box.append(&pick_btn);

    // 2. Icon-only Theme Toggle Button (Sun/Moon icon toggles theme & icon)
    let theme_toggle_btn = gtk4::Button::new();
    theme_toggle_btn.add_css_class("wallpaper-action-btn");
    theme_toggle_btn.set_cursor_from_name(Some("pointer"));
    theme_toggle_btn.set_size_request(38, 38);

    let initial_theme_icon = if is_dark { "brightness" } else { "dark-mode" };
    let theme_icon = babydra_utils::ui::icon::get_icon(initial_theme_icon, 18);
    theme_icon.set_pixel_size(18);
    theme_icon.set_valign(gtk4::Align::Center);
    theme_icon.set_halign(gtk4::Align::Center);
    theme_toggle_btn.set_child(Some(&theme_icon));
    actions_box.append(&theme_toggle_btn);

    preview_overlay.add_overlay(&actions_box);
    top_grid.attach(&preview_overlay, 0, 0, 1, 1);

    // Column 1 (Right): System Themes Configuration Dropdowns (2x2 Grid)
    let theme_grid = gtk4::Grid::new();
    theme_grid.set_column_spacing(16);
    theme_grid.set_row_spacing(28);
    theme_grid.set_column_homogeneous(true);
    theme_grid.set_valign(gtk4::Align::Center);

    // Field 1: GTK Theme
    let gtk_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    let gtk_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.gtk_theme")));
    gtk_lbl.add_css_class("spec-label");
    gtk_lbl.set_halign(gtk4::Align::Start);
    gtk_box.append(&gtk_lbl);

    let gtk_items: Vec<&str> = gtk_themes.iter().map(|s| s.as_str()).collect();
    let gtk_model = gtk4::StringList::new(&gtk_items);
    let gtk_dropdown = gtk4::DropDown::new(Some(gtk_model), Option::<gtk4::Expression>::None);
    gtk_dropdown.set_cursor_from_name(Some("pointer"));
    gtk_box.append(&gtk_dropdown);
    theme_grid.attach(&gtk_box, 0, 0, 1, 1);
=======
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
>>>>>>> hard-develop

    // Field 2: Icon Theme
    let icon_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    let icon_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.icon_theme")));
    icon_lbl.add_css_class("spec-label");
    icon_lbl.set_halign(gtk4::Align::Start);
    icon_box.append(&icon_lbl);

<<<<<<< HEAD
    let icon_items: Vec<&str> = icon_themes.iter().map(|s| s.as_str()).collect();
    let icon_model = gtk4::StringList::new(&icon_items);
    let icon_dropdown = gtk4::DropDown::new(Some(icon_model), Option::<gtk4::Expression>::None);
    icon_dropdown.set_cursor_from_name(Some("pointer"));
    icon_box.append(&icon_dropdown);
    theme_grid.attach(&icon_box, 1, 0, 1, 1);

    // Field 3: Cursor Theme
    let cursor_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    let cursor_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.cursor_theme")));
    cursor_lbl.add_css_class("spec-label");
    cursor_lbl.set_halign(gtk4::Align::Start);
    cursor_box.append(&cursor_lbl);

    let cursor_items: Vec<&str> = cursor_themes.iter().map(|s| s.as_str()).collect();
    let cursor_model = gtk4::StringList::new(&cursor_items);
    let cursor_dropdown = gtk4::DropDown::new(Some(cursor_model), Option::<gtk4::Expression>::None);
    cursor_dropdown.set_cursor_from_name(Some("pointer"));
    cursor_box.append(&cursor_dropdown);
    theme_grid.attach(&cursor_box, 0, 1, 1, 1);

    // Field 4: Cursor Size
    let size_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    let size_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.cursor_size")));
    size_lbl.add_css_class("spec-label");
    size_lbl.set_halign(gtk4::Align::Start);
    size_box.append(&size_lbl);

    let size_strs: Vec<String> = cursor_sizes.iter().map(|s| format!("{} px", s)).collect();
    let size_items: Vec<&str> = size_strs.iter().map(|s| s.as_str()).collect();
    let size_model = gtk4::StringList::new(&size_items);
    let size_dropdown = gtk4::DropDown::new(Some(size_model), Option::<gtk4::Expression>::None);
    size_dropdown.set_cursor_from_name(Some("pointer"));
    size_box.append(&size_dropdown);
    theme_grid.attach(&size_box, 1, 1, 1, 1);

    top_grid.attach(&theme_grid, 1, 0, 1, 1);
    dashboard_panel.append(&top_grid);

    // Separator Line
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("profile-separator");
    dashboard_panel.append(&sep);

    // Quick Select Section Title
    let quick_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.quick_select")));
    quick_lbl.add_css_class("settings-row-title");
    quick_lbl.set_halign(gtk4::Align::Start);
    dashboard_panel.append(&quick_lbl);

    // Quick Select Box container for ~/.babydra/wallpaper grid
    let quick_select_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    quick_select_box.set_margin_top(4);
    quick_select_box.set_margin_end(4);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&quick_select_box));
    dashboard_panel.append(&scroll);

    main_box.append(&dashboard_panel);
=======
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
>>>>>>> hard-develop

    (
        main_box,
        preview_pic,
        pick_btn,
        theme_toggle_btn,
        gtk_dropdown,
        icon_dropdown,
        cursor_dropdown,
        size_dropdown,
        quick_select_box,
    )
}
