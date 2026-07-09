//! Wi-Fi configurations control panel.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod render;

struct WifiState {
    enabled: bool,
    networks: Vec<(String, String, String, bool)>,
}

pub fn create_wifi_widget() -> gtk4::Box {
    let (main_box, wifi_switch, list_box) = render::build_wifi_ui();

    let wifi_status = babydra_common::helper::wifi::get_wifi_state().0;
    let state = Rc::new(RefCell::new(WifiState {
        enabled: wifi_status,
        networks: Vec::new(),
    }));

    wifi_switch.set_active(wifi_status);

    let render_networks = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
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
                            let _ = babydra_common::helper::wifi::connect_wifi(&ssid_clone, None, None);
                        } else {
                            if let Some(win) = list_box_parent.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
                                let dialog = gtk4::Dialog::with_buttons(
                                    Some(&format!("Kết nối đến {}", ssid_clone)),
                                    Some(&win),
                                    gtk4::DialogFlags::MODAL,
                                    &[("Hủy", gtk4::ResponseType::Cancel), ("Kết nối", gtk4::ResponseType::Accept)],
                                );

                                let content_area = dialog.content_area();
                                content_area.set_margin_top(12);
                                content_area.set_margin_bottom(12);
                                content_area.set_margin_start(12);
                                content_area.set_margin_end(12);
                                content_area.set_spacing(10);

                                let user_entry = if security_clone == "8021x" {
                                    let user_lbl = gtk4::Label::new(Some("Tài khoản (Identity):"));
                                    user_lbl.set_halign(gtk4::Align::Start);
                                    content_area.append(&user_lbl);

                                    let entry = gtk4::Entry::new();
                                    content_area.append(&entry);
                                    Some(entry)
                                } else {
                                    None
                                };

                                let entry_lbl = gtk4::Label::new(Some("Nhập mật khẩu cho mạng này:"));
                                entry_lbl.set_halign(gtk4::Align::Start);
                                content_area.append(&entry_lbl);

                                let pw_entry = gtk4::Entry::new();
                                pw_entry.set_visibility(false);
                                content_area.append(&pw_entry);

                                let ssid_btn = ssid_clone.clone();
                                let user_entry_btn = user_entry.clone();
                                dialog.connect_response(move |d, res| {
                                    if res == gtk4::ResponseType::Accept {
                                        let password = pw_entry.text().to_string();
                                        let username = user_entry_btn.as_ref().map(|e| e.text().to_string());
                                        let _ = babydra_common::helper::wifi::connect_wifi(
                                            &ssid_btn,
                                            username.as_deref(),
                                            Some(&password),
                                        );
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
            {
                let mut st = state_clone.borrow_mut();
                if st.enabled {
                    st.networks = babydra_common::helper::wifi::scan_networks();
                } else {
                    st.networks.clear();
                }
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
