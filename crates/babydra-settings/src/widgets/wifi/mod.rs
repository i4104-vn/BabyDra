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

    let wifi_status = babydra_common::services::system::wifi::get_wifi_state().0;
    let state = Rc::new(RefCell::new(WifiState {
        enabled: wifi_status,
        networks: Vec::new(),
    }));

    wifi_switch.set_active(wifi_status);

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

    let refresh_networks = {
        let state_clone = state.clone();
        let render_clone = render_networks.clone();
        move || {
            {
                let mut st = state_clone.borrow_mut();
                if st.enabled {
                    st.networks = babydra_common::services::system::wifi::scan_networks();
                } else {
                    st.networks.clear();
                }
            }
            render_clone();
        }
    };

    // Initialize list
    let refresh_init = refresh_networks.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        refresh_init();
        glib::ControlFlow::Break
    });

    // Trigger scan on state change or periodically
    let refresh_periodic = refresh_networks.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(6), move || {
        refresh_periodic();
        glib::ControlFlow::Continue
    });

    wifi_switch.connect_state_set(move |_, is_active| {
        let _ = babydra_common::services::system::wifi::set_wifi_enabled(is_active);
        let mut st = state.borrow_mut();
        st.enabled = is_active;
        if !is_active {
            st.networks.clear();
        }
        let render_now = render_networks.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
            render_now();
            glib::ControlFlow::Break
        });
        glib::Propagation::Proceed
    });

    overlay
}
