use gtk4::prelude::*;
use std::rc::Rc;
use super::items;

pub fn create_control_center_grid(on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Box {
    let main_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    main_layout.add_css_class("control-center-grid");
    main_layout.set_hexpand(true);
    main_layout.set_valign(gtk4::Align::Fill);
    main_layout.set_vexpand(true);

    let left_box = create_left_box_toggles(on_popover_toggled.clone());
    let right_grid = gtk4::Grid::new();
    right_grid.set_column_spacing(10);
    right_grid.set_row_spacing(10);
    right_grid.set_hexpand(true);
    right_grid.set_vexpand(true);
    right_grid.set_valign(gtk4::Align::Fill);

    let dnd_btn = create_dnd_tile();
    right_grid.attach(&dnd_btn, 0, 0, 2, 1);

    let small_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    small_box.set_hexpand(true);
    small_box.set_homogeneous(true);
    small_box.set_vexpand(true);
    small_box.set_valign(gtk4::Align::Fill);

    let night_btn = create_night_light_tile();
    let clean_btn = items::clean::render::create_clean_tile(on_popover_toggled);

    small_box.append(&clean_btn);
    small_box.append(&night_btn);
    right_grid.attach(&small_box, 0, 1, 2, 1);

    main_layout.append(&left_box);
    main_layout.append(&right_grid);
    main_layout
}

fn create_left_box_toggles(on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    container.add_css_class("control-left-toggles-box");
    container.set_valign(gtk4::Align::Fill);
    container.set_vexpand(true);

    let wifi_tile = items::wifi::render::create_wifi_tile(on_popover_toggled);
    let bt_btn = items::bluetooth::render::create_bluetooth_tile();

    container.append(&wifi_tile);
    container.append(&bt_btn);
    container
}

fn is_dnd_active() -> bool {
    // Check dunst
    if let Ok(output) = std::process::Command::new("dunstctl").arg("is-paused").output() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout == "true" {
            return true;
        }
    }
    // Check mako
    if let Ok(output) = std::process::Command::new("makoctl").arg("mode").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("dnd") {
            return true;
        }
    }
    false
}

pub fn create_dnd_tile() -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("control-dnd-tile");
    btn.set_hexpand(true);
    btn.set_valign(gtk4::Align::Fill);
    btn.set_vexpand(true);

    let main_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    main_box.set_valign(gtk4::Align::Center);
    main_box.set_halign(gtk4::Align::Center);

    let active = is_dnd_active();
    if active {
        btn.add_css_class("active");
    }

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_valign(gtk4::Align::Center);

    let icon_color = if active { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" };
    let icon_widget = babydra_common::icon::get_icon_colored("bell-off", 18, icon_color);
    icon_container.append(&icon_widget);

    let label = gtk4::Label::new(Some("DND"));
    label.add_css_class("control-dnd-label");
    label.set_valign(gtk4::Align::Center);

    main_box.append(&icon_container);
    main_box.append(&label);
    btn.set_child(Some(&main_box));

    btn.connect_clicked(move |b| {
        let current_active = b.has_css_class("active");
        let new_active = !current_active;

        if new_active {
            b.add_css_class("active");
            let _ = std::process::Command::new("dunstctl").arg("set-paused").arg("true").spawn();
            let _ = std::process::Command::new("makoctl").args(&["mode", "-a", "dnd"]).spawn();
        } else {
            b.remove_css_class("active");
            let _ = std::process::Command::new("dunstctl").arg("set-paused").arg("false").spawn();
            let _ = std::process::Command::new("makoctl").args(&["mode", "-r", "dnd"]).spawn();
        }

        if let Some(old) = icon_container.first_child() {
            icon_container.remove(&old);
        }
        let color = if new_active { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" };
        let new_img = babydra_common::icon::get_icon_colored("bell-off", 18, color);
        icon_container.append(&new_img);
    });

    btn
}

fn is_night_light_active() -> bool {
    if let Ok(output) = std::process::Command::new("pgrep").arg("-x").arg("gammastep").output() {
        if output.status.success() {
            return true;
        }
    }
    if let Ok(output) = std::process::Command::new("pgrep").arg("-x").arg("wl-gammarelay").output() {
        if output.status.success() {
            return true;
        }
    }
    false
}

pub fn create_night_light_tile() -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("control-square-tile");
    btn.set_size_request(56, 56);
    btn.set_halign(gtk4::Align::Center);
    btn.set_valign(gtk4::Align::Center);
    btn.set_hexpand(false);
    btn.set_vexpand(false);

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.set_valign(gtk4::Align::Center);
    main_box.set_halign(gtk4::Align::Center);

    let active = is_night_light_active();
    if active {
        btn.add_css_class("active");
    }

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_halign(gtk4::Align::Center);

    let icon_color = if active { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" };
    let icon_widget = babydra_common::icon::get_icon_colored("night-light", 18, icon_color);
    icon_container.append(&icon_widget);

    main_box.append(&icon_container);
    btn.set_child(Some(&main_box));

    btn.connect_clicked(move |b| {
        let current_active = b.has_css_class("active");
        let new_active = !current_active;

        if new_active {
            b.add_css_class("active");
            let _ = std::process::Command::new("gammastep").args(&["-O", "4500", "-b", "1.0:1.0"]).spawn();
        } else {
            b.remove_css_class("active");
            let _ = std::process::Command::new("pkill").arg("-x").arg("gammastep").status();
        }

        if let Some(old) = icon_container.first_child() {
            icon_container.remove(&old);
        }
        let color = if new_active { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" };
        let new_img = babydra_common::icon::get_icon_colored("night-light", 18, color);
        icon_container.append(&new_img);
    });

    btn
}
