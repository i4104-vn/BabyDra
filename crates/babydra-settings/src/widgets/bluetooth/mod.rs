//! Bluetooth devices management panel.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::channel;

use babydra_common::{is_bluetooth_enabled, set_bluetooth_enabled, get_bluetooth_devices, BtDevice};
use gtk4::prelude::*;

mod handler;
mod render;

pub struct BluetoothState {
    pub enabled: bool,
    pub devices: Vec<BtDevice>,
    pub is_loading: bool,
}

pub fn create_bluetooth_widget() -> gtk4::Widget {
    let (main_box, bt_switch, list_box) = render::build_bluetooth_ui();

    let state = Rc::new(RefCell::new(BluetoothState {
        enabled: false,
        devices: Vec::new(),
        is_loading: true,
    }));

    let bt_switch_c = bt_switch.clone();
    let state_c_init = state.clone();
    let (tx_status, rx_status) = channel::<bool>();
    std::thread::spawn(move || {
        let status = is_bluetooth_enabled();
        let _ = tx_status.send(status);
    });

    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        match rx_status.try_recv() {
            Ok(status) => {
                bt_switch_c.set_active(status);
                state_c_init.borrow_mut().enabled = status;
                glib::ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
        }
    });

    let render_devices = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            let st = state_clone.borrow();
<<<<<<< HEAD
            handler::render_device_list(&list_box_clone, &st);
=======
            if !st.enabled {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
                placeholder_box.set_valign(gtk4::Align::Center);
                placeholder_box.set_halign(gtk4::Align::Center);
                placeholder_box.set_margin_top(30);
                placeholder_box.set_margin_bottom(30);

                let bt_icon = babydra_utils::ui::icon::get_icon("bluetooth", 24);
                bt_icon.set_pixel_size(24);
                bt_icon.add_css_class("settings-row-icon");
                placeholder_box.append(&bt_icon);

                let lbl = gtk4::Label::new(Some("Bluetooth đang tắt"));
                lbl.add_css_class("settings-row-title");
                placeholder_box.append(&lbl);

                let desc = gtk4::Label::new(Some("Bật công tắc phía trên để kết nối các thiết bị ngoại vi"));
                desc.add_css_class("settings-row-desc");
                placeholder_box.append(&desc);

                row.set_child(Some(&placeholder_box));
                list_box_clone.append(&row);
                return;
            }

            if st.devices.is_empty() {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
                placeholder_box.set_valign(gtk4::Align::Center);
                placeholder_box.set_halign(gtk4::Align::Center);
                placeholder_box.set_margin_top(30);
                placeholder_box.set_margin_bottom(30);

                let bt_icon = babydra_utils::ui::icon::get_icon("bluetooth", 24);
                bt_icon.set_pixel_size(24);
                bt_icon.add_css_class("settings-row-icon");
                placeholder_box.append(&bt_icon);

                let lbl = gtk4::Label::new(Some("Không tìm thấy thiết bị nào đã lưu"));
                lbl.add_css_class("settings-row-title");
                placeholder_box.append(&lbl);

                row.set_child(Some(&placeholder_box));
                list_box_clone.append(&row);
                return;
            }

            for dev in &st.devices {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
                hbox.set_margin_top(10);
                hbox.set_margin_bottom(10);
                hbox.set_margin_start(16);
                hbox.set_margin_end(16);

                let icon = babydra_utils::ui::icon::get_icon("bluetooth", 16);
                icon.set_pixel_size(16);
                icon.set_valign(gtk4::Align::Center);
                icon.add_css_class("settings-row-icon");
                hbox.append(&icon);

                let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
                text_box.set_valign(gtk4::Align::Center);
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
                    let connected_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
                    connected_badge.add_css_class("connected-pill");
                    connected_badge.set_valign(gtk4::Align::Center);

                    let check_icon = babydra_utils::ui::icon::get_icon("check", 14);
                    check_icon.set_pixel_size(14);
                    connected_badge.append(&check_icon);

                    let connected_lbl = gtk4::Label::new(Some("Đã kết nối"));
                    connected_lbl.add_css_class("connected-text");
                    connected_badge.append(&connected_lbl);

                    hbox.append(&connected_badge);

                    let disconnect_btn = gtk4::Button::with_label("Ngắt");
                    disconnect_btn.set_valign(gtk4::Align::Center);
                    disconnect_btn.add_css_class("connect-pill-btn");
                    let mac_clone = dev.mac.clone();
                    disconnect_btn.connect_clicked(move |_| {
                        let _ = Command::new("bluetoothctl").arg("disconnect").arg(&mac_clone).output();
                    });
                    hbox.append(&disconnect_btn);
                } else {
                    let connect_btn = gtk4::Button::with_label("Kết nối");
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
>>>>>>> hard-develop
        }
    };

    // Async thread scanning channel
    let (tx_devs, rx_devs) = channel::<Vec<BtDevice>>();

    let trigger_refresh = {
        let tx_c = tx_devs.clone();
        let state_c = state.clone();
        let list_box_c = list_box.clone();
        let render_c = render_devices.clone();
        move || {
            // ONLY fetch when tab is active/mapped!
            if !list_box_c.is_mapped() {
                return;
            }

            let (enabled, is_empty) = {
                let st = state_c.borrow();
                (st.enabled, st.devices.is_empty())
            };

            if enabled {
                if is_empty {
                    state_c.borrow_mut().is_loading = true;
                    render_c();
                }
                let tx_sub = tx_c.clone();
                std::thread::spawn(move || {
                    let devs = get_bluetooth_devices();
                    let _ = tx_sub.send(devs);
                });
            }
        }
    };

    let state_c_rx = state.clone();
    let render_c_rx = render_devices.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok(devs) = rx_devs.try_recv() {
            let mut st = state_c_rx.borrow_mut();
            st.devices = devs;
            st.is_loading = false;
            updated = true;
        }
        if updated {
            render_c_rx();
        }
        glib::ControlFlow::Continue
    });

    // Trigger fetch instantly when tab becomes active/mapped
    let trigger_map = trigger_refresh.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    // Trigger initial fetch if mapped
    let trigger_init = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        trigger_init();
        glib::ControlFlow::Break
    });

    // Refresh periodically (every 5s) ONLY when tab is mapped
    let trigger_periodic = trigger_refresh.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
        trigger_periodic();
        glib::ControlFlow::Continue
    });

    let trigger_switch = trigger_refresh.clone();
    let state_switch = state.clone();
    let render_switch = render_devices.clone();
    bt_switch.connect_state_set(move |_, is_active| {
        set_bluetooth_enabled(is_active);
        {
            let mut st = state_switch.borrow_mut();
            st.enabled = is_active;
            if !is_active {
                st.devices.clear();
                st.is_loading = false;
            }
        }
        if !is_active {
            render_switch();
        } else {
            trigger_switch();
        }
        glib::Propagation::Proceed
    });

    main_box.into()
}
