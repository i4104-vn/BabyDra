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

fn get_current_wallpaper() -> String {
    if let Some(path) = babydra_common::get_current_wallpaper() {
        return path.to_string_lossy().to_string();
    }
    "".to_string()
}

pub fn create_appearance_widget() -> gtk4::Box {
    let gtk_themes = babydra_common::services::system::theme::get_gtk_themes();
    let icon_themes = babydra_common::services::system::theme::get_icon_themes();
    let cursor_themes = babydra_common::services::system::theme::get_cursor_themes();
    let cursor_sizes = vec![16, 24, 32, 48, 64];

    let wp_path = get_current_wallpaper();
    let is_dark = babydra_utils::ui::theme::is_dark_mode();

    let (
        main_box,
        preview_pic,
        pick_btn,
        theme_toggle_btn,
        gtk_dropdown,
        icon_dropdown,
        cursor_dropdown,
        size_dropdown,
        quick_select_box,
    ) = render::build_appearance_ui(
        &wp_path,
        is_dark,
        &gtk_themes,
        &icon_themes,
        &cursor_themes,
        &cursor_sizes,
    );

    // Auto-apply system theme when dropdown selection changes
    let gtk_themes_c = gtk_themes.clone();
    let icon_themes_c = icon_themes.clone();
    let cursor_themes_c = cursor_themes.clone();
    let cursor_sizes_c = cursor_sizes.clone();

    let gtk_d = gtk_dropdown.clone();
    let icon_d = icon_dropdown.clone();
    let cursor_d = cursor_dropdown.clone();
    let size_d = size_dropdown.clone();

    let apply_theme_settings = move || {
        let gtk_idx = gtk_d.selected() as usize;
        let icon_idx = icon_d.selected() as usize;
        let cursor_idx = cursor_d.selected() as usize;
        let size_idx = size_d.selected() as usize;

        let selected_gtk = gtk_themes_c.get(gtk_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_icon = icon_themes_c.get(icon_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_cursor = cursor_themes_c.get(cursor_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_size = cursor_sizes_c.get(size_idx).cloned().unwrap_or(24);

        let _ = babydra_common::services::system::theme::apply_appearance(
            &selected_gtk,
            &selected_icon,
            &selected_cursor,
            selected_size,
        );
    };

    let apply_cb1 = apply_theme_settings.clone();
    gtk_dropdown.connect_selected_notify(move |_| apply_cb1());

    let apply_cb2 = apply_theme_settings.clone();
    icon_dropdown.connect_selected_notify(move |_| apply_cb2());

    let apply_cb3 = apply_theme_settings.clone();
    cursor_dropdown.connect_selected_notify(move |_| apply_cb3());

    let apply_cb4 = apply_theme_settings.clone();
    size_dropdown.connect_selected_notify(move |_| apply_cb4());

    // Theme Toggle Icon Button Click Handler
    let theme_btn_clone = theme_toggle_btn.clone();
    theme_toggle_btn.connect_clicked(move |_| {
        let currently_dark = babydra_utils::ui::theme::is_dark_mode();
        let new_dark = !currently_dark;
        babydra_utils::ui::theme::set_dark_mode(new_dark);
        babydra_utils::ui::theme::init_theme();

        let new_icon_name = if new_dark { "brightness" } else { "dark-mode" };
        let new_icon = babydra_utils::ui::icon::get_icon(new_icon_name, 18);
        new_icon.set_pixel_size(18);
        new_icon.set_valign(gtk4::Align::Center);
        new_icon.set_halign(gtk4::Align::Center);
        theme_btn_clone.set_child(Some(&new_icon));
    });

    // Helper closure to render wallpapers grid in quick_select_box
    let render_wallpapers_grid = {
        let quick_select_box_clone = quick_select_box.clone();
        let preview_pic_clone = preview_pic.clone();
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

                let title = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.no_wallpapers")));
                title.add_css_class("settings-row-title");
                empty_box.append(&title);

                let sub = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.no_wallpapers_sub")));
                sub.add_css_class("settings-row-desc");
                empty_box.append(&sub);

                quick_select_box_clone.append(&empty_box);
            } else {
                let flow = gtk4::FlowBox::new();
                flow.set_selection_mode(gtk4::SelectionMode::None);
                flow.set_max_children_per_line(4);
                flow.set_min_children_per_line(4);
                flow.set_column_spacing(12);
                flow.set_row_spacing(12);
                flow.set_homogeneous(true);

                for wp in wallpapers {
                    let btn = gtk4::Button::new();
                    btn.add_css_class("wallpaper-thumb-card");
                    btn.set_cursor_from_name(Some("pointer"));

                    let pic = gtk4::Picture::for_filename(&wp);
                    pic.set_size_request(130, 105);
                    pic.set_content_fit(gtk4::ContentFit::Cover);

                    btn.set_child(Some(&pic));

                    let wp_clone = wp.clone();
                    let preview_cb = preview_pic_clone.clone();
                    btn.connect_clicked(move |_| {
                        let _ = babydra_common::set_wallpaper(&wp_clone);
                        preview_cb.set_filename(Some(&wp_clone));
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
    let preview_clone = preview_pic.clone();
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
                            preview_cb.set_filename(Some(&dest_path));
                            render_grid_after_pick();
                        }
                    }
                }
            });
        }
    });

    main_box
}
