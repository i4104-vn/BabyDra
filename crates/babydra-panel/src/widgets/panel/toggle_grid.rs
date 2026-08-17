use super::items;
use gtk4::prelude::*;
use std::rc::Rc;

/// Creates a new `control center grid`.
pub fn create_control_center_grid(
    on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>,
) -> gtk4::Box {
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

    let top_small_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    top_small_box.set_hexpand(true);
    top_small_box.set_homogeneous(true);
    top_small_box.set_vexpand(true);
    top_small_box.set_valign(gtk4::Align::Fill);

    let dnd_btn = create_dnd_tile();
    let vpn_btn = items::vpn::render::create_vpn_tile(on_popover_toggled.clone());
    top_small_box.append(&dnd_btn);
    top_small_box.append(&vpn_btn);
    right_grid.attach(&top_small_box, 0, 0, 2, 1);

    let bottom_small_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    bottom_small_box.set_hexpand(true);
    bottom_small_box.set_homogeneous(true);
    bottom_small_box.set_vexpand(true);
    bottom_small_box.set_valign(gtk4::Align::Fill);

    let night_btn = create_night_light_tile();
    let clean_btn = items::clean::render::create_clean_tile(on_popover_toggled);

    bottom_small_box.append(&clean_btn);
    bottom_small_box.append(&night_btn);
    right_grid.attach(&bottom_small_box, 0, 1, 2, 1);

    main_layout.append(&left_box);
    main_layout.append(&right_grid);
    main_layout
}

/// Creates a new `left box toggles`.
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

/// Returns `true` when `dnd active` holds, `false` otherwise.
fn is_dnd_active() -> bool {
    babydra_island::widgets::notification::is_dnd_active()
}

/// Creates a new `dnd tile`.
pub fn create_dnd_tile() -> gtk4::Button {
    let active = is_dnd_active();
    babydra_ui_kit::components::create_square_toggle_tile("bell-off", "", active, |new_active| {
        babydra_island::widgets::notification::set_dnd_active(new_active);
    })
}

/// Returns `true` when `night light active` holds, `false` otherwise.
fn is_night_light_active() -> bool {
    if let Ok(output) = std::process::Command::new("pgrep")
        .arg("-x")
        .arg("gammastep")
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    if let Ok(output) = std::process::Command::new("pgrep")
        .arg("-x")
        .arg("wl-gammarelay")
        .output()
    {
        if output.status.success() {
            return true;
        }
    }
    false
}

/// Creates a new `night light tile`.
pub fn create_night_light_tile() -> gtk4::Button {
    let active = is_night_light_active();
    babydra_ui_kit::components::create_square_toggle_tile("night-light", "", active, |new_active| {
        if new_active {
            let _ = std::process::Command::new("gammastep")
                .args(&["-O", "4500", "-b", "1.0:1.0"])
                .spawn();
        } else {
            let _ = std::process::Command::new("pkill")
                .arg("-x")
                .arg("gammastep")
                .status();
        }
    })
}
