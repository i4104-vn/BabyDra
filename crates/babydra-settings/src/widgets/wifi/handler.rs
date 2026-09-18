//! Top-level handler coordinating placeholder states and delegating list rendering.

use super::sections;
use babydra_core::models::settings::WifiState;
use babydra_core::models::ActiveNetworkType;
use babydra_ui_kit::components::modals::{WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog};
use std::rc::Rc;
use std::sync::mpsc::Sender;

#[allow(unused_imports)]
pub use super::search::filter_wifi_list;

/// Renders the network list into the list box, managing placeholder states and category sections.
pub fn render_network_list(
    list_box: &gtk4::ListBox,
    state_ref: &WifiState,
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    _config_dialog: &Rc<WifiConfigDialog>,
    tx_connect_req: Sender<(String, Option<String>, Option<String>)>,
) {
    crate::widgets::helpers::clear_list_box(list_box);

    let active_net = state_ref.active_network.clone();
    let is_ethernet_active = active_net.is_connected
        && active_net.network_type == ActiveNetworkType::Ethernet;

    // 1. Wi-Fi Disabled placeholder
    if !state_ref.enabled {
        list_box.append(&crate::widgets::helpers::create_placeholder(
            crate::widgets::helpers::PlaceholderState::Disabled {
                title_key: "settings.wifi_disabled",
                desc_key: "settings.wifi_disabled_sub",
                icon_name: "wifi",
            },
        ));
        return;
    }

    // 2. Loading placeholder
    if state_ref.enabled && state_ref.is_loading && state_ref.networks.is_empty() {
        list_box.append(&crate::widgets::helpers::create_placeholder(
            crate::widgets::helpers::PlaceholderState::Loading,
        ));
        return;
    }

    // 3. No networks available placeholder
    if state_ref.networks.is_empty() && !is_ethernet_active {
        list_box.append(&crate::widgets::helpers::create_placeholder(
            crate::widgets::helpers::PlaceholderState::Empty {
                title_key: "settings.wifi_no_networks",
                desc_key: None,
                icon_name: "wifi",
            },
        ));
        return;
    }

    // 4. Render category sections (Connected, Saved, Available)
    sections::render_all_sections(
        list_box,
        state_ref,
        info_dialog,
        password_dialog,
        &tx_connect_req,
    );
}
