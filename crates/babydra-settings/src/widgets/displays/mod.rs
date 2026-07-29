pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;
use std::cell::RefCell;
use std::rc::Rc;
use render::DisplayCardRow;
use babydra_common::models::display::MonitorConfig;

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

            // Refresh Rate
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

    let _ = babydra_common::services::system::display::save_displays(&current_monitors);
}

/// Creates the display settings widget with event bindings
pub fn create_displays_widget() -> Widget {
    let monitors = babydra_common::services::system::display::get_displays();
    let widget = render::build(&monitors);

    let monitors_rc = Rc::new(RefCell::new(monitors));
    let card_rows_rc = Rc::new(widget.card_rows);

    let monitors_c = monitors_rc.clone();
    let card_rows_c = card_rows_rc.clone();
    widget.save_btn.connect_clicked(move |_| {
        let monitors = monitors_c.borrow();
        save_display_configs(&monitors, &card_rows_c);
    });

    widget.refresh_btn.connect_clicked(move |b| {
        let _ = b.activate_action("win.rebuild-ui", None);
    });

    widget.container.into()
}
