pub mod render;

pub use babydra_common::helper::wifi::{
    strip_ansi_escapes, get_wifi_state, known_networks, scan_networks, connect_wifi,
};

use gtk4::prelude::*;
use tokio::sync::mpsc;

pub fn connect_wifi_async(
    ssid: &str,
    username: Option<String>,
    password: Option<String>,
    sub_label: gtk4::Label,
    left_btn: gtk4::Button,
    circle: gtk4::Box,
    icon_widget: gtk4::Image,
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
    let circle_c = circle.clone();
    let icon_widget_c = icon_widget.clone();
    let popover_c = popover.clone();
    let ssid_str2 = ssid.to_string();

    glib::spawn_future_local(async move {
        if let Some(success) = rx.recv().await {
            if success {
                sub_label_c.set_text(&ssid_str2);
                left_btn_c.add_css_class("active");
                circle_c.add_css_class("active");
                let new_img = babydra_common::icon::get_icon_colored("wifi", 14, "#ffffff");
                if let Some(paintable) = new_img.paintable() {
                    icon_widget_c.set_paintable(Some(&paintable));
                }
                popover_c.popdown();
            } else {
                sub_label_c.set_text("Failed");
                popover_c.popdown();
            }
        }
    });
}
