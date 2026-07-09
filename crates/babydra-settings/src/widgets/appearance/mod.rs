//! Appearance and themes personalization panel.

use gtk4::prelude::*;
use std::process::Command;

mod render;

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
    let _ = Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "gtk-theme", theme_name])
        .output();

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

fn get_current_wallpaper() -> String {
    let output = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.background", "picture-uri"])
        .output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.trim().replace("'", "").replace("file://", "")
        }
        Err(_) => "".to_string(),
    }
}

pub fn create_appearance_widget() -> gtk4::Box {
    let themes = get_gtk_themes();
    let current_theme = match Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output() 
    {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().replace("'", ""),
        Err(_) => "Adwaita".to_string(),
    };

    let wp_path = get_current_wallpaper();
    let is_dark = get_color_scheme() == "dark";

    let (
        main_box,
        preview_img,
        pick_btn,
        light_card,
        dark_card,
        trans_switch,
        dropdown,
    ) = render::build_appearance_ui(&wp_path, is_dark, &themes, &current_theme);

    // Light theme button
    let light_card_clone = light_card.clone();
    let dark_card_clone = dark_card.clone();
    light_card.connect_clicked(move |_| {
        set_color_scheme(false);
        babydra_common::init_theme();
        light_card_clone.add_css_class("active");
        dark_card_clone.remove_css_class("active");
    });

    // Dark theme button
    let light_card_clone2 = light_card.clone();
    let dark_card_clone2 = dark_card.clone();
    dark_card.connect_clicked(move |_| {
        set_color_scheme(true);
        babydra_common::init_theme();
        dark_card_clone2.add_css_class("active");
        light_card_clone2.remove_css_class("active");
    });

    // Pick Wallpaper
    let preview_clone = preview_img.clone();
    let parent_box = main_box.clone();
    pick_btn.connect_clicked(move |_| {
        if let Some(win) = parent_box.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
            let file_dialog = gtk4::FileDialog::new();
            file_dialog.set_title("Chọn hình nền");

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some("Hình ảnh (*.png, *.jpg, *.jpeg, *.webp)"));
            filter.add_mime_type("image/png");
            filter.add_mime_type("image/jpeg");
            filter.add_mime_type("image/webp");
            file_dialog.set_default_filter(Some(&filter));

            let preview_cb = preview_clone.clone();
            file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let _ = babydra_common::set_wallpaper(&path);
                        preview_cb.set_from_file(Some(&path));
                    }
                }
            });
        }
    });

    trans_switch.connect_state_set(move |_, is_active| {
        let alpha = if is_active { "0.85" } else { "1.00" };
        let _ = Command::new("gsettings")
            .args(&["set", "org.gnome.desktop.interface", "text-scaling-factor", alpha]) // dummy or transparency toggle mock
            .output();
        glib::Propagation::Proceed
    });

    dropdown.connect_selected_notify(move |dd| {
        if let Some(selected_str) = dd.selected_item().and_downcast::<gtk4::StringObject>().map(|o| o.string()) {
            set_gtk_theme(&selected_str);
        }
    });

    main_box
}
