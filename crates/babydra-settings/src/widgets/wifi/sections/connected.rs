//! Connected network section (Ethernet and active Wi-Fi).

use super::super::rows::{create_category_header_row, create_ethernet_row, create_wifi_row};
use babydra_core::i18n::trans;
use babydra_core::models::network::ActiveNetworkInfo;
use babydra_core::models::settings::wifi::WifiNetwork;
use babydra_core::models::settings::WifiState;
use babydra_ui_kit::components::modals::{WifiInfoDialog, WifiPasswordDialog};
use std::rc::Rc;
use std::sync::mpsc::Sender;

/// Renders the connected section containing active Ethernet and active Wi-Fi networks.
#[allow(clippy::too_many_arguments)]
pub fn render_connected_section(
    list_box: &gtk4::ListBox,
    is_ethernet_active: bool,
    active_net: &ActiveNetworkInfo,
    connected_wifi: &[WifiNetwork],
    state_ref: &WifiState,
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    tx_connect_req: &Sender<(String, Option<String>, Option<String>)>,
) {
    let total_connected = if is_ethernet_active {
        1 + connected_wifi.len()
    } else {
        connected_wifi.len()
    };

    if total_connected == 0 {
        return;
    }

    // Category header
    let header = create_category_header_row(&trans("settings.wifi_connected"));
    list_box.append(&header);

    // Ethernet card row if active
    if is_ethernet_active {
        let eth_row = create_ethernet_row(active_net);
        list_box.append(&eth_row);
    }

    // Connected Wi-Fi rows
    for net in connected_wifi {
        let row = create_wifi_row(
            net,
            state_ref,
            info_dialog,
            password_dialog,
            tx_connect_req,
        );
        list_box.append(&row);
    }
}
