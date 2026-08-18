//! Wi-Fi configurations control panel.

pub use babydra_core::models::settings::WifiState;
use babydra_core::models::settings::wifi::WifiNetwork;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod handler;
mod render;

/// Creates a new `wifi widget`.
pub fn create_wifi_widget() -> gtk4::Widget {
    let (overlay, toggle_row, list_box, info_dialog, password_dialog, config_dialog) =
        render::build_wifi_ui();

    let info_dialog = Rc::new(info_dialog);
    let password_dialog = Rc::new(password_dialog);
    let config_dialog = Rc::new(config_dialog);

    let state = Rc::new(RefCell::new(WifiState {
        enabled: false,
        networks: Vec::new(),
        is_loading: true,
        connecting_ssid: None,
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
            let st = state_clone.borrow();
            handler::render_network_list(
                &list_box_clone,
                &st,
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
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
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
    let (tx_scan, rx_scan) = std::sync::mpsc::channel::<Vec<WifiNetwork>>();
    let list_box_mapped_check = list_box.clone();
    let render_nets_loading = render_networks.clone();
    let trigger_wifi_scan = {
        let tx_c = tx_scan.clone();
        let state_c = state.clone();
        let list_box_c = list_box_mapped_check.clone();
        let render_c = render_nets_loading.clone();
        move || {
            // ONLY fetch when tab is active/mapped!
            if !list_box_c.is_mapped() {
                return;
            }

            let (enabled, is_empty) = {
                let st = state_c.borrow();
                (st.enabled, st.networks.is_empty())
            };

            if enabled {
                if is_empty {
                    state_c.borrow_mut().is_loading = true;
                    render_c();
                }
                let tx_sub = tx_c.clone();
                std::thread::spawn(move || {
                    let nets = babydra_core::services::system::wifi::scan_networks();
                    let _ = tx_sub.send(nets);
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
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        while let Ok(_) = rx_connect.try_recv() {
            state_done.borrow_mut().connecting_ssid = None;
            render_done();
            trigger_done();
        }
        glib::ControlFlow::Continue
    });

    let state_scan_render = state.clone();
    let render_nets = render_networks.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok(nets) = rx_scan.try_recv() {
            let mut st = state_scan_render.borrow_mut();
            st.networks = nets;
            st.is_loading = false;
            updated = true;
        }
        if updated {
            render_nets();
        }
        glib::ControlFlow::Continue
    });

    // Trigger scan instantly when tab becomes active/mapped
    let trigger_map = trigger_wifi_scan.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    let trigger_init = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        trigger_init();
        glib::ControlFlow::Break
    });

    // Trigger periodic scan (every 6s) ONLY when tab is mapped
    let trigger_periodic = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(6), move || {
        trigger_periodic();
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
            let mut st = state_switch.borrow_mut();
            st.enabled = is_active_bool;
            if !is_active_bool {
                st.networks.clear();
                st.is_loading = false;
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

    overlay.into()
}
