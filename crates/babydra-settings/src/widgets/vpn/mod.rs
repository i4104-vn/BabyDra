//! VPN and WireGuard connections manager.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use babydra_common::services::system::vpn::{
    delete_vpn_connection, get_vpn_connections, import_vpn_profile, save_vpn_connection,
};

mod handler;
mod render;

pub fn create_vpn_widget() -> gtk4::Box {
    let (main_box, _vpn_switch, import_btn, add_custom_btn, list_box, config_dialog) = render::build_vpn_ui();

    let state = Rc::new(RefCell::new(get_vpn_connections()));

    let config_dialog_clone = config_dialog.clone();
    let render_vpns = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        let dialog_c = config_dialog_clone.clone();
        move || {
            let vpns = state_clone.borrow();
            handler::render_vpn_list(&list_box_clone, &vpns, &dialog_c);
        }
    };

    let refresh_vpns = {
        let state_clone = state.clone();
        let render_clone = render_vpns.clone();
        move || {
            *state_clone.borrow_mut() = get_vpn_connections();
            render_clone();
        }
    };

    // Load initial
    let refresh_init = refresh_vpns.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        refresh_init();
        glib::ControlFlow::Break
    });

    // Refresh periodic
    let refresh_periodic = refresh_vpns.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(4), move || {
        refresh_periodic();
        glib::ControlFlow::Continue
    });

    // Handle Add Custom VPN button
    let config_dialog_new = config_dialog.clone();
    add_custom_btn.connect_clicked(move |_| {
        config_dialog_new.show_for_new();
    });

    // Handle Config Dialog Save
    let refresh_on_save = refresh_vpns.clone();
    config_dialog.connect_save(move |details| {
        let _ = save_vpn_connection(&details);
        refresh_on_save();
    });

    // Handle Config Dialog Delete
    let refresh_on_delete = refresh_vpns.clone();
    config_dialog.connect_delete(move |name| {
        let _ = delete_vpn_connection(&name);
        refresh_on_delete();
    });

    // Handle config file import
    let list_box_parent = list_box.clone();
    let refresh_on_import = refresh_vpns.clone();
    import_btn.connect_clicked(move |_| {
        if let Some(win) = list_box_parent.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
            let file_dialog = gtk4::FileDialog::new();
            file_dialog.set_title(&babydra_common::i18n::t("settings.open_vpn_profile"));

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some(&babydra_common::i18n::t("settings.vpn_filter")));
            filter.add_pattern("*.ovpn");
            filter.add_pattern("*.conf");
            file_dialog.set_default_filter(Some(&filter));

            let refresh_cb = refresh_on_import.clone();
            file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let path_str = path.to_string_lossy().to_string();
                        let _ = import_vpn_profile(&path_str);
                        refresh_cb();
                    }
                }
            });
        }
    });

    main_box
}
