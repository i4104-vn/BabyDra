//! Wi-Fi configurations control panel.

pub use babydra_core::models::settings::WifiState;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod events;
mod handler;
mod render;
mod rows;
mod search;
mod sections;

/// Creates a new `wifi widget`.
pub fn create_wifi_widget() -> gtk4::Widget {
    let ui = render::build_wifi_ui();

    let info_dialog = Rc::new(ui.info_dialog);
    let password_dialog = Rc::new(ui.password_dialog);
    let config_dialog = Rc::new(ui.config_dialog);

    let state = Rc::new(RefCell::new(WifiState {
        enabled: false,
        networks: Vec::new(),
        is_loading: true,
        connecting_ssid: None,
        active_network: Default::default(),
    }));

    let (tx_connect_req, rx_connect_req) =
        std::sync::mpsc::channel::<(String, Option<String>, Option<String>)>();

    // Closure to render network list and re-apply active search filter
    let render_networks: Rc<dyn Fn()> = Rc::new({
        let list_box_clone = ui.list_box.clone();
        let state_clone = state.clone();
        let info_dlg_c = info_dialog.clone();
        let pwd_dlg_c = password_dialog.clone();
        let cfg_dlg_c = config_dialog.clone();
        let tx_req_c = tx_connect_req.clone();
        let search_entry_c = ui.search_entry.clone();
        move || {
            let state_ref = state_clone.borrow();
            handler::render_network_list(
                &list_box_clone,
                &state_ref,
                &info_dlg_c,
                &pwd_dlg_c,
                &cfg_dlg_c,
                tx_req_c.clone(),
            );
            let query = search_entry_c.text().to_string();
            if !query.is_empty() {
                search::filter_wifi_list(&list_box_clone, &query);
            }
        }
    });

    // 1. Scanner & periodic discovery
    let trigger_scan = events::setup_scanner(
        state.clone(),
        render_networks.clone(),
        ui.list_box.clone(),
    );

    // 2. Connection manager
    events::setup_connection_manager(
        rx_connect_req,
        state.clone(),
        render_networks.clone(),
        trigger_scan.clone(),
        ui.list_box.clone(),
    );

    // 3. Adapter hardware status synchronization
    events::setup_status_sync(
        state,
        ui.toggle_row,
        render_networks,
        trigger_scan.clone(),
        ui.list_box.clone(),
    );

    // 4. Modal dialogs wiring
    events::wire_dialog_signals(
        &info_dialog,
        &password_dialog,
        &config_dialog,
        tx_connect_req,
        trigger_scan.clone(),
    );

    // 5. Wire search filtering
    let list_box_search = ui.list_box;
    ui.search_entry.connect_changed(move |entry| {
        let q = entry.text().to_string();
        search::filter_wifi_list(&list_box_search, &q);
    });

    // 6. Wire manual refresh button
    ui.refresh_btn.connect_clicked(move |_| {
        trigger_scan();
    });

    ui.root
}
