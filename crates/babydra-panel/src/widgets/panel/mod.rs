pub mod items;
pub mod modal;
pub mod popover;
mod render;
pub mod toggle_grid;

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub use items::backlight::detect_ddc_bus;

/// Creates a unified status indicators capsule containing (1) status details button and (2) clock button.
/// Clicking the status button toggles Control Center; clicking the clock button toggles Calendar.
/// The two panels are mutually exclusive.
pub fn create_status_icons(
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::Box {
    let (status_box, status_button, separator, vol_icon, net_icon, vpn_icon, bat_widget) =
        render::build_status_row();

    // Initial update of volume icon on load
    items::volume::update_topbar_volume(&vol_icon);

    // Setup status popovers (VPN, Network, Volume, Battery)
    let popovers = popover::setup_status_popover(&vol_icon, &net_icon, &vpn_icon, &bat_widget);

    // Scroll controller for volume on status button
    let scroll_controller =
        gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let vol_icon_scroll = vol_icon.clone();
    let update_vol_scroll = popovers.update_volume_popover.clone();
    let pop_vol_scroll = popovers.vol_popover.clone();
    scroll_controller.connect_scroll(move |_, _dx, dy| {
        let current_vol = items::volume::get_current_volume();
        let step = 5.0;
        let new_vol = if dy < 0.0 {
            (current_vol + step).min(100.0)
        } else if dy > 0.0 {
            (current_vol - step).max(0.0)
        } else {
            current_vol
        };

        if (new_vol - current_vol).abs() > 0.1 {
            items::volume::set_volume(new_vol);
            items::volume::update_topbar_volume(&vol_icon_scroll);
            if pop_vol_scroll.is_visible() {
                update_vol_scroll();
            }
        }
        gtk4::glib::Propagation::Stop
    });
    status_button.add_controller(scroll_controller);

    let app_clone = app.clone();
    let ccw_clone = control_center_window.clone();
    let cw_clone = calendar_window.clone();
    let lw_clone = launcher_window.clone();
    let vol_icon_clone = vol_icon.clone();
    status_button.connect_clicked(move |_| {
        let launcher_active = { lw_clone.borrow().clone() };
        if let Some(win) = launcher_active {
            win.close();
        }

        let cal_active = { cw_clone.borrow().clone() };
        if let Some(win) = cal_active {
            win.close();
        }

        let existing = {
            let borrow = ccw_clone.borrow();
            borrow.clone()
        };
        if let Some(existing_window) = existing {
            existing_window.close();
        } else {
            let q_win =
                modal::create_cc_window(&app_clone, ccw_clone.clone(), vol_icon_clone.clone());
            if let Ok(mut borrow) = ccw_clone.try_borrow_mut() {
                *borrow = Some(q_win);
            }
        }
    });

    let clock_button = crate::widgets::clock::create_clock_widget(
        app,
        control_center_window.clone(),
        calendar_window.clone(),
        launcher_window.clone(),
    );

    status_box.append(&status_button);
    status_box.append(&separator);
    status_box.append(&clock_button);

    status_box
}
