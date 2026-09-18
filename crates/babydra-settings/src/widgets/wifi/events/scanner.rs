//! Wi-Fi network scanner and periodic polling.

use babydra_core::models::network::ActiveNetworkInfo;
use babydra_core::models::settings::wifi::WifiNetwork;
use babydra_core::models::settings::WifiState;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Sets up background Wi-Fi network scanning, map events, and periodic polling.
pub fn setup_scanner(
    state: Rc<RefCell<WifiState>>,
    render_networks: Rc<dyn Fn()>,
    list_box: gtk4::ListBox,
) -> Rc<dyn Fn()> {
    let (tx_scan, rx_scan) = std::sync::mpsc::channel::<(Vec<WifiNetwork>, ActiveNetworkInfo)>();
    let scan_in_flight = Rc::new(Cell::new(false));

    let list_box_c = list_box.clone();
    let state_c = state.clone();
    let render_c = render_networks.clone();
    let scan_in_flight_c = scan_in_flight.clone();
    let tx_c = tx_scan;

    let trigger_scan: Rc<dyn Fn()> = Rc::new(move || {
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
    });

    // Receive scan results
    let state_rx = state;
    let render_rx = render_networks;
    let lb_rx = list_box.clone();
    let scan_in_flight_rx = scan_in_flight;

    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok((nets, active)) = rx_scan.try_recv() {
            scan_in_flight_rx.set(false);
            let mut state_ref = state_rx.borrow_mut();
            let changed = state_ref.networks != nets
                || state_ref.active_network != active
                || state_ref.is_loading;
            state_ref.networks = nets;
            state_ref.active_network = active;
            state_ref.is_loading = false;
            updated |= changed;
        }
        if updated && lb_rx.is_mapped() {
            render_rx();
        }
        glib::ControlFlow::Continue
    });

    // Trigger on map
    let trigger_map = trigger_scan.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    // Periodic scan (every 6s) when mapped
    let trigger_periodic = trigger_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(6), move || {
        trigger_periodic();
        glib::ControlFlow::Continue
    });

    // Initial scan
    let trigger_initial = trigger_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
        trigger_initial();
        glib::ControlFlow::Break
    });

    trigger_scan
}
