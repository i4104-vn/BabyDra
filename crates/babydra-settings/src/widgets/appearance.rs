//! Appearance and themes personalization panel.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;

fn get_color_scheme() -> String {
    let output = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "color-scheme"])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let raw = stdout.trim().replace("'", "");
            if raw == "prefer-dark" {
                "dark".to_string()
            } else {
                "light".to_string()
            }
        }
        Err(_) => "dark".to_string(),
    }
}

fn set_color_scheme(dark: bool) {
    let scheme = if dark { "prefer-dark" } else { "prefer-light" };
    let _ = Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "color-scheme", scheme])
        .output();
}

fn get_gtk_themes() -> Vec<String> {
    let mut themes = vec!["Adwaita".to_string(), "Adwaita-dark".to_string()];
    if let Ok(entries) = std::fs::read_dir("/usr/share/themes") {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().to_string();
            if !themes.contains(&name) && !name.starts_with('.') {
                themes.push(name);
            }
        }
    }
    themes.sort();
    themes
}

fn set_gtk_theme(theme_name: &str) {
    // 1. Update gsettings
    let _ = Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "gtk-theme", theme_name])
        .output();

    // 2. Update config files
    let home = std::env::var("HOME").unwrap_or_default();
    let set_ini = |path: &str| {
        let full_path = std::path::PathBuf::from(&home).join(path);
        let _ = std::fs::create_dir_all(full_path.parent().unwrap());
        let content = format!("[Settings]\ngtk-theme-name={}\ngtk-font-name=Inter 11\n", theme_name);
        let _ = std::fs::write(full_path, content);
    };

    set_ini(".config/gtk-3.0/settings.ini");
    set_ini(".config/gtk-4.0/settings.ini");
}

pub fn create_appearance_widget() -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = gtk4::Label::new(Some("Giao diện & Cá nhân hóa"));
    title_lbl.add_css_class("settings-title");
    title_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&title_lbl);

    let is_dark = get_color_scheme() == "dark";

    // --- Dark/Light Mode Row ---
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
    dark_switch.set_active(is_dark);
    dark_switch.set_valign(gtk4::Align::Center);
    theme_card.append(&dark_switch);

    main_box.append(&theme_card);

    // --- Wallpaper Row ---
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

    // --- GTK Theme Row ---
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

    let themes = get_gtk_themes();
    let dropdown = gtk4::DropDown::from_strings(&themes.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    dropdown.set_valign(gtk4::Align::Center);
    
    // Find active GTK theme
    let current_theme = match Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output() 
    {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().replace("'", ""),
        Err(_) => "Adwaita".to_string(),
    };
    if let Some(pos) = themes.iter().position(|t| t == &current_theme) {
        dropdown.set_selected(pos as u32);
    }
    gtk_card.append(&dropdown);

    main_box.append(&gtk_card);

    // --- Event Connectors ---
    dark_switch.connect_state_set(move |_, is_active| {
        set_color_scheme(is_active);
        // Reload shell theme manager
        babydra_common::init_theme();
        glib::Propagation::Proceed
    });

    let wp_card_parent = wp_card.clone();
    select_wp_btn.connect_clicked(move |_| {
        if let Some(win) = wp_card_parent.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
            let file_dialog = gtk4::FileDialog::new();
            file_dialog.set_title("Chọn hình nền");

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some("Hình ảnh (*.png, *.jpg, *.jpeg, *.webp)"));
            filter.add_mime_type("image/png");
            filter.add_mime_type("image/jpeg");
            filter.add_mime_type("image/webp");
            file_dialog.set_default_filter(Some(&filter));

            file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let _ = babydra_common::set_wallpaper(&path);
                    }
                }
            });
        }
    });

    dropdown.connect_selected_notify(move |dd| {
        if let Some(selected_str) = dd.selected_item().and_downcast::<gtk4::StringObject>().map(|o| o.string()) {
            set_gtk_theme(&selected_str);
        }
    });

    main_box
}
