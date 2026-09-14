pub mod items;
pub mod modal;
pub mod popover;
mod render;
pub mod state;
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
    ws_popover: gtk4::Popover,
) -> gtk4::Box {
    let (status_box, status_button, separator, vol_icon, net_widgets, vpn_icon, bat_widget) =
        render::build_status_row();

    // Initial update of volume icon on load
    items::volume::update_topbar_volume(&vol_icon);

    // Setup status popovers (VPN, Network, Volume, Battery)
    let popovers = popover::setup_status_popover(
        &vol_icon,
        &net_widgets,
        &vpn_icon,
        &bat_widget,
        control_center_window.clone(),
        calendar_window.clone(),
        launcher_window.clone(),
        ws_popover,
    );

    // Real-time event listener for system volume & mute changes
    {
        let vol_icon_l = vol_icon.clone();
        let cached_vol_l = popovers.current_volume.clone();
        let cached_muted_l = popovers.current_muted.clone();
        let update_vol_pop_l = popovers.update_volume_popover.clone();
        let vol_pop_l = popovers.vol_popover.clone();

        babydra_core::services::system::volume::spawn_volume_listener(move |state| {
            cached_vol_l.set(state.volume);
            cached_muted_l.set(state.muted);
            items::volume::update_topbar_volume_state(&vol_icon_l, state.volume, state.muted);
            if vol_pop_l.is_visible() {
                update_vol_pop_l();
            }
            modal::sync_cc_volume(state.volume, state.muted);
        });
    }

    // Real-time event listener for system brightness changes
    {
        babydra_core::services::system::backlight::spawn_brightness_listener(move |val| {
            modal::sync_cc_brightness(val);
        });
    }

    // Scroll controller for volume on status button
    let scroll_controller = gtk4::EventControllerScroll::new(
        gtk4::EventControllerScrollFlags::VERTICAL | gtk4::EventControllerScrollFlags::DISCRETE,
    );
    let vol_icon_scroll = vol_icon.clone();
    let update_vol_scroll = popovers.update_volume_popover.clone();
    let pop_vol_scroll = popovers.vol_popover.clone();
    let cached_vol = popovers.current_volume.clone();
    let cached_muted = popovers.current_muted.clone();
    scroll_controller.connect_scroll(move |_, _dx, dy| {
        let current_vol = cached_vol.get();
        let step = 5.0;
        let new_vol = if dy < 0.0 {
            (current_vol + step).min(100.0)
        } else if dy > 0.0 {
            (current_vol - step).max(0.0)
        } else {
            current_vol
        };

        if (new_vol - current_vol).abs() > 0.1 {
            cached_vol.set(new_vol);
            let was_muted = cached_muted.get();
            if was_muted && new_vol > 0.0 {
                cached_muted.set(false);
                babydra_core::services::system::volume::set_muted(false);
            }
            items::volume::set_volume(new_vol);
            items::volume::update_topbar_volume_state(&vol_icon_scroll, new_vol, new_vol == 0.0);
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
    let popovers_click = popovers.clone();
    status_button.connect_clicked(move |_| {
        popovers_click.popdown_all();

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

    let popovers_clock = popovers.clone();
    let clock_button = crate::widgets::clock::create_clock_widget(
        app,
        control_center_window.clone(),
        calendar_window.clone(),
        launcher_window.clone(),
        Some(Rc::new(move || popovers_clock.popdown_all())),
    );

    status_box.append(&status_button);
    status_box.append(&separator);
    status_box.append(&clock_button);

    status_box
}
