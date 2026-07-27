//! Bluetooth devices management panel.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;

use babydra_common::{is_bluetooth_enabled, set_bluetooth_enabled, get_bluetooth_devices, BtDevice};

mod render;

struct BluetoothState {
    enabled: bool,
    devices: Vec<BtDevice>,
}

pub fn create_bluetooth_widget() -> gtk4::Box {
    let (main_box, bt_switch, list_box) = render::build_bluetooth_ui();

    let bt_status = is_bluetooth_enabled();
    let state = Rc::new(RefCell::new(BluetoothState {
        enabled: bt_status,
        devices: Vec::new(),
    }));

    bt_switch.set_active(bt_status);

    let render_devices = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            while let Some(child) = list_box_clone.first_child() {
                list_box_clone.remove(&child);
            }

            let st = state_clone.borrow();
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
                list_box_clone.append(&row);
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
                list_box_clone.append(&row);
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

                // Blue Rounded Square Badge with Bluetooth Icon (Matching Wi-Fi)
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

                // Device Name + MAC (Aligned Left)
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
                    // Connected Checkmark Badge (Matching Wi-Fi)
                    let check_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                    check_badge.add_css_class("active-check-badge");
                    check_badge.set_valign(gtk4::Align::Center);

                    let check_icon = babydra_utils::ui::icon::get_icon("check", 14);
                    check_icon.set_pixel_size(14);
                    check_badge.append(&check_icon);
                    hbox.append(&check_badge);

                    let disconnect_btn = gtk4::Button::with_label("Disconnect");
                    disconnect_btn.set_valign(gtk4::Align::Center);
                    disconnect_btn.add_css_class("connect-pill-btn");
                    let mac_clone = dev.mac.clone();
                    disconnect_btn.connect_clicked(move |_| {
                        let _ = Command::new("bluetoothctl").arg("disconnect").arg(&mac_clone).output();
                    });
                    hbox.append(&disconnect_btn);
                } else {
                    let connect_btn = gtk4::Button::with_label("Connect");
                    connect_btn.set_valign(gtk4::Align::Center);
                    connect_btn.add_css_class("connect-pill-btn");
                    let mac_clone = dev.mac.clone();
                    connect_btn.connect_clicked(move |_| {
                        let _ = Command::new("bluetoothctl").arg("connect").arg(&mac_clone).output();
                    });
                    hbox.append(&connect_btn);
                }

                row.set_child(Some(&hbox));
                list_box_clone.append(&row);
            }
        }
    };

    let refresh_devices = {
        let state_clone = state.clone();
        let render_clone = render_devices.clone();
        move || {
            {
                let mut st = state_clone.borrow_mut();
                if st.enabled {
                    st.devices = get_bluetooth_devices();
                } else {
                    st.devices.clear();
                }
            }
            render_clone();
        }
    };

    // Load initial list
    let refresh_init = refresh_devices.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        refresh_init();
        glib::ControlFlow::Break
    });

    // Refresh periodically
    let refresh_periodic = refresh_devices.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
        refresh_periodic();
        glib::ControlFlow::Continue
    });

    bt_switch.connect_state_set(move |_, is_active| {
        set_bluetooth_enabled(is_active);
        let mut st = state.borrow_mut();
        st.enabled = is_active;
        if !is_active {
            st.devices.clear();
        }
        let render_now = render_devices.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
            render_now();
            glib::ControlFlow::Break
        });
        glib::Propagation::Proceed
    });

    main_box
}
