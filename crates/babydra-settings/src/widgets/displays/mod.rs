pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;
use std::rc::Rc;
use std::cell::RefCell;

pub fn create_displays_widget() -> Widget {
    let monitors = babydra_common::services::system::display::get_displays();
    let widget = render::build(&monitors);
    let monitors_rc = Rc::new(RefCell::new(monitors));

    let card_rows_rc = Rc::new(widget.card_rows);
    let monitors_c = monitors_rc.clone();

    widget.save_btn.connect_clicked(move |_| {
        let mut current_monitors = monitors_c.borrow().clone();
        for (i, row) in card_rows_rc.iter().enumerate() {
            if let Some(mon) = current_monitors.get_mut(i) {
                mon.enabled = row.enable_switch.is_active();

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
    });

    widget.refresh_btn.connect_clicked(move |b| {
        let _ = b.activate_action("win.rebuild-ui", None);
    });

    widget.container.into()
}
