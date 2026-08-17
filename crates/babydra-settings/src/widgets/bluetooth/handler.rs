//! Bluetooth device list renderer and event handlers.

use super::BluetoothState;
use babydra_core::services::system::bluetooth::{connect_device, disconnect_device};
use gtk4::prelude::*;

/// Renders `device list`.
pub fn render_device_list(list_box: &gtk4::ListBox, st: &BluetoothState) {
    crate::widgets::helpers::clear_list_box(list_box);

    if !st.enabled {
        list_box.append(&crate::widgets::helpers::create_placeholder_row(
            crate::widgets::helpers::PlaceholderState::Disabled {
                title_key: "settings.bt_off",
                desc_key: "settings.bt_off_sub",
                icon_name: "bluetooth",
            },
        ));
        return;
    }

    if st.enabled && st.is_loading && st.devices.is_empty() {
        list_box.append(&crate::widgets::helpers::create_placeholder_row(
            crate::widgets::helpers::PlaceholderState::Loading,
        ));
        return;
    }

    if st.devices.is_empty() {
        list_box.append(&crate::widgets::helpers::create_placeholder_row(
            crate::widgets::helpers::PlaceholderState::Empty {
                title_key: "settings.bt_no_devices",
                desc_key: Some("settings.bt_no_devices_sub"),
                icon_name: "bluetooth",
            },
        ));
        return;
    }

    for dev in &st.devices {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
        hbox.set_margin_top(4);
        hbox.set_margin_bottom(4);
        hbox.set_margin_start(8);
        hbox.set_margin_end(8);

        let icon_badge = crate::widgets::helpers::create_icon_badge("bluetooth", 18, true);
        hbox.append(&icon_badge);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        text_box.set_valign(gtk4::Align::Center);
        text_box.set_halign(gtk4::Align::Start);
        text_box.set_hexpand(true);

        let name_lbl = gtk4::Label::new(Some(&dev.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&name_lbl);

        let mac_lbl = gtk4::Label::new(Some(&dev.mac));
        mac_lbl.add_css_class("settings-row-desc");
        mac_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&mac_lbl);

        hbox.append(&text_box);

        if dev.connected {
            let check_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            check_badge.add_css_class("active-check-badge");
            check_badge.set_valign(gtk4::Align::Center);

            let check_icon = babydra_ui_kit::ui::icon::get_icon("check", 14);
            check_icon.set_pixel_size(14);
            check_badge.append(&check_icon);
            hbox.append(&check_badge);

            let disconnect_btn =
                gtk4::Button::with_label(&babydra_core::i18n::t("settings.disconnect"));
            disconnect_btn.set_valign(gtk4::Align::Center);
            disconnect_btn.add_css_class("connect-pill-btn");
            let mac_clone = dev.mac.clone();
            disconnect_btn.connect_clicked(move |_| {
                let mac = mac_clone.clone();
                std::thread::spawn(move || {
                    let _ = disconnect_device(&mac);
                });
            });
            hbox.append(&disconnect_btn);
        } else {
            let connect_btn = gtk4::Button::with_label(&babydra_core::i18n::t("settings.connect"));
            connect_btn.set_valign(gtk4::Align::Center);
            connect_btn.add_css_class("suggested-action");
            let mac_clone = dev.mac.clone();
            connect_btn.connect_clicked(move |_| {
                let mac = mac_clone.clone();
                std::thread::spawn(move || {
                    let _ = connect_device(&mac);
                });
            });
            hbox.append(&connect_btn);
        }

        row.set_child(Some(&hbox));
        list_box.append(&row);
    }
}
