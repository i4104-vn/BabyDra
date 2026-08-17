//! VPN and WireGuard connections manager.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::mpsc::channel;

use babydra_core::services::system::vpn::{
    delete_vpn_connection, get_vpn_connections, import_vpn_profile, save_vpn_connection, VpnConn,
};

mod handler;
mod render;

/// Creates a new `VPN widget`.
pub fn create_vpn_widget() -> gtk4::Widget {
    let (main_box, import_btn, add_custom_btn, list_box, config_dialog, log_dialog) =
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
    let list_box_c = list_box.clone();
    let config_dialog_c = config_dialog.clone();
    let log_dialog_c = log_dialog.clone();
    let trigger_ref_c = trigger_refresh.clone();
    let tx_action_c = tx_action.clone();

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
            if let Some(root) = list_box_c.root() {
                let _ = root.activate_action("win.refresh-sidebar", None);
            }
            handler::render_vpn_list(
                &list_box_c,
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

    // Trigger fetch instantly when tab becomes active/mapped
    let trigger_map = trigger_refresh.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    // Initial fetch ONLY if mapped
    let list_box_init = list_box.clone();
    let trigger_init = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        if list_box_init.is_mapped() {
            trigger_init();
        }
        glib::ControlFlow::Break
    });

    // Periodic refresh (every 4s) ONLY when tab is mapped
    let list_box_periodic = list_box.clone();
    let trigger_periodic = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(4), move || {
        if list_box_periodic.is_mapped() {
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
            let _ = delete_vpn_connection(&name);
            trigger_cb();
        });
    });

    // Handle config file import
    let list_box_parent = list_box.clone();
    let trigger_on_import = trigger_refresh.clone();
    import_btn.connect_clicked(move |_| {
        if let Some(win) = list_box_parent
            .root()
            .and_then(|r| r.downcast::<gtk4::Window>().ok())
        {
            let file_dialog = gtk4::FileDialog::new();
            file_dialog.set_title(&babydra_core::i18n::t("settings.open_vpn_profile"));

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some(&babydra_core::i18n::t("settings.vpn_filter")));
            filter.add_pattern("*.ovpn");
            filter.add_pattern("*.conf");
            file_dialog.set_default_filter(Some(&filter));

            let trigger_cb = trigger_on_import.clone();
            file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let path_str = path.to_string_lossy().to_string();
                        let trigger_cb2 = trigger_cb.clone();
                        std::thread::spawn(move || {
                            let _ = import_vpn_profile(&path_str);
                            trigger_cb2();
                        });
                    }
                }
            });
        }
    });

    main_box.into()
}
