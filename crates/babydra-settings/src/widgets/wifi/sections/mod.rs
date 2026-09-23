//! Network list sections coordinator.

pub mod available;
pub mod connected;
pub mod saved;

use babydra_core::models::settings::WifiState;
use babydra_core::models::ActiveNetworkType;
use babydra_ui_kit::components::modals::{WifiInfoDialog, WifiPasswordDialog};
use std::rc::Rc;
use std::sync::mpsc::Sender;

/// Partitions and renders all network sections: Connected, Saved, and Available.
pub fn render_all_sections(
    list_box: &gtk4::ListBox,
    state_ref: &WifiState,
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    tx_connect_req: &Sender<(String, Option<String>, Option<String>)>,
) {
    let active_net = state_ref.active_network.clone();
    let is_ethernet_active = active_net.is_connected
        && active_net.network_type == ActiveNetworkType::Ethernet;

    // Partition networks into: Connected, Saved, Available
    let mut connected_wifi = Vec::new();
    let mut saved_wifi = Vec::new();
    let mut available_wifi = Vec::new();

    // Sort networks by signal strength descending
    let mut sorted_nets = state_ref.networks.clone();
    sorted_nets.sort_by_key(|a| std::cmp::Reverse(a.signal));

    for net in sorted_nets {
        if net.is_connected || state_ref.connecting_ssid.as_ref() == Some(&net.ssid) {
            connected_wifi.push(net);
        } else if net.is_saved {
            saved_wifi.push(net);
        } else {
            available_wifi.push(net);
        }
    }

    // 1. Connected Section
    connected::render_connected_section(
        list_box,
        is_ethernet_active,
        &active_net,
        &connected_wifi,
        state_ref,
        info_dialog,
        password_dialog,
        tx_connect_req,
    );

    // 2. Saved Networks Section
    saved::render_saved_section(
        list_box,
        &saved_wifi,
        state_ref,
        info_dialog,
        password_dialog,
        tx_connect_req,
    );

    // 3. Available Networks Section
    available::render_available_section(
        list_box,
        &available_wifi,
        state_ref,
        info_dialog,
        password_dialog,
        tx_connect_req,
    );
}
