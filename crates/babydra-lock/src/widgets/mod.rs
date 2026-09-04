//! Lock screen window assembly and password verification logic.

mod render;

use babydra_core::verify_password;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::widgets::render::PrimaryCard;

/// Spawns a lock window assigned to a specific monitor.
pub fn create_lock_window(
    app: &gtk4::Application,
    monitor: Option<&gtk4::gdk::Monitor>,
    is_primary: bool,
    custom_wallpaper: Option<&str>,
) {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);

    let kbd_mode = if is_primary {
        KeyboardMode::Exclusive
    } else {
        KeyboardMode::None
    };

    babydra_ui_kit::ui::window::init_layer_window(
        &window,
        Layer::Overlay,
        kbd_mode,
        -1,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        0,
        None,
    );
    window.set_opacity(1.0);

    if let Some(m) = monitor {
        window.set_monitor(m);
    }

    window.add_css_class("lock-window");
    window.connect_close_request(|_| glib::Propagation::Stop);

    let overlay = gtk4::Overlay::new();

    let bg_picture = render::build_wallpaper_img(custom_wallpaper);
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
        let PrimaryCard { card_box, entry, status_label, clock_label, date_label } = render::build_primary_card();
        setup_clock_timer(&clock_label, &date_label);
        setup_auth_handler(app, &entry, &status_label, &card_box);
        setup_focus_management(&window, &entry);
        center_box.append(&card_box);
    } else {
        let (clock_label, date_label) = render::build_clock_labels();
        setup_clock_timer(&clock_label, &date_label);
        center_box.append(&clock_label);
        center_box.append(&date_label);
    }

    overlay.add_overlay(&center_box);
    window.set_child(Some(&overlay));
    window.present();
}

/// Sets up a 1-second interval timer to update clock and date labels.
fn setup_clock_timer(clock_label: &gtk4::Label, date_label: &gtk4::Label) {
    let (time, date) = babydra_core::format_clock_date("lock.date_format");
    clock_label.set_text(&time);
    date_label.set_text(&date);

    let cl = clock_label.clone();
    let dl = date_label.clone();
    glib::timeout_add_seconds_local(1, move || {
        let (time, date) = babydra_core::format_clock_date("lock.date_format");
        cl.set_text(&time);
        dl.set_text(&date);
        glib::ControlFlow::Continue
    });
}

/// Wires up password verification and shake animation on enter press.
fn setup_auth_handler(
    app: &gtk4::Application,
    entry: &gtk4::PasswordEntry,
    status_label: &gtk4::Label,
    card_box: &gtk4::Box,
) {
    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let entry_clone = entry.clone();
    let status_label_clone = status_label.clone();
    let card_clone = card_box.clone();
    let app_clone = app.clone();

    entry.connect_activate(move |_| {
        let password = entry_clone.text().to_string();
        entry_clone.set_text("");

        if verify_password(&username, &password) {
            babydra_core::save_last_user(&username);
            app_clone.quit();
        } else {
            status_label_clone.set_text(&babydra_core::i18n::trans("lock.status_incorrect"));
            status_label_clone.add_css_class("error");
            card_clone.add_css_class("shake-error");

            let status_lbl = status_label_clone.clone();
            let card_box_ref = card_clone.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(1600), move || {
                status_lbl.set_text(&babydra_core::i18n::trans("lock.status"));
                status_lbl.remove_css_class("error");
                card_box_ref.remove_css_class("shake-error");
            });
        }
    });
}

/// Ensures the password entry grabs keyboard focus on spawn and on window click.
fn setup_focus_management(window: &gtk4::ApplicationWindow, entry: &gtk4::PasswordEntry) {
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
}
