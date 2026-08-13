use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use babydra_common::services::wallpaper::{get_wallpaper_dir, get_local_wallpapers};

pub fn setup_appearance_handlers(
    main_box: &gtk4::Box,
    preview_pic: &gtk4::Picture,
    pick_btn: &gtk4::Button,
    theme_toggle_btn: &gtk4::Button,
    gtk_dropdown: &gtk4::DropDown,
    icon_dropdown: &gtk4::DropDown,
    cursor_dropdown: &gtk4::DropDown,
    size_dropdown: &gtk4::DropDown,
    target_dropdown: &gtk4::DropDown,
    quick_select_box: &gtk4::Box,
    gtk_themes: Vec<String>,
    icon_themes: Vec<String>,
    cursor_themes: Vec<String>,
    cursor_sizes: Vec<u32>,
) {
    let gtk_d = gtk_dropdown.clone();
    let icon_d = icon_dropdown.clone();
    let cursor_d = cursor_dropdown.clone();
    let size_d = size_dropdown.clone();

    let current_app = babydra_common::services::system::theme::get_current_appearance();

    if let Some(idx) = gtk_themes.iter().position(|t| t == &current_app.gtk_theme) {
        gtk_d.set_selected(idx as u32);
    }
    if let Some(idx) = icon_themes.iter().position(|t| t == &current_app.icon_theme) {
        icon_d.set_selected(idx as u32);
    }
    if let Some(idx) = cursor_themes.iter().position(|t| t == &current_app.cursor_theme) {
        cursor_d.set_selected(idx as u32);
    }
    if let Some(idx) = cursor_sizes.iter().position(|s| s == &current_app.cursor_size) {
        size_d.set_selected(idx as u32);
    }

    let initializing = Rc::new(Cell::new(true));

    let gtk_d_c = gtk_d.clone();
    let apply_theme_settings = move || {
        let gtk_idx = gtk_d_c.selected() as usize;
        let icon_idx = icon_d.selected() as usize;
        let cursor_idx = cursor_d.selected() as usize;
        let size_idx = size_d.selected() as usize;

        let selected_gtk = gtk_themes.get(gtk_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_icon = icon_themes.get(icon_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_cursor = cursor_themes.get(cursor_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_size = cursor_sizes.get(size_idx).cloned().unwrap_or(24);

        if let Some(root) = gtk_d_c.root() {
            let _ = root.activate_action("win.show-loading", Some(&true.to_variant()));

            let (tx, rx) = std::sync::mpsc::channel();

            std::thread::spawn(move || {
                let _ = babydra_common::services::system::theme::apply_appearance(
                    &selected_gtk,
                    &selected_icon,
                    &selected_cursor,
                    selected_size,
                );
                let notif_title = babydra_common::i18n::t("settings.notif_theme_title");
                let notif_msg = babydra_common::i18n::t("settings.notif_theme_msg")
                    .replace("{gtk}", &selected_gtk)
                    .replace("{icon}", &selected_icon);
                babydra_common::send_settings_notification(
                    &notif_title,
                    &notif_msg,
                );
                let _ = tx.send(());
            });

            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
                if rx.try_recv().is_ok() {
                    let _ = root.activate_action("win.show-loading", Some(&false.to_variant()));
                    gtk4::glib::ControlFlow::Break
                } else {
                    gtk4::glib::ControlFlow::Continue
                }
            });
        } else {
            let _ = babydra_common::services::system::theme::apply_appearance(
                &selected_gtk,
                &selected_icon,
                &selected_cursor,
                selected_size,
            );
        }
    };

    let apply_cb1 = apply_theme_settings.clone();
    let init_flag1 = initializing.clone();
    gtk_dropdown.connect_selected_notify(move |_| {
        if !init_flag1.get() { apply_cb1(); }
    });

    let apply_cb2 = apply_theme_settings.clone();
    let init_flag2 = initializing.clone();
    icon_dropdown.connect_selected_notify(move |_| {
        if !init_flag2.get() { apply_cb2(); }
    });

    let apply_cb3 = apply_theme_settings.clone();
    let init_flag3 = initializing.clone();
    cursor_dropdown.connect_selected_notify(move |_| {
        if !init_flag3.get() { apply_cb3(); }
    });

    let apply_cb4 = apply_theme_settings.clone();
    let init_flag4 = initializing.clone();
    size_dropdown.connect_selected_notify(move |_| {
        if !init_flag4.get() { apply_cb4(); }
    });

    initializing.set(false);

    let theme_btn_clone = theme_toggle_btn.clone();
    theme_toggle_btn.connect_clicked(move |_| {
        let currently_dark = babydra_utils::ui::theme::is_dark_mode();
        let new_dark = !currently_dark;
        
        let spinner = gtk4::Spinner::builder().spinning(true).halign(gtk4::Align::Center).valign(gtk4::Align::Center).build();
        theme_btn_clone.set_child(Some(&spinner));
        
        babydra_utils::ui::theme::set_dark_mode(new_dark);
    });

    if let Some(settings) = gtk4::Settings::default() {
        let theme_btn_clone_notify = theme_toggle_btn.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            let new_dark = babydra_utils::ui::theme::is_dark_mode();
            
            let notif_title = babydra_common::i18n::t("settings.notif_display_mode_title");
            let notif_msg = if new_dark {
                babydra_common::i18n::t("settings.notif_dark_mode_enabled")
            } else {
                babydra_common::i18n::t("settings.notif_light_mode_enabled")
            };
            babydra_common::send_settings_notification(&notif_title, &notif_msg);

            let new_icon_name = if new_dark { "brightness" } else { "dark-mode" };
            let new_icon = babydra_utils::ui::icon::get_icon(new_icon_name, 18);
            new_icon.set_pixel_size(18);
            new_icon.set_valign(gtk4::Align::Center);
            new_icon.set_halign(gtk4::Align::Center);
            theme_btn_clone_notify.set_child(Some(&new_icon));
        });
    }

    // Dynamic Target Selection & Wallpaper State Management
    let desktop_wp_path = Rc::new(RefCell::new(babydra_common::get_current_wallpaper()));
    let greeter_wp_path = Rc::new(RefCell::new(babydra_common::get_greeter_wallpaper()));
    let target_mode = Rc::new(Cell::new(0u32)); // 0 = Desktop, 1 = Lock screen

    let preview_pic_target = preview_pic.clone();
    let desktop_wp_ref = desktop_wp_path.clone();
    let greeter_wp_ref = greeter_wp_path.clone();
    let target_mode_ref = target_mode.clone();

    target_dropdown.connect_selected_notify(move |dd| {
        let sel = dd.selected();
        target_mode_ref.set(sel);
        if sel == 0 {
            if let Some(ref p) = *desktop_wp_ref.borrow() {
                preview_pic_target.set_filename(Some(p));
            } else {
                preview_pic_target.set_filename(None::<&str>);
            }
        } else {
            if let Some(ref p) = *greeter_wp_ref.borrow() {
                preview_pic_target.set_filename(Some(p));
            } else {
                preview_pic_target.set_filename(None::<&str>);
            }
        }
    });

    let render_wallpapers_grid = {
        let quick_select_box_clone = quick_select_box.clone();
        let preview_pic_clone = preview_pic.clone();
        let desktop_wp_path_clone = desktop_wp_path.clone();
        let greeter_wp_path_clone = greeter_wp_path.clone();
        let target_mode_clone = target_mode.clone();

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
                    let desktop_wp_cb = desktop_wp_path_clone.clone();
                    let greeter_wp_cb = greeter_wp_path_clone.clone();
                    let target_mode_cb = target_mode_clone.clone();

                    btn.connect_clicked(move |_| {
                        let is_lock = target_mode_cb.get() == 1;
                        if is_lock {
                            let _ = babydra_common::set_greeter_wallpaper(&wp_clone);
                            *greeter_wp_cb.borrow_mut() = Some(wp_clone.clone());
                            preview_cb.set_filename(Some(&wp_clone));
                            babydra_common::send_settings_notification(
                                &babydra_common::i18n::t("settings.notif_greeter_wallpaper_title"),
                                &babydra_common::i18n::t("settings.notif_greeter_wallpaper_msg"),
                            );
                        } else {
                            let _ = babydra_common::set_wallpaper(&wp_clone);
                            *desktop_wp_cb.borrow_mut() = Some(wp_clone.clone());
                            preview_cb.set_filename(Some(&wp_clone));
                            if let Some(root) = preview_cb.root() {
                                let _ = root.activate_action("win.refresh-sidebar", None);
                            }
                            babydra_common::send_settings_notification(
                                &babydra_common::i18n::t("settings.notif_wallpaper_title"),
                                &babydra_common::i18n::t("settings.notif_wallpaper_msg"),
                            );
                        }
                    });

                    flow.insert(&btn, -1);
                }

                quick_select_box_clone.append(&flow);
            }
        }
    };

    render_wallpapers_grid();

    let preview_clone = preview_pic.clone();
    let parent_box = main_box.clone();
    let render_grid_cb = render_wallpapers_grid;
    let desktop_wp_pick = desktop_wp_path.clone();
    let greeter_wp_pick = greeter_wp_path.clone();
    let target_mode_pick = target_mode.clone();

    // Floating '+' Button Picker
    pick_btn.connect_clicked(move |_| {
        if let Some(win) = parent_box.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
            let file_dialog = gtk4::FileDialog::new();
            let is_lock = target_mode_pick.get() == 1;
            let title = if is_lock {
                babydra_common::i18n::t("settings.pick_greeter_wallpaper")
            } else {
                babydra_common::i18n::t("settings.pick_wallpaper")
            };
            file_dialog.set_title(&title);

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some(&babydra_common::i18n::t("settings.image_filter")));
            filter.add_mime_type("image/png");
            filter.add_mime_type("image/jpeg");
            filter.add_mime_type("image/webp");
            file_dialog.set_default_filter(Some(&filter));

            let preview_cb = preview_clone.clone();
            let render_grid_after_pick = render_grid_cb.clone();
            let desktop_wp_file = desktop_wp_pick.clone();
            let greeter_wp_file = greeter_wp_pick.clone();

            file_dialog.open(Some(&win), None::<&gtk4::gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let target_dir = get_wallpaper_dir();
                        if let Some(filename) = path.file_name() {
                            let dest_path = target_dir.join(filename);
                            if path != dest_path {
                                let _ = std::fs::copy(&path, &dest_path);
                            }
                            if is_lock {
                                let _ = babydra_common::set_greeter_wallpaper(&dest_path);
                                *greeter_wp_file.borrow_mut() = Some(dest_path.clone());
                                preview_cb.set_filename(Some(&dest_path));
                                render_grid_after_pick();
                                babydra_common::send_settings_notification(
                                    &babydra_common::i18n::t("settings.notif_greeter_wallpaper_title"),
                                    &babydra_common::i18n::t("settings.notif_greeter_wallpaper_msg"),
                                );
                            } else {
                                let _ = babydra_common::set_wallpaper(&dest_path);
                                *desktop_wp_file.borrow_mut() = Some(dest_path.clone());
                                preview_cb.set_filename(Some(&dest_path));
                                render_grid_after_pick();
                                if let Some(root) = preview_cb.root() {
                                    let _ = root.activate_action("win.refresh-sidebar", None);
                                }
                                babydra_common::send_settings_notification(
                                    &babydra_common::i18n::t("settings.notif_wallpaper_title"),
                                    &babydra_common::i18n::t("settings.notif_wallpaper_msg"),
                                );
                            }
                        }
                    }
                }
            });
        }
    });
}
