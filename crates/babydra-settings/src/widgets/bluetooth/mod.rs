//! Bluetooth devices management panel.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::channel;

use babydra_core::models::settings::bluetooth::BtDevice;
pub use babydra_core::models::settings::BluetoothState;
use babydra_core::{get_bt_devices, is_bluetooth_enabled, set_bt_enabled};
use gtk4::prelude::*;

mod handler;
mod render;

/// Creates a new `bluetooth widget`.
pub fn create_bt_widget() -> gtk4::Widget {
    let (main_box, toggle_row, list_box, refresh_btn) = render::build_bluetooth_ui();

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
            let state_ref = state_clone.borrow();
            handler::render_device_list(&list_box_clone, &state_ref);
        }
    };

    render_devices();

    // Async thread scanning channel
    let (tx_devs, rx_devs) = channel::<Vec<BtDevice>>();
    let device_scan_in_flight = Rc::new(Cell::new(false));

    let trigger_refresh = {
        let tx_c = tx_devs.clone();
        let state_c = state.clone();
        let list_box_c = list_box.clone();
        let render_c = render_devices.clone();
        let device_scan_in_flight_c = device_scan_in_flight.clone();
        move || {
            // ONLY fetch when tab is active/mapped!
            if !list_box_c.is_mapped() {
                return;
            }

            let (enabled, is_empty) = {
                let state_ref = state_c.borrow();
                (state_ref.enabled, state_ref.devices.is_empty())
            };

            if enabled {
                if device_scan_in_flight_c.get() {
                    return;
                }
                device_scan_in_flight_c.set(true);
                if is_empty {
                    state_c.borrow_mut().is_loading = true;
                    render_c();
                }
                let tx_sub = tx_c.clone();
                std::thread::spawn(move || {
                    let devs = get_bt_devices();
                    let _ = tx_sub.send(devs);
                });
            }
        }
    };

    let trigger_refresh_btn = trigger_refresh.clone();
    refresh_btn.connect_clicked(move |_| {
        trigger_refresh_btn();
    });

    let state_c_rx = state.clone();
    let render_c_rx = render_devices.clone();
    let list_box_rx = list_box.clone();
    let device_scan_in_flight_rx = device_scan_in_flight.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok(devs) = rx_devs.try_recv() {
            device_scan_in_flight_rx.set(false);
            let mut state_ref = state_c_rx.borrow_mut();
            let changed = state_ref.devices != devs || state_ref.is_loading;
            state_ref.devices = devs;
            state_ref.is_loading = false;
            updated |= changed;
        }
        if updated && list_box_rx.is_mapped() {
            render_c_rx();
        }
        glib::ControlFlow::Continue
    });

    // Trigger fetch instantly when tab becomes active/mapped
    let trigger_map = trigger_refresh.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    // Refresh periodically (every 5s) ONLY when tab is mapped
    let trigger_periodic = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
        trigger_periodic();
        glib::ControlFlow::Continue
    });

    let trigger_initial = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
        trigger_initial();
        glib::ControlFlow::Break
    });

    // Poll adapter power state off the GTK thread so changes made outside
    // Settings are reflected without blocking the window.
    let (tx_status_poll, rx_status_poll) = channel::<bool>();
    let status_in_flight = Rc::new(Cell::new(false));
    let trigger_status_refresh = trigger_refresh.clone();
    let state_status = state.clone();
    let render_status = render_devices.clone();
    let toggle_status = toggle_row.clone();
    let list_box_status = list_box.clone();
    let status_in_flight_rx = status_in_flight.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        while let Ok(enabled) = rx_status_poll.try_recv() {
            status_in_flight_rx.set(false);
            let changed = state_status.borrow().enabled != enabled;
            if !changed {
                continue;
            }
            toggle_status.switch.set_active(enabled);
            toggle_status.set_active(enabled);
            let mut state_ref = state_status.borrow_mut();
            state_ref.enabled = enabled;
            if !enabled {
                state_ref.devices.clear();
                state_ref.is_loading = false;
            }
            drop(state_ref);
            if list_box_status.is_mapped() {
                if enabled {
                    trigger_status_refresh();
                } else {
                    render_status();
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
                let _ = tx.send(is_bluetooth_enabled());
            });
        }
        glib::ControlFlow::Continue
    });

    let trigger_switch = trigger_refresh.clone();
    let state_switch = state.clone();
    let render_switch = render_devices.clone();
    let toggle_row_switch = toggle_row.clone();
    toggle_row.switch.connect_state_set(move |is_active| {
        toggle_row_switch.set_active(is_active);
        std::thread::spawn(move || {
            set_bt_enabled(is_active);
        });
        {
            let mut state_ref = state_switch.borrow_mut();
            state_ref.enabled = is_active;
            if !is_active {
                state_ref.devices.clear();
                state_ref.is_loading = false;
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
