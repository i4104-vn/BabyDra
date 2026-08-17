//! Bluetooth devices management panel.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::channel;

use babydra_core::{get_bluetooth_devices, is_bluetooth_enabled, set_bluetooth_enabled, BtDevice};
use gtk4::prelude::*;

mod handler;
mod render;

pub struct BluetoothState {
    pub enabled: bool,
    pub devices: Vec<BtDevice>,
    pub is_loading: bool,
}

/// Creates a new `bluetooth widget`.
pub fn create_bluetooth_widget() -> gtk4::Widget {
    let (main_box, toggle_row, list_box) = render::build_bluetooth_ui();

    let state = Rc::new(RefCell::new(BluetoothState {
        enabled: false,
        devices: Vec::new(),
        is_loading: true,
    }));

    let toggle_row_c = toggle_row.clone();
    let state_c_init = state.clone();
    let (tx_status, rx_status) = channel::<bool>();
    std::thread::spawn(move || {
        let status = is_bluetooth_enabled();
        let _ = tx_status.send(status);
    });

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

    let render_devices = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            let st = state_clone.borrow();
            handler::render_device_list(&list_box_clone, &st);
        }
    };

    // Async thread scanning channel
    let (tx_devs, rx_devs) = channel::<Vec<BtDevice>>();

    let trigger_refresh = {
        let tx_c = tx_devs.clone();
        let state_c = state.clone();
        let list_box_c = list_box.clone();
        let render_c = render_devices.clone();
        move || {
            // ONLY fetch when tab is active/mapped!
            if !list_box_c.is_mapped() {
                return;
            }

            let (enabled, is_empty) = {
                let st = state_c.borrow();
                (st.enabled, st.devices.is_empty())
            };

            if enabled {
                if is_empty {
                    state_c.borrow_mut().is_loading = true;
                    render_c();
                }
                let tx_sub = tx_c.clone();
                std::thread::spawn(move || {
                    let devs = get_bluetooth_devices();
                    let _ = tx_sub.send(devs);
                });
            }
        }
    };

    let state_c_rx = state.clone();
    let render_c_rx = render_devices.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok(devs) = rx_devs.try_recv() {
            let mut st = state_c_rx.borrow_mut();
            st.devices = devs;
            st.is_loading = false;
            updated = true;
        }
        if updated {
            render_c_rx();
        }
        glib::ControlFlow::Continue
    });

    // Trigger fetch instantly when tab becomes active/mapped
    let trigger_map = trigger_refresh.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    // Trigger initial fetch if mapped
    let trigger_init = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        trigger_init();
        glib::ControlFlow::Break
    });

    // Refresh periodically (every 5s) ONLY when tab is mapped
    let trigger_periodic = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
        trigger_periodic();
        glib::ControlFlow::Continue
    });

    let trigger_switch = trigger_refresh.clone();
    let state_switch = state.clone();
    let render_switch = render_devices.clone();
    let toggle_row_switch = toggle_row.clone();
    toggle_row.switch.connect_state_set(move |is_active| {
        toggle_row_switch.set_active(is_active);
        std::thread::spawn(move || {
            set_bluetooth_enabled(is_active);
        });
        {
            let mut st = state_switch.borrow_mut();
            st.enabled = is_active;
            if !is_active {
                st.devices.clear();
                st.is_loading = false;
            }
        }
        if !is_active {
            render_switch();
        } else {
            trigger_switch();
        }
    });

    main_box.into()
}
