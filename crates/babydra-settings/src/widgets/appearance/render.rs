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
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);
    main_box.set_margin_start(16);
    main_box.set_margin_end(16);

    // Title
    let title_lbl = baby_utils::components::create_title("Wallpaper & Colors");
    main_box.append(&title_lbl);

    // Dashboard glass panel container
    let cc_box = baby_utils::components::create_card(gtk4::Orientation::Vertical, 20);
    cc_box.set_margin_top(24);
    cc_box.set_margin_bottom(24);
    cc_box.set_margin_start(24);
    cc_box.set_margin_end(24);

    // Three columns config row
    let config_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    config_row.set_homogeneous(true);

    // Column 1: Wallpaper Preview
    let preview_col = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    preview_col.add_css_class("wallpaper-preview-frame");
    preview_col.set_size_request(160, 110);

    let preview_img = gtk4::Image::new();
    if !current_wallpaper_path.is_empty() {
        let clean_path = current_wallpaper_path.replace("file://", "");
        preview_img.set_from_file(Some(&clean_path));
    } else {
        preview_img.set_icon_name(Some("image-missing"));
    }
    preview_img.set_pixel_size(100);
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

    let pick_icon = gtk4::Image::from_icon_name("folder-open-symbolic");
    pick_icon.set_pixel_size(24);
    pick_content.append(&pick_icon);

    let pick_lbl = gtk4::Label::new(Some("Choose File"));
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
    let light_icon = gtk4::Image::from_icon_name("weather-clear-symbolic");
    light_icon.set_pixel_size(16);
    light_content.append(&light_icon);
    let light_lbl = gtk4::Label::new(Some("Light"));
    light_lbl.add_css_class("settings-label");
    light_content.append(&light_lbl);
    light_card.set_child(Some(&light_content));
    theme_box.append(&light_card);

    let dark_card = gtk4::Button::new();
    dark_card.add_css_class("theme-option-card");
    let dark_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    dark_content.set_halign(gtk4::Align::Center);
    let dark_icon = gtk4::Image::from_icon_name("weather-clear-night-symbolic");
    dark_icon.set_pixel_size(16);
    dark_content.append(&dark_icon);
    let dark_lbl = gtk4::Label::new(Some("Dark"));
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
    let palette_icon = gtk4::Image::from_icon_name("preferences-desktop-theme-symbolic");
    palette_icon.set_pixel_size(16);
    gtk_left.append(&palette_icon);

    let gtk_lbl = gtk4::Label::new(Some("GTK Theme"));
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
