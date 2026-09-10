pub mod popover;
pub mod render;

pub use babydra_core::services::system::wifi::{connect_wifi, scan_networks};

use gtk4::prelude::*;
use tokio::sync::mpsc;

/// Connect wifi async.
pub fn connect_wifi_async(
    ssid: &str,
    username: Option<String>,
    password: Option<String>,
    sub_label: gtk4::Label,
    left_btn: gtk4::Button,
    popover: gtk4::Popover,
) {
    let (tx, mut rx) = mpsc::unbounded_channel::<bool>();

    let ssid_str = ssid.to_string();
    std::thread::spawn(move || {
        let success = connect_wifi(&ssid_str, username.as_deref(), password.as_deref());
        let _ = tx.send(success);
    });

    let sub_label_c = sub_label.clone();
    let left_btn_c = left_btn.clone();
    let popover_c = popover.clone();
    let ssid_str2 = ssid.to_string();

    glib::spawn_future_local(async move {
        if let Some(success) = rx.recv().await {
            if success {
                sub_label_c.set_text(&ssid_str2);
                babydra_ui_kit::components::update_toggle_state(&left_btn_c, true, "wifi");
                popover_c.popdown();
            } else {
                sub_label_c.set_text("Failed");
                popover_c.popdown();
            }
        }
    });
}

