//! Positioning logic for the context menu window layout.

use gtk4::prelude::*;

/// Places the context menu widget box at the cursor coordinates (x, y).
/// Adjusts the placement to prevent the menu from clipping outside screen bounds,
/// and applies a genie slide-in entrance animation.
pub fn position_and_animate_menu(
    x: f64,
    y: f64,
    window: &gtk4::ApplicationWindow,
    fixed_layout: &gtk4::Fixed,
    menu_box: &gtk4::Box,
) {
    let mut win_width = window.width() as f64;
    let mut win_height = window.height() as f64;

    if win_width <= 1.0 || win_height <= 1.0 {
        if let Some(display) = gtk4::gdk::Display::default() {
            let monitors = display.monitors();
            let mut found_monitor = None;
            let x_i = x as i32;
            let y_i = y as i32;
            for i in 0..monitors.n_items() {
                if let Some(item) = monitors.item(i) {
                    if let Ok(monitor) = item.downcast::<gtk4::gdk::Monitor>() {
                        let geometry = monitor.geometry();
                        if x_i >= geometry.x()
                            && x_i < geometry.x() + geometry.width()
                            && y_i >= geometry.y()
                            && y_i < geometry.y() + geometry.height()
                        {
                            found_monitor = Some(monitor);
                            break;
                        }
                    }
                }
            }
            let monitor = found_monitor.or_else(|| {
                monitors.item(0).and_then(|item| item.downcast::<gtk4::gdk::Monitor>().ok())
            });
            if let Some(monitor) = monitor {
                let geometry = monitor.geometry();
                win_width = geometry.width() as f64;
                win_height = geometry.height() as f64;
            }
        }
    }

    let menu_w = 220.0;
    let menu_h = 240.0;

    let mut pos_x = x;
    let mut pos_y = y;

    if pos_x + menu_w > win_width {
        pos_x = (win_width - menu_w - 10.0).max(0.0);
    }
    if pos_y + menu_h > win_height {
        pos_y = (win_height - menu_h - 10.0).max(0.0);
    }

    fixed_layout.put(menu_box, pos_x, pos_y);
    menu_box.set_opacity(1.0);

    babydra_common::animation::genie_in(
        menu_box.upcast_ref(),
        220,
        240,
        450,
    );
}

