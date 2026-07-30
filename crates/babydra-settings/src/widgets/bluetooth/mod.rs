//! Bluetooth devices management panel.

use std::cell::RefCell;
use std::rc::Rc;

use babydra_common::{is_bluetooth_enabled, set_bluetooth_enabled, get_bluetooth_devices, BtDevice};

mod handler;
mod render;

pub struct BluetoothState {
    pub enabled: bool,
    pub devices: Vec<BtDevice>,
}

pub fn create_bluetooth_widget() -> gtk4::Box {
    let (main_box, bt_switch, list_box) = render::build_bluetooth_ui();

    let state = Rc::new(RefCell::new(BluetoothState {
        enabled: false,
        devices: Vec::new(),
    }));

    let bt_switch_c = bt_switch.clone();
    let state_c_init = state.clone();
    let (tx_status, rx_status) = std::sync::mpsc::channel::<bool>();
    std::thread::spawn(move || {
        let status = is_bluetooth_enabled();
        let _ = tx_status.send(status);
    });

    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        match rx_status.try_recv() {
            Ok(status) => {
                bt_switch_c.set_active(status);
                state_c_init.borrow_mut().enabled = status;
                glib::ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
        }
    });

    let render_devices = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            let st = state_clone.borrow();
            handler::render_device_list(&list_box_clone, &st);
        }
    };

    let refresh_devices = {
        let state_clone = state.clone();
        let render_clone = render_devices.clone();
        move || {
            {
                let mut st = state_clone.borrow_mut();
                if st.enabled {
                    st.devices = get_bluetooth_devices();
                } else {
                    st.devices.clear();
                }
            }
            render_clone();
        }
    };

    // Load initial list
    let refresh_init = refresh_devices.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        refresh_init();
        glib::ControlFlow::Break
    });

    // Refresh periodically
    let refresh_periodic = refresh_devices.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
        refresh_periodic();
        glib::ControlFlow::Continue
    });

    bt_switch.connect_state_set(move |_, is_active| {
        set_bluetooth_enabled(is_active);
        let mut st = state.borrow_mut();
        st.enabled = is_active;
        if !is_active {
            st.devices.clear();
        }
        let render_now = render_devices.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
            render_now();
            glib::ControlFlow::Break
        });
        glib::Propagation::Proceed
    });

    main_box
}
