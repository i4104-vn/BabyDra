//! Bluetooth device list renderer and event handlers.

use gtk4::prelude::*;
use babydra_common::services::system::bluetooth::{connect_device, disconnect_device};
use super::BluetoothState;

pub fn render_device_list(list_box: &gtk4::ListBox, st: &BluetoothState) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    if !st.enabled {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");
        row.set_selectable(false);
        row.set_activatable(false);
        row.set_vexpand(true);
        row.set_valign(gtk4::Align::Fill);

        let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 14);
        placeholder_box.set_valign(gtk4::Align::Center);
        placeholder_box.set_halign(gtk4::Align::Center);
        placeholder_box.set_vexpand(true);
        placeholder_box.set_hexpand(true);
        placeholder_box.set_margin_top(48);
        placeholder_box.set_margin_bottom(48);

        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Center);

        let bt_icon = babydra_utils::ui::icon::get_icon("bluetooth", 24);
        bt_icon.set_pixel_size(24);
        bt_icon.set_valign(gtk4::Align::Center);
        bt_icon.set_halign(gtk4::Align::Center);
        bt_icon.set_vexpand(true);
        icon_badge.append(&bt_icon);
        placeholder_box.append(&icon_badge);

        let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.bt_off")));
        lbl.add_css_class("settings-row-title");
        placeholder_box.append(&lbl);

        let desc = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.bt_off_sub")));
        desc.add_css_class("settings-row-desc");
        placeholder_box.append(&desc);

        row.set_child(Some(&placeholder_box));
        list_box.append(&row);
        return;
    }

    if st.devices.is_empty() {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");
        row.set_selectable(false);
        row.set_activatable(false);
        row.set_vexpand(true);
        row.set_valign(gtk4::Align::Fill);

        let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 14);
        placeholder_box.set_valign(gtk4::Align::Center);
        placeholder_box.set_halign(gtk4::Align::Center);
        placeholder_box.set_vexpand(true);
        placeholder_box.set_hexpand(true);
        placeholder_box.set_margin_top(48);
        placeholder_box.set_margin_bottom(48);

        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Center);

        let bt_icon = babydra_utils::ui::icon::get_icon("bluetooth", 24);
        bt_icon.set_pixel_size(24);
        bt_icon.set_valign(gtk4::Align::Center);
        bt_icon.set_halign(gtk4::Align::Center);
        bt_icon.set_vexpand(true);
        icon_badge.append(&bt_icon);
        placeholder_box.append(&icon_badge);

        let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.bt_no_devices")));
        lbl.add_css_class("settings-row-title");
        placeholder_box.append(&lbl);

        let desc = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.bt_no_devices_sub")));
        desc.add_css_class("settings-row-desc");
        placeholder_box.append(&desc);

        row.set_child(Some(&placeholder_box));
        list_box.append(&row);
        return;
    }

    for dev in &st.devices {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
        hbox.set_margin_top(12);
        hbox.set_margin_bottom(12);
        hbox.set_margin_start(16);
        hbox.set_margin_end(16);

        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Start);

        let icon = babydra_utils::ui::icon::get_icon("bluetooth", 18);
        icon.set_pixel_size(18);
        icon.set_valign(gtk4::Align::Center);
        icon.set_halign(gtk4::Align::Center);
        icon.set_vexpand(true);
        icon_badge.append(&icon);
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

            let check_icon = babydra_utils::ui::icon::get_icon("check", 14);
            check_icon.set_pixel_size(14);
            check_badge.append(&check_icon);
            hbox.append(&check_badge);

            let disconnect_btn = gtk4::Button::with_label(&babydra_common::i18n::t("settings.disconnect"));
            disconnect_btn.set_valign(gtk4::Align::Center);
            disconnect_btn.add_css_class("connect-pill-btn");
            let mac_clone = dev.mac.clone();
            disconnect_btn.connect_clicked(move |_| {
                let _ = disconnect_device(&mac_clone);
            });
            hbox.append(&disconnect_btn);
        } else {
            let connect_btn = gtk4::Button::with_label(&babydra_common::i18n::t("settings.connect"));
            connect_btn.set_valign(gtk4::Align::Center);
            connect_btn.add_css_class("connect-pill-btn");
            let mac_clone = dev.mac.clone();
            connect_btn.connect_clicked(move |_| {
                let _ = connect_device(&mac_clone);
            });
            hbox.append(&connect_btn);
        }

        row.set_child(Some(&hbox));
        list_box.append(&row);
    }
}
