//! Wi-Fi configurations control panel.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

struct WifiState {
    enabled: bool,
    networks: Vec<(String, String, String, bool)>,
}

pub fn create_wifi_widget() -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = gtk4::Label::new(Some("Wi-Fi & Mạng"));
    title_lbl.add_css_class("settings-title");
    title_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&title_lbl);

    let wifi_status = babydra_common::helper::wifi::get_wifi_state().unwrap_or(false);
    let state = Rc::new(RefCell::new(WifiState {
        enabled: wifi_status,
        networks: Vec::new(),
    }));

    // Switch Card Row
    let switch_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    switch_card.add_css_class("settings-card");
    switch_card.set_valign(gtk4::Align::Center);

    let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let status_title = gtk4::Label::new(Some("Bật/Tắt Wi-Fi"));
    status_title.add_css_class("settings-label");
    status_title.set_halign(gtk4::Align::Start);
    let status_desc = gtk4::Label::new(Some("Bật hoặc tắt bộ thu phát mạng không dây"));
    status_desc.add_css_class("settings-desc");
    status_desc.set_halign(gtk4::Align::Start);
    label_box.append(&status_title);
    label_box.append(&status_desc);
    switch_card.append(&label_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    switch_card.append(&spacer);

    let wifi_switch = gtk4::Switch::new();
    wifi_switch.set_active(wifi_status);
    wifi_switch.set_valign(gtk4::Align::Center);
    switch_card.append(&wifi_switch);

    main_box.append(&switch_card);

    // List title
    let list_title = gtk4::Label::new(Some("Danh sách mạng khả dụng"));
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

    let render_networks = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            // Clear existing rows
            while let Some(child) = list_box_clone.first_child() {
                list_box_clone.remove(&child);
            }

            let st = state_clone.borrow();
            if !st.enabled {
                let placeholder = gtk4::Label::new(Some("Wi-Fi đang tắt"));
                placeholder.add_css_class("settings-desc");
                placeholder.set_margin_top(20);
                placeholder.set_margin_bottom(20);
                list_box_clone.append(&placeholder);
                return;
            }

            if st.networks.is_empty() {
                let placeholder = gtk4::Label::new(Some("Không tìm thấy mạng nào..."));
                placeholder.add_css_class("settings-desc");
                placeholder.set_margin_top(20);
                placeholder.set_margin_bottom(20);
                list_box_clone.append(&placeholder);
                return;
            }

            for (ssid, security, strength, is_connected) in &st.networks {
                let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
                row.set_margin_top(8);
                row.set_margin_bottom(8);
                row.set_margin_start(8);
                row.set_margin_end(8);

                let icon_name = if *is_connected {
                    "network-wireless-connected-symbolic"
                } else {
                    "network-wireless-signal-excellent-symbolic"
                };
                let wifi_icon = gtk4::Image::from_icon_name(icon_name);
                wifi_icon.set_valign(gtk4::Align::Center);
                row.append(&wifi_icon);

                let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
                let ssid_lbl = gtk4::Label::new(Some(ssid));
                ssid_lbl.add_css_class("settings-label");
                ssid_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&ssid_lbl);

                let sec_text = if security == "open" {
                    format!("Mạng mở • Tín hiệu {}%", strength)
                } else {
                    format!("Bảo mật ({}) • Tín hiệu {}%", security.to_uppercase(), strength)
                };
                let sec_lbl = gtk4::Label::new(Some(&sec_text));
                sec_lbl.add_css_class("settings-desc");
                sec_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&sec_lbl);

                row.append(&text_box);

                let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                spacer.set_hexpand(true);
                row.append(&spacer);

                if *is_connected {
                    let connected_lbl = gtk4::Label::new(Some("Đã kết nối"));
                    connected_lbl.add_css_class("success-text");
                    connected_lbl.set_valign(gtk4::Align::Center);
                    row.append(&connected_lbl);
                } else {
                    let connect_btn = gtk4::Button::with_label("Kết nối");
                    connect_btn.set_valign(gtk4::Align::Center);
                    connect_btn.add_css_class("suggested-action");

                    let ssid_clone = ssid.clone();
                    let security_clone = security.clone();
                    let list_box_parent = list_box_clone.clone();
                    connect_btn.connect_clicked(move |_| {
                        if security_clone == "open" {
                            let _ = babydra_common::helper::wifi::connect_wifi(&ssid_clone, "");
                        } else {
                            // Show password dialog
                            if let Some(win) = list_box_parent.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
                                let dialog = gtk4::Dialog::with_buttons(
                                    Some(&format!("Kết nối đến {}", ssid_clone)),
                                    Some(&win),
                                    gtk4::DialogFlags::MODAL,
                                    &[("Hủy", gtk4::ResponseType::Cancel), ("Kết nối", gtk4::ResponseType::Accept)],
                                );

                                let content_area = dialog.content_area();
                                content_area.set_margin_all(12);
                                content_area.set_spacing(10);

                                let entry_lbl = gtk4::Label::new(Some("Nhập mật khẩu cho mạng này:"));
                                entry_lbl.set_halign(gtk4::Align::Start);
                                content_area.append(&entry_lbl);

                                let pw_entry = gtk4::Entry::new();
                                pw_entry.set_visibility(false);
                                content_area.append(&pw_entry);

                                let ssid_btn = ssid_clone.clone();
                                dialog.connect_response(move |d, res| {
                                    if res == gtk4::ResponseType::Accept {
                                        let password = pw_entry.text().to_string();
                                        let _ = babydra_common::helper::wifi::connect_wifi(&ssid_btn, &password);
                                    }
                                    d.destroy();
                                });
                                dialog.present();
                            }
                        }
                    });
                    row.append(&connect_btn);
                }

                list_box_clone.append(&row);
            }
        }
    };

    let refresh_networks = {
        let state_clone = state.clone();
        let render_clone = render_networks.clone();
        move || {
            let mut st = state_clone.borrow_mut();
            if st.enabled {
                st.networks = babydra_common::helper::wifi::scan_networks();
            } else {
                st.networks.clear();
            }
            render_clone();
        }
    };

    // Initialize list
    let refresh_init = refresh_networks.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        refresh_init();
        glib::ControlFlow::Break
    });

    // Trigger scan on state change or periodically
    let refresh_periodic = refresh_networks.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(6), move || {
        refresh_periodic();
        glib::ControlFlow::Continue
    });

    wifi_switch.connect_state_set(move |_, is_active| {
        let _ = babydra_common::helper::wifi::set_wifi_enabled(is_active);
        let mut st = state.borrow_mut();
        st.enabled = is_active;
        if !is_active {
            st.networks.clear();
        }
        let render_now = render_networks.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
            render_now();
            glib::ControlFlow::Break
        });
        glib::Propagation::Proceed
    });

    main_box
}
