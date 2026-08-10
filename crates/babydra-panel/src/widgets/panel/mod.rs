pub mod items;
pub mod modal;
pub mod toggle_grid;
mod render;

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub use items::backlight::detect_ddc_bus;

struct PopoverRow {
    key: String,
    val: String,
    css_class: Option<String>,
}

impl PopoverRow {
    fn new(key: &str, val: &str, css_class: Option<&str>) -> Self {
        Self {
            key: key.to_string(),
            val: val.to_string(),
            css_class: css_class.map(|s| s.to_string()),
        }
    }
}

fn get_speed_color_class(bytes_per_sec: f64) -> &'static str {
    if bytes_per_sec > 1_048_576.0 {
        "speed-high"
    } else if bytes_per_sec > 102_400.0 {
        "speed-medium"
    } else {
        "speed-low"
    }
}

fn build_popover_card(title: &str, rows: Vec<PopoverRow>) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    card.add_css_class("status-popover-card");
    card.set_margin_top(4);
    card.set_margin_bottom(4);
    card.set_margin_start(6);
    card.set_margin_end(6);

    let title_lbl = gtk4::Label::new(Some(title));
    title_lbl.add_css_class("status-popover-header");
    title_lbl.set_halign(gtk4::Align::Start);
    card.append(&title_lbl);

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("status-popover-sep");
    card.append(&sep);

    for row in rows {
        let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row_box.add_css_class("status-popover-row");

        let key_lbl = gtk4::Label::new(Some(&row.key));
        key_lbl.add_css_class("status-popover-key");
        key_lbl.set_halign(gtk4::Align::Start);
        key_lbl.set_hexpand(true);

        let val_lbl = gtk4::Label::new(Some(&row.val));
        val_lbl.add_css_class("status-popover-val");
        if let Some(ref cls) = row.css_class {
            val_lbl.add_css_class(cls);
        }
        val_lbl.set_halign(gtk4::Align::End);

        row_box.append(&key_lbl);
        row_box.append(&val_lbl);
        card.append(&row_box);
    }

    card
}

fn attach_hover_popover(
    anchor_widget: &impl IsA<gtk4::Widget>,
    popover: &gtk4::Popover,
    update_fn: Rc<dyn Fn()>,
) {
    popover.set_autohide(false);

    let is_hovered = Rc::new(RefCell::new(false));

    // Motion controller on the anchor icon
    let motion_icon = gtk4::EventControllerMotion::new();

    let is_hovered_icon_enter = is_hovered.clone();
    let popover_enter = popover.clone();
    let update_fn_enter = update_fn.clone();
    motion_icon.connect_enter(move |_, _, _| {
        *is_hovered_icon_enter.borrow_mut() = true;
        update_fn_enter();
        popover_enter.popup();
    });

    let is_hovered_icon_leave = is_hovered.clone();
    let popover_leave = popover.clone();
    motion_icon.connect_leave(move |_| {
        *is_hovered_icon_leave.borrow_mut() = false;
        let is_h = is_hovered_icon_leave.clone();
        let pop = popover_leave.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            if !*is_h.borrow() {
                pop.popdown();
            }
            gtk4::glib::ControlFlow::Break
        });
    });
    anchor_widget.add_controller(motion_icon);

    // Motion controller on Popover content to keep open while mouse is inside popover card
    let motion_popover = gtk4::EventControllerMotion::new();
    let is_hovered_pop_enter = is_hovered.clone();
    motion_popover.connect_enter(move |_, _, _| {
        *is_hovered_pop_enter.borrow_mut() = true;
    });

    let is_hovered_pop_leave = is_hovered.clone();
    let popover_pop_leave = popover.clone();
    motion_popover.connect_leave(move |_| {
        *is_hovered_pop_leave.borrow_mut() = false;
        let is_h = is_hovered_pop_leave.clone();
        let pop = popover_pop_leave.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            if !*is_h.borrow() {
                pop.popdown();
            }
            gtk4::glib::ControlFlow::Break
        });
    });
    popover.add_controller(motion_popover);
}

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

    // Initial update of volume icon on load
    items::volume::update_topbar_volume_icon(&vol_icon);

    // Create custom GTK4 Popovers anchored to each status icon
    let vpn_popover = babydra_utils::components::create_popover(&vpn_icon, gtk4::PositionType::Bottom, "status-popover");
    let net_popover = babydra_utils::components::create_popover(&net_icon, gtk4::PositionType::Bottom, "status-popover");
    let vol_popover = babydra_utils::components::create_popover(&vol_icon, gtk4::PositionType::Bottom, "status-popover");

    let bat_popover_opt = if let Some(ref bat_area) = bat_widget {
        let bat_pop = babydra_utils::components::create_popover(bat_area, gtk4::PositionType::Bottom, "status-popover");
        Some(bat_pop)
    } else {
        None
    };

    let update_vpn_tooltip = {
        let vpn_icon_c = vpn_icon.clone();
        let vpn_popover_c = vpn_popover.clone();
        Rc::new(move || {
            if let Some(active_vpn) = babydra_common::services::system::vpn::get_active_vpn_fast() {
                let proto_str = if !active_vpn.cipher.is_empty() {
                    format!("{} ({})", active_vpn.conn_type.to_uppercase(), active_vpn.cipher)
                } else {
                    active_vpn.conn_type.to_uppercase()
                };

                let display_name = if active_vpn.name.chars().count() > 30 {
                    let truncated: String = active_vpn.name.chars().take(30).collect();
                    format!("{}...", truncated)
                } else {
                    active_vpn.name.clone()
                };

                let mut rows = vec![
                    PopoverRow::new("Status", "Active", None),
                    PopoverRow::new("Name", &display_name, None),
                    PopoverRow::new("Type", &proto_str, None),
                ];

                if !active_vpn.remote_server.is_empty() {
                    rows.push(PopoverRow::new("Server", &active_vpn.remote_server, None));
                } else if !active_vpn.gateway.is_empty() {
                    rows.push(PopoverRow::new("Gateway", &active_vpn.gateway, None));
                }

                if !active_vpn.ip_address.is_empty() {
                    rows.push(PopoverRow::new("VPN IP", &active_vpn.ip_address, None));
                }

                if !active_vpn.username.is_empty() {
                    rows.push(PopoverRow::new("User", &active_vpn.username, None));
                }

                if !active_vpn.dev_iface.is_empty() {
                    rows.push(PopoverRow::new("Interface", &active_vpn.dev_iface, None));
                }

                let card = build_popover_card("VPN Connection", rows);
                vpn_popover_c.set_child(Some(&card));
                vpn_icon_c.set_visible(true);
            } else {
                vpn_icon_c.set_visible(false);
            }
        })
    };

    let update_network_tooltip = {
        let net_popover_c = net_popover.clone();
        Rc::new(move || {
            let (enabled, ssid) = babydra_common::helper::wifi::get_wifi_state();
            let speed = babydra_common::helper::network::get_network_speed();
            let local_ip = babydra_common::helper::network::get_local_ip();

            let rx_cls = get_speed_color_class(speed.rx_speed);
            let tx_cls = get_speed_color_class(speed.tx_speed);

            let rows = if !enabled {
                vec![PopoverRow::new("Status", "Disabled", None)]
            } else if ssid == "Disconnected" || ssid == "Off" {
                vec![PopoverRow::new("Status", "Disconnected", None)]
            } else {
                vec![
                    PopoverRow::new("SSID", &ssid, None),
                    PopoverRow::new("IP Address", &local_ip, None),
                    PopoverRow::new("Download", &format!("↓ {}", babydra_common::helper::network::format_speed(speed.rx_speed)), Some(rx_cls)),
                    PopoverRow::new("Upload", &format!("↑ {}", babydra_common::helper::network::format_speed(speed.tx_speed)), Some(tx_cls)),
                ]
            };

            let card = build_popover_card("Network Connection", rows);
            net_popover_c.set_child(Some(&card));
        })
    };

    let update_volume_popover = {
        let vol_icon_c = vol_icon.clone();
        let vol_popover_c = vol_popover.clone();
        Rc::new(move || {
            items::volume::update_topbar_volume_icon(&vol_icon_c);
            let is_m = items::volume::is_muted();
            let vol_pct = items::volume::get_current_volume();
            let dev_name = items::volume::get_active_output_device_name();

            let vol_str = if is_m {
                format!("Muted ({:.0}%)", vol_pct)
            } else {
                format!("{:.0}%", vol_pct)
            };

            let mut rows = vec![
                PopoverRow::new("Volume", &vol_str, None),
            ];
            if let Some(ref dev) = dev_name {
                rows.push(PopoverRow::new("Device", dev, None));
            }

            let card = build_popover_card("Audio Output", rows);
            vol_popover_c.set_child(Some(&card));
        })
    };

    let bat_popover_c = bat_popover_opt.clone();
    let update_battery_popover = Rc::new(move || {
        if let Some(ref bat_pop) = bat_popover_c {
            if let Some(info) = render::get_battery_info() {
                let mut rows = vec![
                    PopoverRow::new("Level", &format!("{}%", info.percentage), None),
                    PopoverRow::new("State", &info.status_text, None),
                ];
                if let Some(ref rem) = info.time_remaining {
                    rows.push(PopoverRow::new("Remaining", rem, None));
                }
                let card = build_popover_card("Power & Battery", rows);
                bat_pop.set_child(Some(&card));
            }
        }
    });

    // Initial check of VPN visibility
    if babydra_common::services::system::vpn::get_active_vpn_fast().is_some() {
        vpn_icon.set_visible(true);
    } else {
        vpn_icon.set_visible(false);
    }

    // Attach smooth hover popover handlers to all topbar icons
    attach_hover_popover(&vpn_icon, &vpn_popover, update_vpn_tooltip.clone());
    attach_hover_popover(&net_icon, &net_popover, update_network_tooltip.clone());
    attach_hover_popover(&vol_icon, &vol_popover, update_volume_popover.clone());

    if let Some(ref bat_area) = bat_widget {
        if let Some(ref bat_pop) = bat_popover_opt {
            attach_hover_popover(bat_area, bat_pop, update_battery_popover.clone());
        }
    }

    let scroll_controller = gtk4::EventControllerScroll::new(
        gtk4::EventControllerScrollFlags::VERTICAL
    );
    let vol_icon_scroll = vol_icon.clone();
    let update_vol_scroll = update_volume_popover.clone();
    let pop_vol_scroll = vol_popover.clone();
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
            if pop_vol_scroll.is_visible() {
                update_vol_scroll();
            }
        }
        gtk4::glib::Propagation::Stop
    });
    status_button.add_controller(scroll_controller);

    let vpn_pop_t = vpn_popover.clone();
    let net_pop_t = net_popover.clone();
    let vol_pop_t = vol_popover.clone();
    let bat_pop_t = bat_popover_opt.clone();

    let update_vpn_t = update_vpn_tooltip.clone();
    let update_net_t = update_network_tooltip.clone();
    let update_vol_t = update_volume_popover.clone();
    let update_bat_t = update_battery_popover.clone();

    let bat_widget_timer = bat_widget.clone();
    let vpn_icon_timer = vpn_icon.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(2000), move || {
        // Only update active popover if it's currently open
        if vpn_pop_t.is_visible() {
            update_vpn_t();
        }
        if net_pop_t.is_visible() {
            update_net_t();
        }
        if vol_pop_t.is_visible() {
            update_vol_t();
        }
        if let Some(ref bp) = bat_pop_t {
            if bp.is_visible() {
                update_bat_t();
            }
        }

        // Check VPN visibility state periodically
        let vpn_active = babydra_common::services::system::vpn::get_active_vpn_fast().is_some();
        if vpn_icon_timer.is_visible() != vpn_active {
            vpn_icon_timer.set_visible(vpn_active);
        }

        if let Some(ref bat_area) = bat_widget_timer {
            bat_area.queue_draw();
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
