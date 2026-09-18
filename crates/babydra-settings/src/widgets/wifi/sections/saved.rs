//! Saved networks section.

use super::super::rows::{create_category_header_row, create_wifi_row};
use babydra_core::i18n::trans;
use babydra_core::models::settings::wifi::WifiNetwork;
use babydra_core::models::settings::WifiState;
use babydra_ui_kit::components::modals::{WifiInfoDialog, WifiPasswordDialog};
use std::rc::Rc;
use std::sync::mpsc::Sender;

/// Renders the saved networks section.
pub fn render_saved_section(
    list_box: &gtk4::ListBox,
    saved_wifi: &[WifiNetwork],
    state_ref: &WifiState,
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    tx_connect_req: &Sender<(String, Option<String>, Option<String>)>,
) {
    if saved_wifi.is_empty() {
        return;
    }

    let header = create_category_header_row(&trans("settings.wifi_saved"));
    list_box.append(&header);

    for net in saved_wifi {
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
