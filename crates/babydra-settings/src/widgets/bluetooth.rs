//! Bluetooth devices management panel.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;

struct BtDevice {
    mac: String,
    name: String,
    connected: bool,
}

struct BluetoothState {
    enabled: bool,
    devices: Vec<BtDevice>,
}

fn is_bluetooth_enabled() -> bool {
    if let Ok(output) = Command::new("bluetoothctl").arg("show").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains("Powered: yes")
    } else {
        false
    }
}

fn set_bluetooth_enabled(enabled: bool) {
    let arg = if enabled { "power on" } else { "power off" };
    let _ = Command::new("sh").arg("-c").arg(&format!("bluetoothctl {}", arg)).output();
}

fn get_bluetooth_devices() -> Vec<BtDevice> {
    let mut devices = Vec::new();
    
    // Get list of known devices
    let output = match Command::new("bluetoothctl").arg("devices").output() {
        Ok(out) => out,
        Err(_) => return devices,
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Device ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let mac = parts[1].to_string();
                let name = parts[2].to_string();
                
                // Check if connected
                let mut connected = false;
                if let Ok(info_out) = Command::new("bluetoothctl").arg("info").arg(&mac).output() {
                    let info_str = String::from_utf8_lossy(&info_out.stdout);
                    connected = info_str.contains("Connected: yes");
                }
                
                devices.push(BtDevice { mac, name, connected });
            }
        }
    }
    
    devices
}

pub fn create_bluetooth_widget() -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = gtk4::Label::new(Some("Bluetooth"));
    title_lbl.add_css_class("settings-title");
    title_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&title_lbl);

    let bt_status = is_bluetooth_enabled();
    let state = Rc::new(RefCell::new(BluetoothState {
        enabled: bt_status,
        devices: Vec::new(),
    }));

    // Toggle switch row
    let switch_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    switch_card.add_css_class("settings-card");
    switch_card.set_valign(gtk4::Align::Center);

    let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let status_title = gtk4::Label::new(Some("Bật/Tắt Bluetooth"));
    status_title.add_css_class("settings-label");
    status_title.set_halign(gtk4::Align::Start);
    let status_desc = gtk4::Label::new(Some("Quản lý kết nối tai nghe, chuột, bàn phím và thiết bị không dây khác"));
    status_desc.add_css_class("settings-desc");
    status_desc.set_halign(gtk4::Align::Start);
    label_box.append(&status_title);
    label_box.append(&status_desc);
    switch_card.append(&label_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    switch_card.append(&spacer);

    let bt_switch = gtk4::Switch::new();
    bt_switch.set_active(bt_status);
    bt_switch.set_valign(gtk4::Align::Center);
    switch_card.append(&bt_switch);

    main_box.append(&switch_card);

    // List title
    let list_title = gtk4::Label::new(Some("Danh sách thiết bị ghép nối"));
    list_title.add_css_class("settings-subtitle");
    list_title.set_halign(gtk4::Align::Start);
    main_box.append(&list_title);

    let list_container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    list_container.add_css_class("settings-card");
    list_container.set_vexpand(true);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    scroll.set_child(Some(&list_box));
    list_container.append(&scroll);

    main_box.append(&list_container);

    let render_devices = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            while let Some(child) = list_box_clone.first_child() {
                list_box_clone.remove(&child);
            }

            let st = state_clone.borrow();
            if !st.enabled {
                let placeholder = gtk4::Label::new(Some("Bluetooth đang tắt"));
                placeholder.add_css_class("settings-desc");
                placeholder.set_margin_top(20);
                placeholder.set_margin_bottom(20);
                list_box_clone.append(&placeholder);
                return;
            }

            if st.devices.is_empty() {
                let placeholder = gtk4::Label::new(Some("Không tìm thấy thiết bị nào đã lưu"));
                placeholder.add_css_class("settings-desc");
                placeholder.set_margin_top(20);
                placeholder.set_margin_bottom(20);
                list_box_clone.append(&placeholder);
                return;
            }

            for dev in &st.devices {
                let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
                row.set_margin_top(8);
                row.set_margin_bottom(8);
                row.set_margin_start(8);
                row.set_margin_end(8);

                let icon = gtk4::Image::from_icon_name("bluetooth-active-symbolic");
                icon.set_valign(gtk4::Align::Center);
                row.append(&icon);

                let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
                let name_lbl = gtk4::Label::new(Some(&dev.name));
                name_lbl.add_css_class("settings-label");
                name_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&name_lbl);

                let mac_lbl = gtk4::Label::new(Some(&dev.mac));
                mac_lbl.add_css_class("settings-desc");
                mac_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&mac_lbl);

                row.append(&text_box);

                let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                spacer.set_hexpand(true);
                row.append(&spacer);

                if dev.connected {
                    let connected_lbl = gtk4::Label::new(Some("Đã kết nối"));
                    connected_lbl.add_css_class("success-text");
                    connected_lbl.set_valign(gtk4::Align::Center);
                    row.append(&connected_lbl);

                    let disconnect_btn = gtk4::Button::with_label("Ngắt");
                    disconnect_btn.set_valign(gtk4::Align::Center);
                    let mac_clone = dev.mac.clone();
                    disconnect_btn.connect_clicked(move |_| {
                        let _ = Command::new("bluetoothctl").arg("disconnect").arg(&mac_clone).output();
                    });
                    row.append(&disconnect_btn);
                } else {
                    let connect_btn = gtk4::Button::with_label("Kết nối");
                    connect_btn.set_valign(gtk4::Align::Center);
                    connect_btn.add_css_class("suggested-action");
                    let mac_clone = dev.mac.clone();
                    connect_btn.connect_clicked(move |_| {
                        let _ = Command::new("bluetoothctl").arg("connect").arg(&mac_clone).output();
                    });
                    row.append(&connect_btn);
                }

                list_box_clone.append(&row);
            }
        }
    };

    let refresh_devices = {
        let state_clone = state.clone();
        let render_clone = render_devices.clone();
        move || {
            let mut st = state_clone.borrow_mut();
            if st.enabled {
                st.devices = get_bluetooth_devices();
            } else {
                st.devices.clear();
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
