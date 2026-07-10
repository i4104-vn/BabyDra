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
    babydra_island::widgets::notification::is_dnd_active()
}

pub fn create_dnd_tile() -> gtk4::Button {
    let active = is_dnd_active();
    let (btn, _) = baby_utils::components::create_toggle_tile(
        "bell-off",
        "DND",
        "",
        "control-dnd-tile",
        active,
        |new_active| {
            babydra_island::widgets::notification::set_dnd_active(new_active);
        }
    );
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
    let active = is_night_light_active();
    baby_utils::components::create_square_toggle_tile(
        "night-light",
        "",
        active,
        |new_active| {
            if new_active {
                let _ = std::process::Command::new("gammastep").args(&["-O", "4500", "-b", "1.0:1.0"]).spawn();
            } else {
                let _ = std::process::Command::new("pkill").arg("-x").arg("gammastep").status();
            }
        }
    )
}
