//! Appearance UI layout generator with local ~/.babydra/wallpaper grid.

use gtk4::prelude::*;

pub fn build_appearance_ui(
    current_wallpaper_path: &str,
    is_dark: bool,
    _themes: &[String],
    _current_theme: &str,
) -> (
    gtk4::Box,
    gtk4::Picture,
    gtk4::Button,
    gtk4::Button,
    gtk4::Button,
    gtk4::Box,
) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Title: Wallpaper & Colors (matching VPN, Bluetooth & Themes header)
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let page_title = gtk4::Label::new(Some("Wallpaper & Colors"));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    header_box.append(&page_title);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_box.append(&spacer);

    main_box.append(&header_box);

    // Dashboard Main Glass Panel
    let dashboard_panel = gtk4::Box::new(gtk4::Orientation::Vertical, 20);
    dashboard_panel.add_css_class("glass-panel");
    dashboard_panel.set_vexpand(true);
    dashboard_panel.set_valign(gtk4::Align::Fill);

    // Top Configuration Grid (2 Columns)
    let config_grid = gtk4::Grid::new();
    config_grid.set_column_spacing(20);
    config_grid.set_row_spacing(16);
    config_grid.set_column_homogeneous(true);

    // Column 1: Wallpaper Preview Picture with Vertical Plus Button Overlayed inside Right Edge
    let preview_overlay = gtk4::Overlay::new();
    preview_overlay.add_css_class("wallpaper-preview-overlay");
    preview_overlay.set_size_request(-1, 120);
    preview_overlay.set_valign(gtk4::Align::Center);

    let preview_pic = gtk4::Picture::new();
    preview_pic.set_size_request(-1, 120);
    preview_pic.set_content_fit(gtk4::ContentFit::Cover);
    preview_pic.add_css_class("wallpaper-preview-picture");

    let clean_path = current_wallpaper_path.replace("file://", "");
    if !clean_path.is_empty() && std::path::Path::new(&clean_path).exists() {
        preview_pic.set_filename(Some(&clean_path));
    }
    preview_overlay.set_child(Some(&preview_pic));

    // Vertical Plus Button overlayed INSIDE the right edge
    let pick_btn = gtk4::Button::new();
    pick_btn.add_css_class("wallpaper-pick-overlay-inside-btn");
    pick_btn.set_cursor_from_name(Some("pointer"));
    pick_btn.set_valign(gtk4::Align::Fill);
    pick_btn.set_halign(gtk4::Align::End);
    pick_btn.set_margin_top(8);
    pick_btn.set_margin_bottom(8);
    pick_btn.set_margin_end(8);
    pick_btn.set_size_request(40, -1);

    let plus_content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    plus_content.set_halign(gtk4::Align::Center);
    plus_content.set_valign(gtk4::Align::Center);

    let plus_icon = babydra_utils::ui::icon::get_icon("plus", 22);
    plus_icon.set_pixel_size(22);
    plus_icon.set_valign(gtk4::Align::Center);
    plus_icon.set_halign(gtk4::Align::Center);
    plus_content.append(&plus_icon);

    pick_btn.set_child(Some(&plus_content));
    preview_overlay.add_overlay(&pick_btn);

    config_grid.attach(&preview_overlay, 0, 0, 1, 1);

    // Column 2: Light & Dark Theme Cards (Increased size)
    let theme_cards_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    theme_cards_box.set_valign(gtk4::Align::Center);

    let light_card = gtk4::Button::new();
    light_card.add_css_class("theme-card-option");
    light_card.set_hexpand(true);
    light_card.set_cursor_from_name(Some("pointer"));

    let light_content = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    light_content.set_halign(gtk4::Align::Center);
    light_content.set_valign(gtk4::Align::Center);
    let light_icon = babydra_utils::ui::icon::get_icon("brightness", 28);
    light_icon.set_pixel_size(28);
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

    let dark_content = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    dark_content.set_halign(gtk4::Align::Center);
    dark_content.set_valign(gtk4::Align::Center);
    let dark_icon = babydra_utils::ui::icon::get_icon("dark-mode", 28);
    dark_icon.set_pixel_size(28);
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

    config_grid.attach(&theme_cards_box, 1, 0, 1, 1);
    dashboard_panel.append(&config_grid);

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
    quick_select_box.set_margin_top(4);
    quick_select_box.set_margin_end(4);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&quick_select_box));

    dashboard_panel.append(&scroll);
    main_box.append(&dashboard_panel);

    (
        main_box,
        preview_pic,
        pick_btn,
        light_card,
        dark_card,
        quick_select_box,
    )
}
