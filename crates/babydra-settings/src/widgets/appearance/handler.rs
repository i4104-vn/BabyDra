use babydra_core::services::wallpaper::get_wallpaper_dir;

use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Sets up `appearance handlers`.
pub fn setup_appearance(
    main_box: &gtk4::Box,
    preview_pic: &gtk4::Picture,
    pick_btn: &gtk4::Button,
    theme_toggle_btn: &gtk4::Button,
    gtk_dropdown: &gtk4::DropDown,
    icon_dropdown: &gtk4::DropDown,
    cursor_dropdown: &gtk4::DropDown,
    size_dropdown: &gtk4::DropDown,
    target_dropdown: &gtk4::DropDown,
    mode_dropdown: &gtk4::DropDown,
    plugin_warning_box: &gtk4::Box,
    quick_select_box: &gtk4::Box,
    avatar_pic: &gtk4::Picture,
    avatar_btn: &gtk4::Button,
    gtk_themes: Vec<String>,
    icon_themes: Vec<String>,
    cursor_themes: Vec<String>,
    cursor_sizes: Vec<u32>,
) {
    let gtk_d = gtk_dropdown.clone();
    let icon_d = icon_dropdown.clone();
    let cursor_d = cursor_dropdown.clone();
    let size_d = size_dropdown.clone();

    let current_app = babydra_core::services::system::theme::get_appearance();

    if let Some(idx) = gtk_themes.iter().position(|t| t == &current_app.gtk_theme) {
        gtk_d.set_selected(idx as u32);
    }
    if let Some(idx) = icon_themes
        .iter()
        .position(|t| t == &current_app.icon_theme)
    {
        icon_d.set_selected(idx as u32);
    }
    if let Some(idx) = cursor_themes
        .iter()
        .position(|t| t == &current_app.cursor_theme)
    {
        cursor_d.set_selected(idx as u32);
    }
    if let Some(idx) = cursor_sizes
        .iter()
        .position(|s| s == &current_app.cursor_size)
    {
        size_d.set_selected(idx as u32);
    }

    let initializing = Rc::new(Cell::new(true));

    let gtk_d_c = gtk_d.clone();
    let apply_theme_settings = move || {
        let gtk_idx = gtk_d_c.selected() as usize;
        let icon_idx = icon_d.selected() as usize;
        let cursor_idx = cursor_d.selected() as usize;
        let size_idx = size_d.selected() as usize;

        let selected_gtk = gtk_themes
            .get(gtk_idx)
            .cloned()
            .unwrap_or_else(|| "Adwaita".to_string());
        let selected_icon = icon_themes
            .get(icon_idx)
            .cloned()
            .unwrap_or_else(|| "Adwaita".to_string());
        let selected_cursor = cursor_themes
            .get(cursor_idx)
            .cloned()
            .unwrap_or_else(|| "Adwaita".to_string());
        let selected_size = cursor_sizes.get(size_idx).cloned().unwrap_or(24);

        if let Some(root) = gtk_d_c.root() {
            let _ = root.activate_action("win.show-loading", Some(&true.to_variant()));

            let (tx, rx) = std::sync::mpsc::channel();

            std::thread::spawn(move || {
                let _ = babydra_core::services::system::theme::apply_appearance(
                    &selected_gtk,
                    &selected_icon,
                    &selected_cursor,
                    selected_size,
                );
                let notif_title = babydra_core::i18n::trans("settings.notif_theme_title");
                let notif_msg = babydra_core::i18n::trans("settings.notif_theme_msg")
                    .replace("{gtk}", &selected_gtk)
                    .replace("{icon}", &selected_icon);
                babydra_core::send_settings_notif(&notif_title, &notif_msg);
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
            let _ = babydra_core::services::system::theme::apply_appearance(
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
        if !init_flag1.get() {
            apply_cb1();
        }
    });

    let apply_cb2 = apply_theme_settings.clone();
    let init_flag2 = initializing.clone();
    icon_dropdown.connect_selected_notify(move |_| {
        if !init_flag2.get() {
            apply_cb2();
        }
    });

    let apply_cb3 = apply_theme_settings.clone();
    let init_flag3 = initializing.clone();
    cursor_dropdown.connect_selected_notify(move |_| {
        if !init_flag3.get() {
            apply_cb3();
        }
    });

    let apply_cb4 = apply_theme_settings.clone();
    let init_flag4 = initializing.clone();
    size_dropdown.connect_selected_notify(move |_| {
        if !init_flag4.get() {
            apply_cb4();
        }
    });

    initializing.set(false);

    let theme_btn_clone = theme_toggle_btn.clone();
    theme_toggle_btn.connect_clicked(move |_| {
        let currently_dark = babydra_ui_kit::ui::theme::is_dark_mode();
        let new_dark = !currently_dark;

        let spinner = gtk4::Spinner::builder()
            .spinning(true)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .build();
        theme_btn_clone.set_child(Some(&spinner));

        babydra_ui_kit::ui::theme::set_dark_mode(new_dark);
    });

    if let Some(settings) = gtk4::Settings::default() {
        let theme_btn_clone_notify = theme_toggle_btn.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            let new_dark = babydra_ui_kit::ui::theme::is_dark_mode();

            let notif_title = babydra_core::i18n::trans("settings.notif_display_mode_title");
            let notif_msg = if new_dark {
                babydra_core::i18n::trans("settings.notif_dark_mode_enabled")
            } else {
                babydra_core::i18n::trans("settings.notif_light_mode_enabled")
            };
            babydra_core::send_settings_notif(&notif_title, &notif_msg);

            let new_icon_name = if new_dark { "brightness" } else { "dark-mode" };
            let new_icon = babydra_ui_kit::ui::icon::get_icon(new_icon_name, 18);
            new_icon.set_pixel_size(18);
            new_icon.set_valign(gtk4::Align::Center);
            new_icon.set_halign(gtk4::Align::Center);
            theme_btn_clone_notify.set_child(Some(&new_icon));
        });
    }

    // Dynamic Target Selection & Wallpaper State Management
    let desktop_wp_path = Rc::new(RefCell::new(babydra_core::get_wallpaper()));
    let greeter_wp_path = Rc::new(RefCell::new(babydra_core::get_greeter_wp()));
    let target_mode = Rc::new(Cell::new(0u32)); // 0 = Desktop, 1 = Lock screen
    let current_mode = Rc::new(RefCell::new(babydra_core::wallpaper::get_wallpaper_mode()));

    // Initialize mode dropdown
    let initial_is_live = *current_mode.borrow() == babydra_core::wallpaper::WallpaperMode::Live;
    mode_dropdown.set_selected(if initial_is_live { 1 } else { 0 });
    if initial_is_live && !babydra_core::wallpaper::is_gstreamer_plugin_available() {
        plugin_warning_box.set_visible(true);
        pick_btn.set_sensitive(false);
    }

    // Initialize preview with current wallpaper thumbnail
    if let Some(ref p) = *desktop_wp_path.borrow() {
        let p_clone = p.clone();
        let pic_clone = preview_pic.clone();
        crate::widgets::helpers::spawn_async_task(
            move || babydra_core::wallpaper::get_or_create_thumbnail(&p_clone),
            move |thumb| {
                if !thumb.as_os_str().is_empty() {
                    let file = gtk4::gio::File::for_path(&thumb);
                    if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                        pic_clone.set_paintable(Some(&texture));
                    } else {
                        pic_clone.set_filename(Some(&thumb));
                    }
                }
            },
            16,
        );
    }

    let preview_pic_target = preview_pic.clone();
    let desktop_wp_ref = desktop_wp_path.clone();
    let greeter_wp_ref = greeter_wp_path.clone();
    let target_mode_ref = target_mode.clone();

    target_dropdown.connect_selected_notify(move |dd| {
        let sel = dd.selected();
        target_mode_ref.set(sel);
        if sel == 0 {
            if let Some(ref p) = *desktop_wp_ref.borrow() {
                let p_clone = p.clone();
                let pic_clone = preview_pic_target.clone();
                crate::widgets::helpers::spawn_async_task(
                    move || babydra_core::wallpaper::get_or_create_thumbnail(&p_clone),
                    move |thumb| {
                        if !thumb.as_os_str().is_empty() {
                            let file = gtk4::gio::File::for_path(&thumb);
                            if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                                pic_clone.set_paintable(Some(&texture));
                            } else {
                                pic_clone.set_filename(Some(&thumb));
                            }
                        }
                    },
                    16,
                );
            } else {
                preview_pic_target.set_paintable(None::<&gtk4::gdk::Paintable>);
            }
        } else {
            if let Some(bytes) = babydra_core::get_greeter_wp_bytes() {
                let stream =
                    gtk4::gio::MemoryInputStream::from_bytes(&gtk4::glib::Bytes::from(&bytes));
                if let Ok(pixbuf) =
                    gtk4::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk4::gio::Cancellable::NONE)
                {
                    preview_pic_target.set_pixbuf(Some(&pixbuf));
                }
            } else if let Some(ref p) = *greeter_wp_ref.borrow() {
                if p.extension().and_then(|e| e.to_str()) != Some("bb") {
                    let file = gtk4::gio::File::for_path(p);
                    if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                        preview_pic_target.set_paintable(Some(&texture));
                    } else {
                        preview_pic_target.set_filename(Some(p));
                    }
                }
            } else {
                preview_pic_target.set_paintable(None::<&gtk4::gdk::Paintable>);
            }
        }
    });


    let render_wallpapers_grid = {
        let quick_select_box_clone = quick_select_box.clone();
        let preview_pic_clone = preview_pic.clone();
        let desktop_wp_path_clone = desktop_wp_path.clone();
        let greeter_wp_path_clone = greeter_wp_path.clone();
        let target_mode_clone = target_mode.clone();
        let current_mode_clone = current_mode.clone();

        Rc::new(move || {
            while let Some(child) = quick_select_box_clone.first_child() {
                quick_select_box_clone.remove(&child);
            }

            let is_live_tab = *current_mode_clone.borrow() == babydra_core::wallpaper::WallpaperMode::Live;
            let wallpapers = if is_live_tab {
                babydra_core::wallpaper::get_live_wallpapers()
            } else {
                babydra_core::wallpaper::get_static_wallpapers()
            };

            if wallpapers.is_empty() {
                let empty_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
                empty_box.set_halign(gtk4::Align::Center);
                empty_box.set_valign(gtk4::Align::Center);
                empty_box.set_margin_top(24);
                empty_box.set_margin_bottom(24);

                let icon = babydra_ui_kit::ui::icon::get_icon("display", 28);
                icon.set_pixel_size(28);
                icon.set_halign(gtk4::Align::Center);
                empty_box.append(&icon);

                let title = if is_live_tab {
                    gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.no_live_wallpapers")))
                } else {
                    gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.no_wallpapers")))
                };
                title.add_css_class("settings-row-title");
                empty_box.append(&title);

                let sub = if is_live_tab {
                    gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.no_live_wallpapers_sub")))
                } else {
                    gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.no_wallpapers_sub")))
                };
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

                    let pic = gtk4::Picture::new();
                    pic.set_size_request(130, 105);
                    pic.set_content_fit(gtk4::ContentFit::Cover);
                    
                    let spinner = gtk4::Spinner::new();
                    spinner.set_halign(gtk4::Align::Center);
                    spinner.set_valign(gtk4::Align::Center);
                    spinner.set_size_request(24, 24);
                    spinner.start();
                    
                    let pic_overlay = gtk4::Overlay::new();
                    pic_overlay.set_child(Some(&pic));
                    pic_overlay.add_overlay(&spinner);
                    
                    let wp_path_clone = wp.clone();
                    let pic_clone = pic.clone();
                    
                    crate::widgets::helpers::spawn_async_task(
                        move || babydra_core::wallpaper::get_or_create_thumbnail(&wp_path_clone),
                        move |thumb_path| {
                            if !thumb_path.as_os_str().is_empty() {
                                let file = gtk4::gio::File::for_path(&thumb_path);
                                if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                                    pic_clone.set_paintable(Some(&texture));
                                } else {
                                    pic_clone.set_filename(Some(&thumb_path));
                                }
                            }
                            spinner.stop();
                            spinner.set_visible(false);
                        },
                        16,
                    );

                    let is_live_file = babydra_core::wallpaper::is_live_wallpaper_file(&wp);
                    let card_child = if is_live_file {
                        let badge_text = if babydra_core::wallpaper::is_video_file(&wp) {
                            babydra_core::i18n::trans("settings.badge_video")
                        } else {
                            babydra_core::i18n::trans("settings.badge_gif")
                        };
                        let badge = gtk4::Label::new(Some(&badge_text));
                        badge.add_css_class("wallpaper-badge");
                        badge.add_css_class("wallpaper-badge-live");
                        badge.set_halign(gtk4::Align::End);
                        badge.set_valign(gtk4::Align::Start);
                        pic_overlay.add_overlay(&badge);
                        pic_overlay.upcast::<gtk4::Widget>()
                    } else {
                        pic_overlay.upcast::<gtk4::Widget>()
                    };

                    btn.set_child(Some(&card_child));

                    let wp_clone = wp.clone();
                    let preview_cb = preview_pic_clone.clone();
                    let desktop_wp_cb = desktop_wp_path_clone.clone();
                    let greeter_wp_cb = greeter_wp_path_clone.clone();
                    let target_mode_cb = target_mode_clone.clone();

                    btn.connect_clicked(move |_| {
                        let thumb_clone = babydra_core::wallpaper::get_or_create_thumbnail(&wp_clone);
                        let is_lock = target_mode_cb.get() == 1;
                        if is_lock {
                            let _ = babydra_core::set_greeter_wp(&thumb_clone);
                            *greeter_wp_cb.borrow_mut() = Some(thumb_clone.clone());
                            let file = gtk4::gio::File::for_path(&thumb_clone);
                            if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                                preview_cb.set_paintable(Some(&texture));
                            } else {
                                preview_cb.set_filename(Some(&thumb_clone));
                            }
                            babydra_core::send_settings_notif(
                                &babydra_core::i18n::trans(
                                    "settings.notif_greeter_wallpaper_title",
                                ),
                                &babydra_core::i18n::trans("settings.notif_greeter_wallpaper_msg"),
                            );
                        } else {
                            let is_video = babydra_core::wallpaper::is_video_file(&wp_clone);
                            if is_video && !babydra_core::wallpaper::is_gstreamer_plugin_available() {
                                babydra_core::send_settings_notif(
                                    &babydra_core::i18n::trans("settings.missing_gst_plugin_title"),
                                    &babydra_core::i18n::trans("settings.missing_gst_plugin_desc"),
                                );
                                return;
                            }
                            let mode_to_set = if babydra_core::wallpaper::is_live_wallpaper_file(&wp_clone) {
                                babydra_core::wallpaper::WallpaperMode::Live
                            } else {
                                babydra_core::wallpaper::WallpaperMode::Static
                            };
                            let _ = babydra_core::wallpaper::set_wallpaper_with_mode(&wp_clone, mode_to_set);
                            *desktop_wp_cb.borrow_mut() = Some(wp_clone.clone());
                            let file = gtk4::gio::File::for_path(&thumb_clone);
                            if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                                preview_cb.set_paintable(Some(&texture));
                            } else {
                                preview_cb.set_filename(Some(&thumb_clone));
                            }
                            if let Some(root) = preview_cb.root() {
                                let _ = root.activate_action("win.refresh-sidebar", None);
                            }
                            let notif_title = if mode_to_set == babydra_core::wallpaper::WallpaperMode::Live {
                                babydra_core::i18n::trans("settings.notif_live_wallpaper_title")
                            } else {
                                babydra_core::i18n::trans("settings.notif_wallpaper_title")
                            };
                            let notif_msg = if mode_to_set == babydra_core::wallpaper::WallpaperMode::Live {
                                babydra_core::i18n::trans("settings.notif_live_wallpaper_msg")
                            } else {
                                babydra_core::i18n::trans("settings.notif_wallpaper_msg")
                            };
                            babydra_core::send_settings_notif(&notif_title, &notif_msg);
                        }
                    });


                    flow.insert(&btn, -1);
                }

                quick_select_box_clone.append(&flow);
            }
        })
    };

    // Mode dropdown change listener
    let current_mode_dd = current_mode.clone();
    let warning_box_dd = plugin_warning_box.clone();
    let pick_btn_dd = pick_btn.clone();
    let render_grid_dd = render_wallpapers_grid.clone();

    mode_dropdown.connect_selected_notify(move |dd| {
        let sel = dd.selected();
        if sel == 1 {
            *current_mode_dd.borrow_mut() = babydra_core::wallpaper::WallpaperMode::Live;
            let ok = babydra_core::wallpaper::is_gstreamer_plugin_available();
            if !ok {
                warning_box_dd.set_visible(true);
                pick_btn_dd.set_sensitive(false);
            } else {
                warning_box_dd.set_visible(false);
                pick_btn_dd.set_sensitive(true);
            }
        } else {
            *current_mode_dd.borrow_mut() = babydra_core::wallpaper::WallpaperMode::Static;
            warning_box_dd.set_visible(false);
            pick_btn_dd.set_sensitive(true);
        }
        render_grid_dd();
    });

    render_wallpapers_grid();

    let preview_clone = preview_pic.clone();
    let parent_box = main_box.clone();
    let render_grid_cb = render_wallpapers_grid.clone();
    let desktop_wp_pick = desktop_wp_path.clone();
    let greeter_wp_pick = greeter_wp_path.clone();
    let target_mode_pick = target_mode.clone();
    let current_mode_pick = current_mode.clone();

    // Floating '+' Button Picker
    pick_btn.connect_clicked(move |_| {
        let is_live_mode = *current_mode_pick.borrow() == babydra_core::wallpaper::WallpaperMode::Live;
        if is_live_mode && !babydra_core::wallpaper::is_gstreamer_plugin_available() {
            babydra_core::send_settings_notif(
                &babydra_core::i18n::trans("settings.missing_gst_plugin_title"),
                &babydra_core::i18n::trans("settings.missing_gst_plugin_desc"),
            );
            return;
        }

        if let Some(win) = parent_box
            .root()
            .and_then(|r| r.downcast::<gtk4::Window>().ok())
        {
            let file_dialog = gtk4::FileDialog::new();
            let is_lock = target_mode_pick.get() == 1;
            let title = if is_lock {
                babydra_core::i18n::trans("settings.pick_greeter_wallpaper")
            } else {
                babydra_core::i18n::trans("settings.pick_wallpaper")
            };
            file_dialog.set_title(&title);

            let filter = gtk4::FileFilter::new();
            if is_live_mode {
                filter.set_name(Some(&babydra_core::i18n::trans("settings.live_filter")));
                filter.add_mime_type("image/gif");
                filter.add_mime_type("video/mp4");
                filter.add_mime_type("video/webm");
                filter.add_mime_type("video/x-matroska");
                filter.add_pattern("*.gif");
                filter.add_pattern("*.mp4");
                filter.add_pattern("*.webm");
                filter.add_pattern("*.mkv");
            } else {
                filter.set_name(Some(&babydra_core::i18n::trans("settings.image_filter")));
                filter.add_mime_type("image/png");
                filter.add_mime_type("image/jpeg");
                filter.add_mime_type("image/webp");
                filter.add_pattern("*.png");
                filter.add_pattern("*.jpg");
                filter.add_pattern("*.jpeg");
                filter.add_pattern("*.webp");
            }
            file_dialog.set_default_filter(Some(&filter));

            let preview_cb = preview_clone.clone();
            let render_grid_after_pick = render_grid_cb.clone();
            let desktop_wp_file = desktop_wp_pick.clone();
            let greeter_wp_file = greeter_wp_pick.clone();

            file_dialog.open(Some(&win), None::<&gtk4::gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        // Enforce video duration limit (< 20 seconds)
                        if babydra_core::wallpaper::is_video_file(&path) {
                            if let Some(dur) = babydra_core::wallpaper::get_video_duration(&path) {
                                if dur > 20.05 {
                                    let title = babydra_core::i18n::trans("settings.video_too_long_title");
                                    let msg = babydra_core::i18n::trans("settings.video_too_long_msg")
                                        .replace("{duration}", &format!("{:.1}", dur));
                                    babydra_core::send_settings_notif(&title, &msg);
                                    return; // REJECT
                                }
                            }
                        }

                        let target_dir = get_wallpaper_dir();
                        if let Some(filename) = path.file_name() {
                            let dest_path = target_dir.join(filename);
                            if path != dest_path {
                                let _ = std::fs::copy(&path, &dest_path);
                            }
                            let thumb_path = babydra_core::wallpaper::get_or_create_thumbnail(&dest_path);

                            if is_lock {
                                let _ = babydra_core::set_greeter_wp(&thumb_path);
                                *greeter_wp_file.borrow_mut() = Some(thumb_path.clone());
                                let file = gtk4::gio::File::for_path(&thumb_path);
                                if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                                    preview_cb.set_paintable(Some(&texture));
                                } else {
                                    preview_cb.set_filename(Some(&thumb_path));
                                }
                                render_grid_after_pick();
                                babydra_core::send_settings_notif(
                                    &babydra_core::i18n::trans(
                                        "settings.notif_greeter_wallpaper_title",
                                    ),
                                    &babydra_core::i18n::trans(
                                        "settings.notif_greeter_wallpaper_msg",
                                    ),
                                );
                            } else {
                                let mode_val = if is_live_mode { babydra_core::wallpaper::WallpaperMode::Live } else { babydra_core::wallpaper::WallpaperMode::Static };
                                let _ = babydra_core::wallpaper::set_wallpaper_with_mode(&dest_path, mode_val);
                                *desktop_wp_file.borrow_mut() = Some(dest_path.clone());
                                let file = gtk4::gio::File::for_path(&thumb_path);
                                if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                                    preview_cb.set_paintable(Some(&texture));
                                } else {
                                    preview_cb.set_filename(Some(&thumb_path));
                                }
                                render_grid_after_pick();

                                if let Some(root) = preview_cb.root() {
                                    let _ = root.activate_action("win.refresh-sidebar", None);
                                }
                                let notif_title = if is_live_mode {
                                    babydra_core::i18n::trans("settings.notif_live_wallpaper_title")
                                } else {
                                    babydra_core::i18n::trans("settings.notif_wallpaper_title")
                                };
                                let notif_msg = if is_live_mode {
                                    babydra_core::i18n::trans("settings.notif_live_wallpaper_msg")
                                } else {
                                    babydra_core::i18n::trans("settings.notif_wallpaper_msg")
                                };
                                babydra_core::send_settings_notif(&notif_title, &notif_msg);

                            }
                        }
                    }
                }
            });
        }
    });


    let avatar_preview_cb = avatar_pic.clone();
    let parent_box_av = main_box.clone();
    avatar_btn.connect_clicked(move |_| {
        if let Some(win) = parent_box_av
            .root()
            .and_then(|r| r.downcast::<gtk4::Window>().ok())
        {
            let file_dialog = gtk4::FileDialog::new();
            file_dialog.set_title(&babydra_core::i18n::trans("settings.pick_avatar"));

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some(&babydra_core::i18n::trans("settings.image_filter")));
            filter.add_mime_type("image/png");
            filter.add_mime_type("image/jpeg");
            filter.add_mime_type("image/webp");
            file_dialog.set_default_filter(Some(&filter));

            let preview_cb = avatar_preview_cb.clone();

            file_dialog.open(Some(&win), None::<&gtk4::gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        if babydra_core::set_avatar(&path).is_ok() {
                            if let Some(bytes) = babydra_core::get_avatar_bytes() {
                                if let Some(pixbuf) =
                                    babydra_ui_kit::ui::image::crop_circle(&bytes, 42)
                                {
                                    preview_cb.set_pixbuf(Some(&pixbuf));
                                }
                            }
                            babydra_core::send_settings_notif(
                                &babydra_core::i18n::trans("settings.notif_avatar_title"),
                                &babydra_core::i18n::trans("settings.notif_avatar_msg"),
                            );
                        }
                    }
                }
            });
        }
    });
}
