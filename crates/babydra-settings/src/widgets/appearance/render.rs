//! Appearance UI layout generator with local ~/.babydra/wallpaper grid.

use gtk4::prelude::*;

pub fn build_appearance_ui(
    current_wallpaper_path: &str,
    is_dark: bool,
    _themes: &[String],
    _current_theme: &str,
) -> (
    gtk4::Box,
    gtk4::Image,
    gtk4::Button,
    gtk4::Button,
    gtk4::Button,
    gtk4::Box,
) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);

    // Header Title: Icon + Wallpaper & Colors
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    let palette_icon = babydra_utils::ui::icon::get_icon("palette", 22);
    palette_icon.set_pixel_size(22);
    palette_icon.add_css_class("settings-row-icon");
    header_box.append(&palette_icon);

    let page_title = gtk4::Label::new(Some("Wallpaper & Colors"));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    header_box.append(&page_title);
    main_box.append(&header_box);

    // Dashboard Main Glass Panel
    let dashboard_panel = gtk4::Box::new(gtk4::Orientation::Vertical, 20);
    dashboard_panel.add_css_class("glass-panel");

    // Top Configuration Grid (3 Columns)
    let config_grid = gtk4::Grid::new();
    config_grid.set_column_spacing(16);
    config_grid.set_row_spacing(16);
    config_grid.set_column_homogeneous(true);

    // Column 1: Current Wallpaper Preview
    let preview_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    preview_box.add_css_class("wallpaper-preview-frame");
    preview_box.set_size_request(140, 100);

    let preview_lbl = gtk4::Label::new(Some("Current Wallpaper Preview"));
    preview_lbl.add_css_class("settings-row-desc");
    preview_lbl.set_halign(gtk4::Align::Start);
    preview_lbl.set_margin_start(8);
    preview_lbl.set_margin_top(6);
    preview_box.append(&preview_lbl);

    let preview_img = gtk4::Image::new();
    let clean_path = current_wallpaper_path.replace("file://", "");
    if !clean_path.is_empty() && std::path::Path::new(&clean_path).exists() {
        preview_img.set_from_file(Some(&clean_path));
        preview_img.set_pixel_size(64);
        preview_img.set_valign(gtk4::Align::Center);
        preview_img.set_halign(gtk4::Align::Center);
        preview_box.append(&preview_img);
    } else {
        let question_icon = babydra_utils::ui::icon::get_icon("display", 32);
        question_icon.set_pixel_size(32);
        question_icon.set_valign(gtk4::Align::Center);
        question_icon.set_halign(gtk4::Align::Center);
        preview_box.append(&question_icon);
    }
    config_grid.attach(&preview_box, 0, 0, 1, 1);

    // Column 2: Choose File Button Card
    let pick_btn = gtk4::Button::new();
    pick_btn.add_css_class("choose-file-card");
    pick_btn.set_cursor_from_name(Some("pointer"));

    let pick_content = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    pick_content.set_halign(gtk4::Align::Center);
    pick_content.set_valign(gtk4::Align::Center);

    let choose_icon = babydra_utils::ui::icon::get_icon("display", 24);
    choose_icon.set_pixel_size(24);
    choose_icon.set_halign(gtk4::Align::Center);
    pick_content.append(&choose_icon);

    let choose_lbl = gtk4::Label::new(Some("Choose File"));
    choose_lbl.add_css_class("settings-row-title");
    pick_content.append(&choose_lbl);

    pick_btn.set_child(Some(&pick_content));
    config_grid.attach(&pick_btn, 1, 0, 1, 1);

    // Column 3: Light & Dark Theme Cards
    let theme_cards_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);

    let light_card = gtk4::Button::new();
    light_card.add_css_class("theme-card-option");
    light_card.set_hexpand(true);
    light_card.set_cursor_from_name(Some("pointer"));

    let light_content = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    light_content.set_halign(gtk4::Align::Center);
    light_content.set_valign(gtk4::Align::Center);
    let light_icon = babydra_utils::ui::icon::get_icon("brightness", 20);
    light_icon.set_pixel_size(20);
    light_content.append(&light_icon);
    let light_lbl = gtk4::Label::new(Some("Light"));
    light_lbl.add_css_class("settings-row-title");
    light_content.append(&light_lbl);
    light_card.set_child(Some(&light_content));
    theme_cards_box.append(&light_card);

    let dark_card = gtk4::Button::new();
    dark_card.add_css_class("theme-card-option");
    dark_card.set_hexpand(true);
    dark_card.set_cursor_from_name(Some("pointer"));

    let dark_content = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    dark_content.set_halign(gtk4::Align::Center);
    dark_content.set_valign(gtk4::Align::Center);
    let dark_icon = babydra_utils::ui::icon::get_icon("dark-mode", 20);
    dark_icon.set_pixel_size(20);
    dark_content.append(&dark_icon);
    let dark_lbl = gtk4::Label::new(Some("Dark"));
    dark_lbl.add_css_class("settings-row-title");
    dark_content.append(&dark_lbl);
    dark_card.set_child(Some(&dark_content));
    theme_cards_box.append(&dark_card);

    if is_dark {
        dark_card.add_css_class("active-dark");
        light_card.remove_css_class("active-dark");
    } else {
        light_card.add_css_class("active-dark");
        dark_card.remove_css_class("active-dark");
    }

    config_grid.attach(&theme_cards_box, 2, 0, 1, 1);
    dashboard_panel.append(&config_grid);

    // Transparency Control Row
    let trans_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    trans_row.set_margin_top(8);
    trans_row.set_margin_bottom(8);

    let eye_icon = babydra_utils::ui::icon::get_icon("info", 18);
    eye_icon.set_pixel_size(18);
    eye_icon.set_valign(gtk4::Align::Center);
    eye_icon.add_css_class("settings-row-icon");
    trans_row.append(&eye_icon);

    let trans_lbl = gtk4::Label::new(Some("Transparency"));
    trans_lbl.add_css_class("settings-row-title");
    trans_lbl.set_valign(gtk4::Align::Center);
    trans_lbl.set_hexpand(true);
    trans_lbl.set_halign(gtk4::Align::Start);
    trans_row.append(&trans_lbl);

    let trans_switch = gtk4::Switch::new();
    trans_switch.set_active(true);
    trans_switch.set_valign(gtk4::Align::Center);
    trans_switch.set_cursor_from_name(Some("pointer"));
    trans_row.append(&trans_switch);

    dashboard_panel.append(&trans_row);

    // Separator Line
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("profile-separator");
    dashboard_panel.append(&sep);

    // Quick Select Section Title
    let quick_lbl = gtk4::Label::new(Some("Quick select"));
    quick_lbl.add_css_class("settings-row-title");
    quick_lbl.set_halign(gtk4::Align::Start);
    dashboard_panel.append(&quick_lbl);

    // Quick Select Box container for ~/.babydra/wallpaper grid
    let quick_select_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    quick_select_box.set_margin_top(8);
    dashboard_panel.append(&quick_select_box);

    main_box.append(&dashboard_panel);

    (
        main_box,
        preview_img,
        pick_btn,
        light_card,
        dark_card,
        quick_select_box,
    )
}
