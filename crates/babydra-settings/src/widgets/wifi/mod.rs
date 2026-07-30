//! Wi-Fi configurations control panel.

use std::cell::RefCell;
use std::rc::Rc;
use babydra_common::models::wifi::WifiNetwork;

mod handler;
mod render;

pub struct WifiState {
    pub enabled: bool,
    pub networks: Vec<WifiNetwork>,
}

pub fn create_wifi_widget() -> gtk4::Overlay {
    let (overlay, wifi_switch, list_box, info_dialog, password_dialog, config_dialog) = render::build_wifi_ui();

    let info_dialog = Rc::new(info_dialog);
    let password_dialog = Rc::new(password_dialog);
    let config_dialog = Rc::new(config_dialog);

    let state = Rc::new(RefCell::new(WifiState {
        enabled: false,
        networks: Vec::new(),
    }));

    // Async fetch initial Wi-Fi switch status off main thread
    let (tx_status, rx_status) = std::sync::mpsc::channel::<bool>();
    std::thread::spawn(move || {
        let status = babydra_common::services::system::wifi::get_wifi_state().0;
        let _ = tx_status.send(status);
    });

    let wifi_switch_c = wifi_switch.clone();
    let state_c_init = state.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        match rx_status.try_recv() {
            Ok(status) => {
                wifi_switch_c.set_active(status);
                state_c_init.borrow_mut().enabled = status;
                glib::ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
        }
    });

    let render_networks = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        let info_dlg_c = info_dialog.clone();
        let pwd_dlg_c = password_dialog.clone();
        let cfg_dlg_c = config_dialog.clone();
        move || {
            let st = state_clone.borrow();
            handler::render_network_list(
                &list_box_clone,
                &st,
                &info_dlg_c,
                &pwd_dlg_c,
                &cfg_dlg_c,
            );
        }
    };

    // Wire Password Dialog submit
    let pwd_dlg_inner = password_dialog.clone();
    password_dialog.connect_submit(move |pwd, username| {
        let ssid = pwd_dlg_inner.ssid_lbl.text().to_string();
        let ssid_clean = ssid.trim_start_matches("Connect to ").to_string();
        pwd_dlg_inner.hide();
        std::thread::spawn(move || {
            let user_ref = username.as_deref();
            let pwd_ref = if pwd.is_empty() { None } else { Some(pwd.as_str()) };
            let _ = babydra_common::services::system::wifi::connect_wifi(&ssid_clean, user_ref, pwd_ref);
        });
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
            let config = babydra_common::services::system::wifi::get_wifi_config(&ssid_clone);
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
            let _ = babydra_common::services::system::wifi::set_wifi_config(&ssid, &config);
        });
    });

    // Background thread scanning channel
    let (tx_scan, rx_scan) = std::sync::mpsc::channel::<Vec<WifiNetwork>>();
    let trigger_wifi_scan = {
        let tx_c = tx_scan.clone();
        let state_c = state.clone();
        move || {
            if state_c.borrow().enabled {
                let tx_sub = tx_c.clone();
                std::thread::spawn(move || {
                    let nets = babydra_common::services::system::wifi::scan_networks();
                    let _ = tx_sub.send(nets);
                });
            }
        }
    };

    let state_scan_render = state.clone();
    let render_nets = render_networks.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok(nets) = rx_scan.try_recv() {
            state_scan_render.borrow_mut().networks = nets;
            updated = true;
        }
        if updated {
            render_nets();
        }
        glib::ControlFlow::Continue
    });

    // Trigger initial scan
    let trigger_init = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        trigger_init();
        glib::ControlFlow::Break
    });

    // Trigger periodic scan (every 6s)
    let trigger_periodic = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(6), move || {
        trigger_periodic();
        glib::ControlFlow::Continue
    });

    let trigger_switch = trigger_wifi_scan.clone();
    let state_switch = state.clone();
    let render_switch = render_networks.clone();
    wifi_switch.connect_state_set(move |_, is_active| {
        let is_active_bool = is_active;
        state_switch.borrow_mut().enabled = is_active_bool;
        std::thread::spawn(move || {
            babydra_common::services::system::wifi::set_wifi_enabled(is_active_bool);
        });
        if is_active_bool {
            trigger_switch();
        } else {
            state_switch.borrow_mut().networks.clear();
            render_switch();
        }
        glib::Propagation::Proceed
    });

    overlay
}
