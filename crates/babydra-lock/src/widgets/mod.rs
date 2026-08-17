//! UI widgets and layout rendering logic for the lock screen windows.

use babydra_core::verify_password;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

/// Builds a wallpaper Picture widget from a custom path or saved greeter background.
pub fn create_wallpaper_picture(custom_path: Option<&str>) -> gtk4::Picture {
    let bg_picture = gtk4::Picture::new();
    bg_picture.set_can_shrink(true);
    bg_picture.set_content_fit(gtk4::ContentFit::Cover);
    bg_picture.set_hexpand(true);
    bg_picture.set_vexpand(true);

    if let Some(path) = custom_path {
        if let Ok(bytes) = std::fs::read(path) {
            let stream = gtk4::gio::MemoryInputStream::from_bytes(&gtk4::glib::Bytes::from(&bytes));
            if let Ok(pixbuf) =
                gtk4::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk4::gio::Cancellable::NONE)
            {
                bg_picture.set_pixbuf(Some(&pixbuf));
                return bg_picture;
            }
        }
    }

    if let Some(bytes) = babydra_core::get_greeter_wallpaper_bytes() {
        let stream = gtk4::gio::MemoryInputStream::from_bytes(&gtk4::glib::Bytes::from(&bytes));
        if let Ok(pixbuf) =
            gtk4::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk4::gio::Cancellable::NONE)
        {
            bg_picture.set_pixbuf(Some(&pixbuf));
            return bg_picture;
        }
    }

    bg_picture
}

/// Spawns a lock window assigned to a specific monitor.
pub fn create_lock_window(
    app: &gtk4::Application,
    monitor: Option<&gtk4::gdk::Monitor>,
    is_primary: bool,
    custom_wallpaper: Option<&str>,
) {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_exclusive_zone(-1);
    // Ensure the window is fully opaque — prevents Wayland compositor from rendering it transparent
    window.set_opacity(1.0);

    if let Some(m) = monitor {
        window.set_monitor(m);
    }

    if is_primary {
        window.set_keyboard_mode(KeyboardMode::Exclusive);
    } else {
        window.set_keyboard_mode(KeyboardMode::None);
    }

    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.add_css_class("lock-window");

    window.connect_close_request(|_| glib::Propagation::Stop);

    let overlay = gtk4::Overlay::new();

    let bg_picture = create_wallpaper_picture(custom_wallpaper);
    overlay.set_child(Some(&bg_picture));

    let tint_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    tint_box.add_css_class("lock-tint");
    tint_box.set_hexpand(true);
    tint_box.set_vexpand(true);
    overlay.add_overlay(&tint_box);

    let center_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    center_box.set_valign(gtk4::Align::Center);
    center_box.set_halign(gtk4::Align::Center);
    center_box.set_hexpand(true);
    center_box.set_vexpand(true);

    if is_primary {
        let card_box = babydra_ui_kit::components::create_card_with_class(
            gtk4::Orientation::Vertical,
            10,
            "lock-card",
        );
        card_box.set_valign(gtk4::Align::Center);
        card_box.set_halign(gtk4::Align::Center);

        let clock_label = gtk4::Label::new(None);
        clock_label.add_css_class("lock-clock");

        let date_label = gtk4::Label::new(None);
        date_label.add_css_class("lock-date");

        let update_clock = {
            let clock_label = clock_label.clone();
            let date_label = date_label.clone();
            move || {
                let (time, date) = babydra_core::format_clock_date("lock.date_format");
                clock_label.set_text(&time);
                date_label.set_text(&date);
                glib::ControlFlow::Continue
            }
        };
        update_clock();
        glib::timeout_add_local(std::time::Duration::from_secs(1), update_clock);

        let avatar_widget: gtk4::Widget = if let Some(bytes) = babydra_core::get_avatar_bytes() {
            if let Some(pixbuf) = babydra_core::crop_to_circle_pixbuf(&bytes, 110) {
                let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
                let img = gtk4::Image::from_paintable(Some(&texture));
                img.set_pixel_size(110);
                img.add_css_class("lock-avatar");
                img.set_halign(gtk4::Align::Center);
                img.set_valign(gtk4::Align::Center);
                img.upcast()
            } else {
                let icon =
                    babydra_ui_kit::ui::icon::get_system_or_file_icon("user-info", "user-info");
                icon.set_pixel_size(110);
                icon.add_css_class("lock-avatar-fallback");
                icon.set_halign(gtk4::Align::Center);
                icon.set_valign(gtk4::Align::Center);
                icon.upcast()
            }
        } else {
            let avatar_icon = babydra_ui_kit::ui::icon::get_icon("avatar-default", 110);
            avatar_icon.add_css_class("lock-avatar");
            avatar_icon.set_halign(gtk4::Align::Center);
            avatar_icon.set_valign(gtk4::Align::Center);
            avatar_icon.upcast()
        };

        let username = std::env::var("USER").unwrap_or_else(|_| "i4104".to_string());
        let user_label = gtk4::Label::new(Some(&username));
        user_label.add_css_class("lock-username");

        let entry = gtk4::Entry::new();
        entry.set_property("im-module", "none");
        entry.set_visibility(false);
        entry.set_placeholder_text(Some(&babydra_core::i18n::t("lock.placeholder")));
        entry.add_css_class("lock-input");
        entry.set_halign(gtk4::Align::Center);
        entry.set_max_length(100);

        let status_label = gtk4::Label::new(Some(&babydra_core::i18n::t("lock.status")));
        status_label.add_css_class("lock-status");

        card_box.append(&clock_label);
        card_box.append(&date_label);
        card_box.append(&avatar_widget);
        card_box.append(&user_label);
        card_box.append(&entry);
        card_box.append(&status_label);

        center_box.append(&card_box);

        let entry_clone = entry.clone();
        let status_label_clone = status_label.clone();
        let card_clone = card_box.clone();
        let username_clone = username.clone();

        entry.connect_activate(move |_| {
            let password = entry_clone.text().to_string();
            entry_clone.set_text("");

            if verify_password(&username_clone, &password) {
                std::process::exit(0);
            } else {
                status_label_clone.set_text(&babydra_core::i18n::t("lock.status_incorrect"));
                status_label_clone.add_css_class("error");
                card_clone.add_css_class("shake-error");

                let status_lbl = status_label_clone.clone();
                let card_box_ref = card_clone.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(1600), move || {
                    status_lbl.set_text(&babydra_core::i18n::t("lock.status"));
                    status_lbl.remove_css_class("error");
                    card_box_ref.remove_css_class("shake-error");
                });
            }
        });

        let entry_focus = entry.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(100), move || {
            entry_focus.grab_focus();
        });

        let entry_click = entry.clone();
        let click_gesture = gtk4::GestureClick::new();
        click_gesture.connect_pressed(move |_, _, _, _| {
            entry_click.grab_focus();
        });
        window.add_controller(click_gesture);
    } else {
        let clock_label = gtk4::Label::new(None);
        clock_label.add_css_class("lock-clock");

        let date_label = gtk4::Label::new(None);
        date_label.add_css_class("lock-date");

        let update_clock = {
            let clock_label = clock_label.clone();
            let date_label = date_label.clone();
            move || {
                let (time, date) = babydra_core::format_clock_date("lock.date_format");
                clock_label.set_text(&time);
                date_label.set_text(&date);
                glib::ControlFlow::Continue
            }
        };
        update_clock();
        glib::timeout_add_local(std::time::Duration::from_secs(1), update_clock);

        center_box.append(&clock_label);
        center_box.append(&date_label);
    }

    overlay.add_overlay(&center_box);
    window.set_child(Some(&overlay));
    window.present();
}
