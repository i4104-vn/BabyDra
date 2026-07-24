//! Appearance and themes personalization panel.

use gtk4::prelude::*;
use std::process::Command;
use std::path::PathBuf;

mod render;

fn get_wallpaper_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let dir = PathBuf::from(home).join(".babydra").join("wallpaper");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn get_local_wallpapers() -> Vec<PathBuf> {
    let dir = get_wallpaper_dir();
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "webp") {
                        files.push(path);
                    }
                }
            }
        }
    }
    files.sort();
    files
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
    let is_dark = babydra_utils::ui::theme::is_dark_mode();

    let (
        main_box,
        preview_img,
        pick_btn,
        light_card,
        dark_card,
        quick_select_box,
    ) = render::build_appearance_ui(&wp_path, is_dark, &themes, &current_theme);

    // Light theme button
    let light_card_clone = light_card.clone();
    let dark_card_clone = dark_card.clone();
    light_card.connect_clicked(move |_| {
        babydra_utils::ui::theme::set_dark_mode(false);
        babydra_utils::ui::theme::init_theme();
        light_card_clone.add_css_class("active-dark");
        dark_card_clone.remove_css_class("active-dark");
    });

    // Dark theme button
    let light_card_clone2 = light_card.clone();
    let dark_card_clone2 = dark_card.clone();
    dark_card.connect_clicked(move |_| {
        babydra_utils::ui::theme::set_dark_mode(true);
        babydra_utils::ui::theme::init_theme();
        dark_card_clone2.add_css_class("active-dark");
        light_card_clone2.remove_css_class("active-dark");
    });

    // Helper closure to render wallpapers grid in quick_select_box
    let render_wallpapers_grid = {
        let quick_select_box_clone = quick_select_box.clone();
        let preview_img_clone = preview_img.clone();
        move || {
            while let Some(child) = quick_select_box_clone.first_child() {
                quick_select_box_clone.remove(&child);
            }

            let wallpapers = get_local_wallpapers();
            if wallpapers.is_empty() {
                let empty_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
                empty_box.set_halign(gtk4::Align::Center);
                empty_box.set_valign(gtk4::Align::Center);
                empty_box.set_margin_top(24);
                empty_box.set_margin_bottom(24);

                let icon = babydra_utils::ui::icon::get_icon("display", 28);
                icon.set_pixel_size(28);
                icon.set_halign(gtk4::Align::Center);
                empty_box.append(&icon);

                let title = gtk4::Label::new(Some("Chưa có hình nền trong ~/.babydra/wallpaper"));
                title.add_css_class("settings-row-title");
                empty_box.append(&title);

                let sub = gtk4::Label::new(Some("Bấm 'Choose File' phía trên để thêm hình nền mới"));
                sub.add_css_class("settings-row-desc");
                empty_box.append(&sub);

                quick_select_box_clone.append(&empty_box);
            } else {
                let flow = gtk4::FlowBox::new();
                flow.set_selection_mode(gtk4::SelectionMode::None);
                flow.set_max_children_per_line(4);
                flow.set_min_children_per_line(2);
                flow.set_column_spacing(12);
                flow.set_row_spacing(12);
                flow.set_homogeneous(true);

                for wp in wallpapers {
                    let btn = gtk4::Button::new();
                    btn.add_css_class("wallpaper-thumb-card");
                    btn.set_cursor_from_name(Some("pointer"));

                    let pic = gtk4::Picture::for_filename(&wp);
                    pic.set_size_request(120, 75);
                    pic.set_content_fit(gtk4::ContentFit::Cover);

                    btn.set_child(Some(&pic));

                    let wp_clone = wp.clone();
                    let preview_cb = preview_img_clone.clone();
                    btn.connect_clicked(move |_| {
                        let _ = babydra_common::set_wallpaper(&wp_clone);
                        preview_cb.set_from_file(Some(&wp_clone));
                    });

                    flow.insert(&btn, -1);
                }

                quick_select_box_clone.append(&flow);
            }
        }
    };

    // Initial render of local wallpaper grid
    render_wallpapers_grid();

    // Pick Wallpaper Button Click
    let preview_clone = preview_img.clone();
    let parent_box = main_box.clone();
    let render_grid_cb = render_wallpapers_grid.clone();
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
            let render_grid_after_pick = render_grid_cb.clone();
            file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let target_dir = get_wallpaper_dir();
                        if let Some(filename) = path.file_name() {
                            let dest_path = target_dir.join(filename);
                            if path != dest_path {
                                let _ = std::fs::copy(&path, &dest_path);
                            }
                            let _ = babydra_common::set_wallpaper(&dest_path);
                            preview_cb.set_from_file(Some(&dest_path));
                            render_grid_after_pick();
                        }
                    }
                }
            });
        }
    });

    main_box
}
