use super::popover::setup_wifi_popover;
use babydra_core::models::{ActiveNetworkInfo, ActiveNetworkType};
use gtk4::prelude::*;
use std::rc::Rc;
use tokio::sync::mpsc;

/// Creates a new `wifi tile`.
pub fn create_wifi_tile(on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    container.add_css_class("control-tile-container");
    container.set_hexpand(false);

    let (left_btn, sub_label) = babydra_ui_kit::components::create_toggle_tile(
        "wifi",
        &babydra_core::i18n::trans("control.network"),
        "...",
        "control-tile-left-btn",
        false,
        |_| {},
    );
    left_btn.set_hexpand(false);

    let (tx, mut rx) = mpsc::unbounded_channel::<ActiveNetworkInfo>();
    std::thread::spawn(move || {
        let info = babydra_core::services::system::network::get_active_network_info();
        let _ = tx.send(info);
    });

    let left_btn_init = left_btn.clone();
    let sub_label_init = sub_label.clone();
    glib::spawn_future_local(async move {
        if let Some(info) = rx.recv().await {
            sub_label_init.set_text(&info.name);
            babydra_ui_kit::components::update_toggle_state(
                &left_btn_init,
                info.is_connected,
                &info.icon_name,
            );
        }
    });

    let circle = left_btn
        .child()
        .and_then(|w| w.downcast::<gtk4::Box>().ok())
        .and_then(|main_box| main_box.first_child())
        .and_then(|c| c.downcast::<gtk4::Box>().ok())
        .unwrap();

    let icon_widget = circle
        .first_child()
        .and_then(|img| img.downcast::<gtk4::Image>().ok())
        .unwrap();

    let right_btn = babydra_ui_kit::components::create_color_btn(
        "go-next-symbolic",
        12,
        "rgba(255, 255, 255, 0.7)",
        &["control-tile-right-btn"],
        None,
        || {},
    );

    let popover = babydra_ui_kit::components::create_popover(
        &container,
        gtk4::PositionType::Right,
        "wifi-popover control-popover",
    );
    popover.set_has_arrow(false);

    setup_wifi_popover(
        &popover,
        sub_label.clone(),
        left_btn.clone(),
        circle.clone(),
        icon_widget.clone(),
    );

    let on_popover_toggled_c = on_popover_toggled.clone();
    let popover_c1 = popover.clone();
    let right_btn_clone = right_btn.clone();
    right_btn.connect_clicked(move |_| {
        popover_c1.popup();
        if let Some(ref cb) = on_popover_toggled_c {
            cb(true);
        }
        let left_icon = babydra_ui_kit::ui::icon::get_icon_colored(
            "go-previous-symbolic",
            12,
            "rgba(255, 255, 255, 0.7)",
        );
        right_btn_clone.set_child(Some(&left_icon));
    });

    let right_btn_c2 = right_btn.clone();
    let on_popover_toggled_c2 = on_popover_toggled.clone();
    popover.connect_closed(move |_| {
        if let Some(ref cb) = on_popover_toggled_c2 {
            cb(false);
        }
        let right_icon = babydra_ui_kit::ui::icon::get_icon_colored(
            "go-next-symbolic",
            12,
            "rgba(255, 255, 255, 0.7)",
        );
        right_btn_c2.set_child(Some(&right_icon));
    });

    let sub_label_c = sub_label.clone();
    let left_btn_c = left_btn.clone();
    left_btn.connect_clicked(move |b| {
        let is_now_active = b.has_css_class("active");
        let sub_label_t = sub_label_c.clone();
        let left_btn_t = left_btn_c.clone();

        let (tx_refresh, mut rx_refresh) = mpsc::unbounded_channel::<ActiveNetworkInfo>();
        std::thread::spawn(move || {
            let active_net = babydra_core::services::system::network::get_active_network_info();
            if active_net.network_type == ActiveNetworkType::Ethernet {
                let _ = tx_refresh.send(active_net);
            } else {
                if is_now_active {
                    babydra_core::services::system::wifi::set_wifi_enabled(true);
                } else {
                    babydra_core::services::system::wifi::set_wifi_enabled(false);
                }
                std::thread::sleep(std::time::Duration::from_millis(300));
                let new_info = babydra_core::services::system::network::get_active_network_info();
                let _ = tx_refresh.send(new_info);
            }
        });

        glib::spawn_future_local(async move {
            if let Some(info) = rx_refresh.recv().await {
                sub_label_t.set_text(&info.name);
                babydra_ui_kit::components::update_toggle_state(
                    &left_btn_t,
                    info.is_connected,
                    &info.icon_name,
                );
            }
        });
    });

    container.append(&left_btn);
    container.append(&right_btn);
    container
}
