//! Appearance UI layout generator matching AppearanceView.vue.

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
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);

    let header_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    let title_lbl = babydra_utils::components::create_title("Giao diện & Hình nền");
    let desc_lbl = gtk4::Label::new(Some("Tùy chỉnh hình nền, chủ đề sáng/tối và giao diện ứng dụng"));
    desc_lbl.add_css_class("settings-header-desc");
    desc_lbl.set_halign(gtk4::Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    main_box.append(&header_box);

    // Dashboard glass panel container
    let cc_box = babydra_utils::components::create_card(gtk4::Orientation::Vertical, 20);
    cc_box.add_css_class("settings-card");

    // Three columns config row
    let config_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    config_row.set_homogeneous(true);

    // Column 1: Wallpaper Preview
    let preview_col = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    preview_col.add_css_class("wallpaper-preview-frame");
    preview_col.set_size_request(160, 110);

    let preview_img = gtk4::Image::new();
    let clean_path = current_wallpaper_path.replace("file://", "");
    if !clean_path.is_empty() && std::path::Path::new(&clean_path).exists() {
        preview_img.set_from_file(Some(&clean_path));
        preview_img.set_pixel_size(110);
    } else {
        let display_icon = babydra_utils::ui::icon::get_icon("display", 48);
        display_icon.set_pixel_size(48);
        display_icon.set_valign(gtk4::Align::Center);
        display_icon.set_halign(gtk4::Align::Center);
        preview_col.append(&display_icon);
    }
    preview_img.set_valign(gtk4::Align::Center);
    preview_img.set_halign(gtk4::Align::Center);
    preview_col.append(&preview_img);
    config_row.append(&preview_col);

    // Column 2: Choose File Button
    let pick_btn = gtk4::Button::new();
    pick_btn.add_css_class("choose-file-card");
    pick_btn.set_size_request(160, 110);

    let pick_content = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    pick_content.set_valign(gtk4::Align::Center);
    pick_content.set_halign(gtk4::Align::Center);

    let pick_icon = babydra_utils::ui::icon::get_icon("folder", 24);
    pick_icon.set_pixel_size(24);
    pick_content.append(&pick_icon);

    let pick_lbl = gtk4::Label::new(Some("Chọn hình nền"));
    pick_lbl.add_css_class("settings-label");
    pick_content.append(&pick_lbl);

    pick_btn.set_child(Some(&pick_content));
    config_row.append(&pick_btn);

    // Column 3: Light/Dark selector cards
    let theme_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    theme_box.set_size_request(160, 110);

    let light_card = gtk4::Button::new();
    light_card.add_css_class("theme-option-card");
    let light_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    light_content.set_halign(gtk4::Align::Center);
    let light_icon = babydra_utils::ui::icon::get_icon("brightness", 16);
    light_icon.set_pixel_size(16);
    light_content.append(&light_icon);
    let light_lbl = gtk4::Label::new(Some("Chế độ Sáng"));
    light_lbl.add_css_class("settings-label");
    light_content.append(&light_lbl);
    light_card.set_child(Some(&light_content));
    theme_box.append(&light_card);

    let dark_card = gtk4::Button::new();
    dark_card.add_css_class("theme-option-card");
    let dark_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    dark_content.set_halign(gtk4::Align::Center);
    let dark_icon = babydra_utils::ui::icon::get_icon("dark-mode", 16);
    dark_icon.set_pixel_size(16);
    dark_content.append(&dark_icon);
    let dark_lbl = gtk4::Label::new(Some("Chế độ Tối"));
    dark_lbl.add_css_class("settings-label");
    dark_content.append(&dark_lbl);
    dark_card.set_child(Some(&dark_content));
    theme_box.append(&dark_card);

    if is_dark {
        dark_card.add_css_class("active");
        light_card.remove_css_class("active");
    } else {
        light_card.add_css_class("active");
        dark_card.remove_css_class("active");
    }

    config_row.append(&theme_box);
    cc_box.append(&config_row);

    // GTK Theme Selector
    let gtk_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    gtk_row.add_css_class("settings-row-item");
    gtk_row.set_margin_top(8);

    let gtk_left = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);

    let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    icon_badge.add_css_class("sidebar-icon-badge");
    icon_badge.add_css_class("badge-pink");
    icon_badge.set_valign(gtk4::Align::Center);

    let palette_icon = babydra_utils::ui::icon::get_icon("settings", 16);
    palette_icon.set_pixel_size(16);
    icon_badge.append(&palette_icon);
    gtk_left.append(&icon_badge);

    let gtk_lbl = gtk4::Label::new(Some("Giao diện GTK Theme"));
    gtk_lbl.add_css_class("settings-label");
    gtk_left.append(&gtk_lbl);
    gtk_row.append(&gtk_left);

    let spacer2 = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer2.set_hexpand(true);
    gtk_row.append(&spacer2);

    let dropdown = gtk4::DropDown::from_strings(&themes.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    dropdown.set_valign(gtk4::Align::Center);
    if let Some(pos) = themes.iter().position(|t| t == current_theme) {
        dropdown.set_selected(pos as u32);
    }
    gtk_row.append(&dropdown);
    cc_box.append(&gtk_row);

    main_box.append(&cc_box);

    (
        main_box,
        preview_img,
        pick_btn,
        light_card,
        dark_card,
        dropdown,
    )
}
