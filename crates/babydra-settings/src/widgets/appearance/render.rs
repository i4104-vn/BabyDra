//! Appearance UI layout generator.

use gtk4::prelude::*;

pub fn build_appearance_ui(themes: &[String], current_theme: &str) -> (gtk4::Box, gtk4::Switch, gtk4::Button, gtk4::DropDown) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = gtk4::Label::new(Some("Giao diện & Cá nhân hóa"));
    title_lbl.add_css_class("settings-title");
    title_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&title_lbl);

    // Dark Mode Row
    let theme_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    theme_card.add_css_class("settings-card");
    theme_card.set_valign(gtk4::Align::Center);

    let theme_label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let theme_title = gtk4::Label::new(Some("Giao diện tối (Dark Mode)"));
    theme_title.add_css_class("settings-label");
    theme_title.set_halign(gtk4::Align::Start);
    let theme_desc = gtk4::Label::new(Some("Chuyển đổi giao diện hệ thống giữa sáng và tối"));
    theme_desc.add_css_class("settings-desc");
    theme_desc.set_halign(gtk4::Align::Start);
    theme_label_box.append(&theme_title);
    theme_label_box.append(&theme_desc);
    theme_card.append(&theme_label_box);

    let theme_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    theme_spacer.set_hexpand(true);
    theme_card.append(&theme_spacer);

    let dark_switch = gtk4::Switch::new();
    dark_switch.set_valign(gtk4::Align::Center);
    theme_card.append(&dark_switch);
    main_box.append(&theme_card);

    // Wallpaper Row
    let wp_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    wp_card.add_css_class("settings-card");
    wp_card.set_valign(gtk4::Align::Center);

    let wp_label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let wp_title = gtk4::Label::new(Some("Hình nền máy tính"));
    wp_title.add_css_class("settings-label");
    wp_title.set_halign(gtk4::Align::Start);
    let wp_desc = gtk4::Label::new(Some("Thay đổi hình nền nền màn hình chính"));
    wp_desc.add_css_class("settings-desc");
    wp_desc.set_halign(gtk4::Align::Start);
    wp_label_box.append(&wp_title);
    wp_label_box.append(&wp_desc);
    wp_card.append(&wp_label_box);

    let wp_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    wp_spacer.set_hexpand(true);
    wp_card.append(&wp_spacer);

    let select_wp_btn = gtk4::Button::with_label("Chọn ảnh...");
    select_wp_btn.set_valign(gtk4::Align::Center);
    select_wp_btn.add_css_class("suggested-action");
    wp_card.append(&select_wp_btn);
    main_box.append(&wp_card);

    // GTK Theme Row
    let gtk_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    gtk_card.add_css_class("settings-card");
    gtk_card.set_valign(gtk4::Align::Center);

    let gtk_label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let gtk_title = gtk4::Label::new(Some("GTK Theme"));
    gtk_title.add_css_class("settings-label");
    gtk_title.set_halign(gtk4::Align::Start);
    let gtk_desc = gtk4::Label::new(Some("Thay đổi kiểu dáng các ứng dụng GTK"));
    gtk_desc.add_css_class("settings-desc");
    gtk_desc.set_halign(gtk4::Align::Start);
    gtk_label_box.append(&gtk_title);
    gtk_label_box.append(&gtk_desc);
    gtk_card.append(&gtk_label_box);

    let gtk_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    gtk_spacer.set_hexpand(true);
    gtk_card.append(&gtk_spacer);

    let dropdown = gtk4::DropDown::from_strings(&themes.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    dropdown.set_valign(gtk4::Align::Center);
    if let Some(pos) = themes.iter().position(|t| t == current_theme) {
        dropdown.set_selected(pos as u32);
    }
    gtk_card.append(&dropdown);
    main_box.append(&gtk_card);

    (main_box, dark_switch, select_wp_btn, dropdown)
}
