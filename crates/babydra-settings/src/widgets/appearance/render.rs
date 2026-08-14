//! Appearance UI layout generator with local ~/.babydra/wallpaper grid and Greeter settings.

use gtk4::prelude::*;

pub fn build_appearance_ui(
    current_wallpaper_path: &str,
    _current_greeter_path: &str,
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
    gtk4::DropDown,
    gtk4::Box,
    gtk4::Picture,
    gtk4::Button,
) {
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

    // Top-Left Dropdown Container inside Preview Overlay
    let top_left_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    top_left_box.set_valign(gtk4::Align::Start);
    top_left_box.set_halign(gtk4::Align::Start);
    top_left_box.set_margin_start(10);
    top_left_box.set_margin_top(10);

    let target_items = vec![
        babydra_common::i18n::t("settings.target_desktop"),
        babydra_common::i18n::t("settings.target_lock"),
    ];
    let target_item_strs: Vec<&str> = target_items.iter().map(|s| s.as_str()).collect();
    let target_model = gtk4::StringList::new(&target_item_strs);
    let target_dropdown = gtk4::DropDown::new(Some(target_model), Option::<gtk4::Expression>::None);
    target_dropdown.add_css_class("wallpaper-target-dropdown");
    target_dropdown.set_cursor_from_name(Some("pointer"));

    top_left_box.append(&target_dropdown);
    preview_overlay.add_overlay(&top_left_box);

    // Vertical Overlay Actions Column on the Right Edge
    let actions_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    actions_box.set_valign(gtk4::Align::End);
    actions_box.set_halign(gtk4::Align::End);
    actions_box.set_margin_end(10);
    actions_box.set_margin_bottom(10);

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
    
    // Top-Right Overlay Container for Avatar
    let top_right_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    top_right_box.set_valign(gtk4::Align::Start);
    top_right_box.set_halign(gtk4::Align::End);
    top_right_box.set_margin_end(10);
    top_right_box.set_margin_top(10);

    let avatar_pic = gtk4::Picture::new();
    avatar_pic.set_size_request(42, 42);
    avatar_pic.add_css_class("avatar-preview-picture");
    if let Some(bytes) = babydra_common::get_avatar_bytes() {
        if let Some(pixbuf) = babydra_common::crop_to_circle_pixbuf(&bytes, 42) {
            avatar_pic.set_pixbuf(Some(&pixbuf));
        }
    }
    
    let avatar_btn = gtk4::Button::new();
    avatar_btn.set_child(Some(&avatar_pic));
    avatar_btn.set_size_request(42, 42);
    avatar_btn.set_valign(gtk4::Align::Center);
    avatar_btn.set_halign(gtk4::Align::Center);
    avatar_btn.add_css_class("avatar-action-btn");
    avatar_btn.set_cursor_from_name(Some("pointer"));
    
    top_right_box.append(&avatar_btn);
    preview_overlay.add_overlay(&top_right_box);

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

    // Field 2: Icon Theme
    let icon_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    let icon_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.icon_theme")));
    icon_lbl.add_css_class("spec-label");
    icon_lbl.set_halign(gtk4::Align::Start);
    icon_box.append(&icon_lbl);

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
    let sep1 = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep1.add_css_class("profile-separator");
    dashboard_panel.append(&sep1);

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

    (
        main_box,
        preview_pic,
        pick_btn,
        theme_toggle_btn,
        gtk_dropdown,
        icon_dropdown,
        cursor_dropdown,
        size_dropdown,
        target_dropdown,
        quick_select_box,
        avatar_pic,
        avatar_btn,
    )
}
