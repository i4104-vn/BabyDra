//! Wi-Fi adapter hardware status polling and switch synchronization.

use babydra_core::models::settings::WifiState;
use babydra_ui_kit::components::ToggleRow;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Sets up initial status check, periodic hardware status polling, and switch toggle wiring.
pub fn setup_status_sync(
    state: Rc<RefCell<WifiState>>,
    toggle_row: ToggleRow,
    render_networks: Rc<dyn Fn()>,
    trigger_scan: Rc<dyn Fn()>,
    list_box: gtk4::ListBox,
) {
    // Initial status fetch
    let toggle_init = toggle_row.clone();
    let state_init = state.clone();
    crate::widgets::helpers::spawn_async_task(
        || babydra_core::services::system::wifi::get_wifi_state().0,
        move |status| {
            toggle_init.switch.set_active(status);
            toggle_init.set_active(status);
            state_init.borrow_mut().enabled = status;
        },
        50,
    );

    // Periodic polling for external hardware changes (rfkill, command-line)
    let (tx_status_poll, rx_status_poll) = std::sync::mpsc::channel::<bool>();
    let status_in_flight = Rc::new(Cell::new(false));

    let trigger_status_scan = trigger_scan.clone();
    let state_status_poll = state.clone();
    let render_status_poll = render_networks.clone();
    let toggle_status_poll = toggle_row.clone();
    let lb_status_poll = list_box.clone();
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
            if lb_status_poll.is_mapped() {
                if enabled {
                    trigger_status_scan();
                } else {
                    render_status_poll();
                }
            }
        }
        glib::ControlFlow::Continue
    });

    let tx_status_trigger = tx_status_poll;
    let status_in_flight_trigger = status_in_flight;
    let lb_status_trigger = list_box;
    glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
        if lb_status_trigger.is_mapped() && !status_in_flight_trigger.get() {
            status_in_flight_trigger.set(true);
            let tx = tx_status_trigger.clone();
            std::thread::spawn(move || {
                let _ = tx.send(babydra_core::services::system::wifi::get_wifi_state().0);
            });
        }
        glib::ControlFlow::Continue
    });

    // Toggle switch user interaction
    let trigger_switch = trigger_scan;
    let state_switch = state;
    let render_switch = render_networks;
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
}
