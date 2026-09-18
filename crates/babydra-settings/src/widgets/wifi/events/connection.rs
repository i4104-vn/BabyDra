//! Wi-Fi connection worker and request listener.

use babydra_core::models::settings::WifiState;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver};

/// Sets up background Wi-Fi connection request processing and completion listener.
pub fn setup_connection_manager(
    rx_req: Receiver<(String, Option<String>, Option<String>)>,
    state: Rc<RefCell<WifiState>>,
    render_networks: Rc<dyn Fn()>,
    trigger_scan: Rc<dyn Fn()>,
    list_box: gtk4::ListBox,
) {
    let (tx_done, rx_done) = channel::<()>();

    let state_req = state.clone();
    let render_req = render_networks.clone();
    let tx_done_c = tx_done.clone();
    let lb_req = list_box.clone();

    // Listen for connection requests
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if !lb_req.is_mapped() {
            return glib::ControlFlow::Continue;
        }
        while let Ok((ssid, user, pwd)) = rx_req.try_recv() {
            if state_req.borrow().connecting_ssid.is_some() {
                continue;
            }
            state_req.borrow_mut().connecting_ssid = Some(ssid.clone());
            render_req();

            let tx = tx_done_c.clone();
            std::thread::spawn(move || {
                let _ = babydra_core::services::system::wifi::connect_wifi(
                    &ssid,
                    user.as_deref(),
                    pwd.as_deref(),
                );
                let _ = tx.send(());
            });
        }
        glib::ControlFlow::Continue
    });

    // Listen for connection completions
    let state_done = state;
    let render_done = render_networks;
    let trigger_done = trigger_scan;
    let lb_done = list_box;

    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        if !lb_done.is_mapped() {
            return glib::ControlFlow::Continue;
        }
        while rx_done.try_recv().is_ok() {
            state_done.borrow_mut().connecting_ssid = None;
            render_done();
            trigger_done();
        }
        glib::ControlFlow::Continue
    });
}
