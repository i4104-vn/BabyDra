//! VPN and WireGuard connections manager.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::mpsc::channel;

use babydra_core::services::system::vpn::{
    delete_vpn, get_vpn_connections, save_vpn_connection, VpnConn,
};

mod handler;
mod render;

/// Creates a new `VPN widget`.
pub fn create_vpn_widget() -> gtk4::Widget {
    let (main_box, add_custom_btn, flow_box, stack, config_dialog, log_dialog) =
        render::build_vpn_ui();

    let state = Rc::new(RefCell::new(Vec::<VpnConn>::new()));
    let connecting_vpns = Rc::new(RefCell::new(HashSet::<String>::new()));
    let is_loading = Rc::new(RefCell::new(true));
    let (tx, rx) = channel::<Vec<VpnConn>>();
    let (tx_action, rx_action) = channel::<(String, bool)>();

    let trigger_refresh = {
        let tx_c = tx.clone();
        move || {
            let tx_sub = tx_c.clone();
            std::thread::spawn(move || {
                let vpns = get_vpn_connections();
                let _ = tx_sub.send(vpns);
            });
        }
    };

    // Receive data from background thread and render on GTK main thread
    let state_c = state.clone();
    let connecting_vpns_c = connecting_vpns.clone();
    let is_loading_c = is_loading.clone();
    let flow_box_c = flow_box.clone();
    let stack_c = stack.clone();
    let config_dialog_c = config_dialog.clone();
    let log_dialog_c = log_dialog.clone();
    let trigger_ref_c = trigger_refresh.clone();
    let tx_action_c = tx_action.clone();
    let main_box_c = main_box.clone();

    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok((name, is_connecting)) = rx_action.try_recv() {
            if is_connecting {
                connecting_vpns_c.borrow_mut().insert(name);
            } else {
                connecting_vpns_c.borrow_mut().remove(&name);
            }
            updated = true;
        }
        while let Ok(vpns) = rx.try_recv() {
            *state_c.borrow_mut() = vpns;
            *is_loading_c.borrow_mut() = false;
            updated = true;
        }
        if updated {
            if let Some(root) = main_box_c.root() {
                let _ = root.activate_action("win.refresh-sidebar", None);
            }
            handler::render_vpn_list(
                &flow_box_c,
                &stack_c,
                &state_c.borrow(),
                *is_loading_c.borrow(),
                &connecting_vpns_c,
                &tx_action_c,
                &config_dialog_c,
                &log_dialog_c,
                trigger_ref_c.clone(),
            );
        }
        glib::ControlFlow::Continue
    });

    // Trigger initial fetch immediately
    trigger_refresh();

    // Trigger fetch instantly when tab becomes active/mapped
    let trigger_map = trigger_refresh.clone();
    main_box.connect_map(move |_| {
        trigger_map();
    });

    // Periodic refresh (every 4s) ONLY when tab is mapped
    let main_box_periodic = main_box.clone();
    let trigger_periodic = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(4), move || {
        if main_box_periodic.is_mapped() {
            trigger_periodic();
        }
        glib::ControlFlow::Continue
    });

    // Handle Add Custom VPN button
    let config_dialog_new = config_dialog.clone();
    add_custom_btn.connect_clicked(move |_| {
        config_dialog_new.show_for_new();
    });

    // Handle Config Dialog Save
    let trigger_on_save = trigger_refresh.clone();
    config_dialog.connect_save(move |details| {
        let trigger_cb = trigger_on_save.clone();
        std::thread::spawn(move || {
            let _ = save_vpn_connection(&details);
            trigger_cb();
        });
    });

    // Handle Config Dialog Delete
    let trigger_on_delete = trigger_refresh.clone();
    config_dialog.connect_delete(move |name| {
        let trigger_cb = trigger_on_delete.clone();
        std::thread::spawn(move || {
            let _ = delete_vpn(&name);
            trigger_cb();
        });
    });

    main_box.into()
}
