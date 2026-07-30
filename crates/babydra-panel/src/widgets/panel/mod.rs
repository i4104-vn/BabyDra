pub mod items;
pub mod modal;
pub mod toggle_grid;
mod render;

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub use items::backlight::detect_ddc_bus;

/// Creates a unified status indicators capsule containing (1) status details button and (2) clock button.
/// Clicking the status button toggles Control Center; clicking the clock button toggles Calendar.
/// The two panels are mutually exclusive.
pub fn create_status_indicators(
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::Box {
    let (status_box, status_button, separator, vol_icon, net_icon, vpn_icon, bat_widget) = render::build_status_indicators_ui();

    // Initial update of volume & network tooltips on load
    items::volume::update_topbar_volume_icon(&vol_icon);

    let update_vpn_tooltip = {
        let vpn_icon_c = vpn_icon.clone();
        Rc::new(move || {
            if let Some(active_vpn) = babydra_common::services::system::vpn::get_active_vpn_fast() {
                let vpn_tooltip = if !active_vpn.gateway.is_empty() {
                    format!(
                        "VPN: Active\nName: {}\nType: {}\nGateway: {}",
                        active_vpn.name,
                        active_vpn.conn_type.to_uppercase(),
                        active_vpn.gateway
                    )
                } else {
                    format!(
                        "VPN: Active\nName: {}\nType: {}",
                        active_vpn.name,
                        active_vpn.conn_type.to_uppercase()
                    )
                };
                vpn_icon_c.set_tooltip_text(Some(&vpn_tooltip));
                vpn_icon_c.set_visible(true);
            } else {
                vpn_icon_c.set_visible(false);
            }
        })
    };

    let update_network_tooltip = {
        let net_icon_c = net_icon.clone();
        Rc::new(move || {
            let (enabled, ssid) = babydra_common::helper::wifi::get_wifi_state();
            let speed = babydra_common::helper::network::get_network_speed();

            let net_tooltip = if !enabled {
                "Network: Disabled".to_string()
            } else if ssid == "Disconnected" || ssid == "Off" {
                "Network: Disconnected".to_string()
            } else {
                format!(
                    "Network: {}\n↓ {}   ↑ {}",
                    ssid,
                    babydra_common::helper::network::format_speed(speed.rx_speed),
                    babydra_common::helper::network::format_speed(speed.tx_speed)
                )
            };
            net_icon_c.set_tooltip_text(Some(&net_tooltip));
        })
    };

    update_vpn_tooltip();
    update_network_tooltip();

    let scroll_controller = gtk4::EventControllerScroll::new(
        gtk4::EventControllerScrollFlags::VERTICAL
    );
    let vol_icon_scroll = vol_icon.clone();
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
            items::volume::update_topbar_volume_icon(&vol_icon_scroll);
        }
        gtk4::glib::Propagation::Stop
    });
    status_button.add_controller(scroll_controller);


    let motion_controller = gtk4::EventControllerMotion::new();
    let update_net_enter = update_network_tooltip.clone();
    let update_vpn_enter = update_vpn_tooltip.clone();
    let vol_icon_enter = vol_icon.clone();
    motion_controller.connect_enter(move |_, _, _| {
        items::volume::update_topbar_volume_icon(&vol_icon_enter);
        update_net_enter();
        update_vpn_enter();
    });
    status_button.add_controller(motion_controller);

    let vol_icon_timer = vol_icon.clone();
    let update_net_timer = update_network_tooltip.clone();
    let update_vpn_timer = update_vpn_tooltip.clone();
    let bat_widget_timer = bat_widget.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(1500), move || {
        items::volume::update_topbar_volume_icon(&vol_icon_timer);
        update_net_timer();
        update_vpn_timer();
        if let Some(ref bat_area) = bat_widget_timer {
            if let Some(info) = render::get_battery_info() {
                let status_str = if info.is_charging { "Charging" } else { "Discharging" };
                bat_area.set_tooltip_text(Some(&format!("Battery: {}% ({})", info.percentage, status_str)));
                bat_area.queue_draw();
            }
        }
        gtk4::glib::ControlFlow::Continue
    });

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
            let q_win = modal::create_control_center_window(&app_clone, ccw_clone.clone(), vol_icon_clone.clone());
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
