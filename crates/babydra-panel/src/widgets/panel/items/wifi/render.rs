use super::popover::{refresh_wifi_popover_list, setup_wifi_popover};
use super::{connect_wifi_async, get_wifi_state, scan_networks};
use babydra_core::i18n::t;
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
        &babydra_core::i18n::t("control.network"),
        "...",
        "control-tile-left-btn",
        false,
        |_| {},
    );
    left_btn.set_hexpand(false);

    let (tx, mut rx) = mpsc::unbounded_channel::<(bool, String)>();
    std::thread::spawn(move || {
        let state = get_wifi_state();
        let _ = tx.send(state);
    });

    let left_btn_init = left_btn.clone();
    let sub_label_init = sub_label.clone();
    glib::spawn_future_local(async move {
        if let Some((is_act, ssid_str)) = rx.recv().await {
            sub_label_init.set_text(&ssid_str);
            let is_connected = is_act && ssid_str != "Off" && ssid_str != "Disconnected";
            babydra_ui_kit::components::update_toggle_tile_state(
                &left_btn_init,
                is_connected,
                "wifi",
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

    let right_btn = babydra_ui_kit::components::create_colored_icon_button(
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
        "taskbar-popover",
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
    left_btn.connect_clicked(move |b| {
        let is_now_active = b.has_css_class("active");
        if is_now_active {
            babydra_core::helper::wifi::set_wifi_enabled(true);
            sub_label_c.set_text(&t("control.scanning"));
        } else {
            babydra_core::helper::wifi::set_wifi_enabled(false);
            sub_label_c.set_text(&t("control.off"));
        }
    });

    container.append(&left_btn);
    container.append(&right_btn);
    container
}
