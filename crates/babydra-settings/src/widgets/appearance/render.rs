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
    let dashboard_panel = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    dashboard_panel.add_css_class("glass-panel");

    // Wallpaper Preview Picture with Overlay Actions (Plus Button & Theme Toggle Button)
    let preview_overlay = gtk4::Overlay::new();
    preview_overlay.add_css_class("wallpaper-preview-overlay");
    preview_overlay.set_size_request(-1, 140);

    let preview_pic = gtk4::Picture::new();
    preview_pic.set_size_request(-1, 140);
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
    dashboard_panel.append(&preview_overlay);

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
    dashboard_panel.append(&quick_select_box);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&dashboard_panel));

    main_box.append(&scroll);

    (
        main_box,
        preview_pic,
        pick_btn,
        theme_toggle_btn,
        quick_select_box,
    )
}
