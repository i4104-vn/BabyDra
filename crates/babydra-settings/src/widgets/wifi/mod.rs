//! Wi-Fi configurations control panel.

use babydra_core::models::settings::wifi::WifiNetwork;
pub use babydra_core::models::settings::WifiState;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

mod handler;
mod render;

/// Creates a new `wifi widget`.
pub fn create_wifi_widget() -> gtk4::Widget {
    let (main_box, toggle_row, list_box, info_dialog, password_dialog, config_dialog) =
        render::build_wifi_ui();

    let info_dialog = Rc::new(info_dialog);
    let password_dialog = Rc::new(password_dialog);
    let config_dialog = Rc::new(config_dialog);

    let state = Rc::new(RefCell::new(WifiState {
        enabled: false,
        networks: Vec::new(),
        is_loading: true,
        connecting_ssid: None,
        active_network: Default::default(),
    }));

    // Async fetch initial Wi-Fi switch status off main thread
    let (tx_status, rx_status) = std::sync::mpsc::channel::<bool>();
    std::thread::spawn(move || {
        let status = babydra_core::services::system::wifi::get_wifi_state().0;
        let _ = tx_status.send(status);
    });

    let toggle_row_c = toggle_row.clone();
    let state_c_init = state.clone();
    glib::timeout_add_local(
        std::time::Duration::from_millis(50),
        move || match rx_status.try_recv() {
            Ok(status) => {
                toggle_row_c.switch.set_active(status);
                toggle_row_c.set_active(status);
                state_c_init.borrow_mut().enabled = status;
                glib::ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
        },
    );

    let (tx_connect_req, rx_connect_req) =
        std::sync::mpsc::channel::<(String, Option<String>, Option<String>)>();
    let (tx_connect, rx_connect) = std::sync::mpsc::channel::<()>();

    let render_networks: Rc<dyn Fn()> = Rc::new({
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        let info_dlg_c = info_dialog.clone();
        let pwd_dlg_c = password_dialog.clone();
        let cfg_dlg_c = config_dialog.clone();
        let tx_req_c = tx_connect_req.clone();
        move || {
            let state_ref = state_clone.borrow();
            handler::render_network_list(
                &list_box_clone,
                &state_ref,
                &info_dlg_c,
                &pwd_dlg_c,
                &cfg_dlg_c,
                tx_req_c.clone(),
            );
        }
    });

    let state_req = state.clone();
    let render_req = render_networks.clone();
    let tx_done_c = tx_connect.clone();
    let list_box_connect = list_box.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if !list_box_connect.is_mapped() {
            return glib::ControlFlow::Continue;
        }
        while let Ok((ssid, user, pwd)) = rx_connect_req.try_recv() {
            if state_req.borrow().connecting_ssid.is_some() {
                continue; // Ignore redundant requests if already connecting
            }
            state_req.borrow_mut().connecting_ssid = Some(ssid.clone());
            render_req();

            let tx_done = tx_done_c.clone();
            std::thread::spawn(move || {
                let _ = babydra_core::services::system::wifi::connect_wifi(
                    &ssid,
                    user.as_deref(),
                    pwd.as_deref(),
                );
                let _ = tx_done.send(());
            });
        }
        glib::ControlFlow::Continue
    });

    // Wire Password Dialog submit
    let pwd_dlg_inner = password_dialog.clone();
    let tx_req_pwd = tx_connect_req.clone();
    password_dialog.connect_submit(move |pwd, username| {
        let ssid = pwd_dlg_inner.ssid_lbl.text().to_string();
        let ssid_clean = ssid.trim_start_matches("Connect to ").to_string();
        pwd_dlg_inner.hide();
        let pwd_opt = if pwd.is_empty() { None } else { Some(pwd) };
        let _ = tx_req_pwd.send((ssid_clean, username, pwd_opt));
    });

    // Wire Info Dialog configure button click
    let info_dlg_inner = info_dialog.clone();
    let cfg_dlg_inner = config_dialog.clone();
    info_dialog.connect_configure(move || {
        let ssid = info_dlg_inner.ssid_lbl.text().to_string();
        let cfg_dlg_target = cfg_dlg_inner.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let ssid_clone = ssid.clone();
        std::thread::spawn(move || {
            let config = babydra_core::services::system::wifi::get_wifi_config(&ssid_clone);
            let _ = tx.send(config);
        });
        glib::timeout_add_local(std::time::Duration::from_millis(30), move || {
            if let Ok(config) = rx.try_recv() {
                cfg_dlg_target.show_for(&ssid, &config);
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    });

    // Wire Config Dialog save button click
    config_dialog.connect_save(move |ssid, config| {
        std::thread::spawn(move || {
            let _ = babydra_core::services::system::wifi::set_wifi_config(&ssid, &config);
        });
    });

    // Background thread scanning channel
    let (tx_scan, rx_scan) = std::sync::mpsc::channel::<(
        Vec<WifiNetwork>,
        babydra_core::models::network::ActiveNetworkInfo,
    )>();
    let scan_in_flight = Rc::new(Cell::new(false));
    let list_box_mapped_check = list_box.clone();
    let render_nets_loading = render_networks.clone();
    let trigger_wifi_scan = {
        let tx_c = tx_scan.clone();
        let state_c = state.clone();
        let list_box_c = list_box_mapped_check.clone();
        let render_c = render_nets_loading.clone();
        let scan_in_flight_c = scan_in_flight.clone();
        move || {
            // ONLY fetch when tab is active/mapped!
            if !list_box_c.is_mapped() {
                return;
            }

            let (enabled, is_empty) = {
                let state_ref = state_c.borrow();
                (state_ref.enabled, state_ref.networks.is_empty())
            };

            if enabled {
                if scan_in_flight_c.get() {
                    return;
                }
                scan_in_flight_c.set(true);
                if is_empty {
                    state_c.borrow_mut().is_loading = true;
                    render_c();
                }
                let tx_sub = tx_c.clone();
                std::thread::spawn(move || {
                    let nets = babydra_core::services::system::wifi::scan_networks();
                    let active = babydra_core::services::system::network::get_active_network_info();
                    let _ = tx_sub.send((nets, active));
                });
            }
        }
    };

    // Wire Info Dialog forget button click
    let info_dlg_forget = info_dialog.clone();
    let trigger_forget = trigger_wifi_scan.clone();
    info_dialog.connect_forget(move || {
        let ssid = info_dlg_forget.ssid_lbl.text().to_string();
        let trigger_scan_c = trigger_forget.clone();
        std::thread::spawn(move || {
            babydra_core::services::system::wifi::forget_wifi(&ssid);
        });
        glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
            trigger_scan_c();
            glib::ControlFlow::Break
        });
    });

    let state_done = state.clone();
    let trigger_done = trigger_wifi_scan.clone();
    let render_done = render_networks.clone();
    let list_box_done = list_box.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        if !list_box_done.is_mapped() {
            return glib::ControlFlow::Continue;
        }
        while rx_connect.try_recv().is_ok() {
            state_done.borrow_mut().connecting_ssid = None;
            render_done();
            trigger_done();
        }
        glib::ControlFlow::Continue
    });

    let state_scan_render = state.clone();
    let render_nets = render_networks.clone();
    let list_box_scan = list_box.clone();
    let scan_in_flight_rx = scan_in_flight.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok((nets, active)) = rx_scan.try_recv() {
            scan_in_flight_rx.set(false);
            let mut state_ref = state_scan_render.borrow_mut();
            let changed = state_ref.networks != nets
                || state_ref.active_network != active
                || state_ref.is_loading;
            state_ref.networks = nets;
            state_ref.active_network = active;
            state_ref.is_loading = false;
            updated |= changed;
        }
        if updated && list_box_scan.is_mapped() {
            render_nets();
        }
        glib::ControlFlow::Continue
    });

    // Scan when the tab becomes active/mapped.
    let trigger_map = trigger_wifi_scan.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    // Trigger periodic scan (every 6s) ONLY when tab is mapped
    let trigger_periodic = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(6), move || {
        trigger_periodic();
        glib::ControlFlow::Continue
    });

    let trigger_initial = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
        trigger_initial();
        glib::ControlFlow::Break
    });

    // Poll the adapter state off the GTK thread so external Wi-Fi changes are
    // reflected without blocking navigation or list rendering.
    let (tx_status_poll, rx_status_poll) = std::sync::mpsc::channel::<bool>();
    let status_in_flight = Rc::new(Cell::new(false));
    let trigger_status_scan = trigger_wifi_scan.clone();
    let state_status_poll = state.clone();
    let render_status_poll = render_networks.clone();
    let toggle_status_poll = toggle_row.clone();
    let list_box_status_poll = list_box.clone();
    let status_in_flight_poll = status_in_flight.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        while let Ok(enabled) = rx_status_poll.try_recv() {
            status_in_flight_poll.set(false);
            let changed = state_status_poll.borrow().enabled != enabled;
            if !changed {
                continue;
            }
            toggle_status_poll.switch.set_active(enabled);
            toggle_status_poll.set_active(enabled);
            let mut state_ref = state_status_poll.borrow_mut();
            state_ref.enabled = enabled;
            if !enabled {
                state_ref.networks.clear();
                state_ref.is_loading = false;
            }
            drop(state_ref);
            if list_box_status_poll.is_mapped() {
                if enabled {
                    trigger_status_scan();
                } else {
                    render_status_poll();
                }
            }
        }
        glib::ControlFlow::Continue
    });

    let tx_status_trigger = tx_status_poll.clone();
    let status_in_flight_trigger = status_in_flight.clone();
    let list_box_status_trigger = list_box.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
        if list_box_status_trigger.is_mapped() && !status_in_flight_trigger.get() {
            status_in_flight_trigger.set(true);
            let tx = tx_status_trigger.clone();
            std::thread::spawn(move || {
                let _ = tx.send(babydra_core::services::system::wifi::get_wifi_state().0);
            });
        }
        glib::ControlFlow::Continue
    });

    let trigger_switch = trigger_wifi_scan.clone();
    let state_switch = state.clone();
    let render_switch = render_networks.clone();
    let toggle_row_switch = toggle_row.clone();
    toggle_row.switch.connect_state_set(move |is_active| {
        let is_active_bool = is_active;
        toggle_row_switch.set_active(is_active_bool);
        {
            let mut state_ref = state_switch.borrow_mut();
            state_ref.enabled = is_active_bool;
            if !is_active_bool {
                state_ref.networks.clear();
                state_ref.is_loading = false;
            }
        }
        std::thread::spawn(move || {
            babydra_core::services::system::wifi::set_wifi_enabled(is_active_bool);
        });
        if is_active_bool {
            trigger_switch();
        } else {
            render_switch();
        }
    });

    main_box.into()
}
