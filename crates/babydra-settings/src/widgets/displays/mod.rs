pub mod render;

use crate::widgets::state::DisplayCardRow;
use babydra_core::models::display::MonitorConfig;
use gtk4::prelude::*;
use gtk4::Widget;
use std::cell::RefCell;
use std::rc::Rc;

/// Reads selected UI values from card rows and persists updated monitor configurations
fn save_display_configs(monitors: &[MonitorConfig], card_rows: &[DisplayCardRow]) {
    let mut current_monitors = monitors.to_vec();
    for (i, row) in card_rows.iter().enumerate() {
        if let Some(mon) = current_monitors.get_mut(i) {
            // Resolution
            let res_idx = row.resolution_dropdown.selected() as usize;
            if let Some(res_str) = mon.available_resolutions.get(res_idx) {
                let parts: Vec<&str> = res_str.split('x').collect();
                if parts.len() == 2 {
                    if let (Ok(w), Ok(h)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                        mon.resolution_width = w;
                        mon.resolution_height = h;
                    }
                }
            }

            let rate_idx = row.rate_dropdown.selected() as usize;
            if let Some(&rate) = mon.available_rates.get(rate_idx) {
                mon.refresh_rate = rate;
            }

            // Orientation
            let orient_idx = row.orientation_dropdown.selected();
            mon.orientation = match orient_idx {
                1 => "left".to_string(),
                2 => "inverted".to_string(),
                3 => "right".to_string(),
                _ => "normal".to_string(),
            };
        }
    }

    let _ = babydra_core::services::system::display::save_displays(&current_monitors);
}

/// Creates the display settings widget with event bindings
pub fn create_displays() -> Widget {
    // Initial 0ms layout build
    let widget = render::build(&[]);
    let container_box = widget.container.clone();
    let ret_box = widget.container.clone();

    let (tx, rx) = std::sync::mpsc::channel::<Vec<MonitorConfig>>();
    std::thread::spawn(move || {
        let monitors = babydra_core::services::system::display::get_displays();
        let _ = tx.send(monitors);
    });

    let refresh_btn_c = widget.refresh_btn.clone();
    refresh_btn_c.connect_clicked(move |b| {
        let _ = b.activate_action("win.rebuild-ui", None);
    });

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        if let Ok(monitors) = rx.try_recv() {
            let new_widget = render::build(&monitors);
            let monitors_rc = Rc::new(RefCell::new(monitors));
            let card_rows_rc = Rc::new(new_widget.card_rows);

            let monitors_c = monitors_rc.clone();
            let card_rows_c = card_rows_rc.clone();
            new_widget.save_btn.connect_clicked(move |_| {
                let monitors = monitors_c.borrow();
                save_display_configs(&monitors, &card_rows_c);
            });

            new_widget.refresh_btn.connect_clicked(move |b| {
                let _ = b.activate_action("win.rebuild-ui", None);
            });

            // Replace container children with newly built header and scrollable monitor cards
            while let Some(c) = container_box.first_child() {
                container_box.remove(&c);
            }
            while let Some(c) = new_widget.container.first_child() {
                new_widget.container.remove(&c);
                container_box.append(&c);
            }

            gtk4::glib::ControlFlow::Break
        } else {
            gtk4::glib::ControlFlow::Continue
        }
    });

    ret_box.into()
}
