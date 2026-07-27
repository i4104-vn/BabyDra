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

pub fn create_wifi_widget() -> gtk4::Box {
    let (main_box, wifi_switch, list_box) = render::build_wifi_ui();

    let wifi_status = babydra_common::services::system::wifi::get_wifi_state().0;
    let state = Rc::new(RefCell::new(WifiState {
        enabled: wifi_status,
        networks: Vec::new(),
    }));

    wifi_switch.set_active(wifi_status);

    let render_networks = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            let st = state_clone.borrow();
            handler::render_network_list(&list_box_clone, &st);
        }
    };

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

    main_box
}
