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
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
                placeholder_box.set_valign(gtk4::Align::Center);
                placeholder_box.set_halign(gtk4::Align::Center);
                placeholder_box.set_margin_top(30);
                placeholder_box.set_margin_bottom(30);

                let wifi_icon = babydra_utils::ui::icon::get_icon("wifi", 24);
                wifi_icon.set_pixel_size(24);
                wifi_icon.add_css_class("settings-row-icon");
                placeholder_box.append(&wifi_icon);

                let lbl = gtk4::Label::new(Some("Wi-Fi đang tắt"));
                lbl.add_css_class("settings-row-title");
                placeholder_box.append(&lbl);

                let desc = gtk4::Label::new(Some("Bật công tắc phía trên để tìm kiếm các mạng Wi-Fi khả dụng"));
                desc.add_css_class("settings-row-desc");
                placeholder_box.append(&desc);

                row.set_child(Some(&placeholder_box));
                list_box_clone.append(&row);
                return;
            }

            if st.networks.is_empty() {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
                placeholder_box.set_valign(gtk4::Align::Center);
                placeholder_box.set_halign(gtk4::Align::Center);
                placeholder_box.set_margin_top(30);
                placeholder_box.set_margin_bottom(30);

                let wifi_icon = babydra_utils::ui::icon::get_icon("wifi", 24);
                wifi_icon.set_pixel_size(24);
                wifi_icon.add_css_class("settings-row-icon");
                placeholder_box.append(&wifi_icon);

                let lbl = gtk4::Label::new(Some("Đang tìm kiếm mạng Wi-Fi..."));
                lbl.add_css_class("settings-row-title");
                placeholder_box.append(&lbl);

                row.set_child(Some(&placeholder_box));
                list_box_clone.append(&row);
                return;
            }

            for (ssid, security, strength, is_connected) in &st.networks {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
                hbox.set_margin_top(10);
                hbox.set_margin_bottom(10);
                hbox.set_margin_start(16);
                hbox.set_margin_end(16);

                let wifi_icon = babydra_utils::ui::icon::get_icon("wifi", 16);
                wifi_icon.set_pixel_size(16);
                wifi_icon.set_valign(gtk4::Align::Center);
                wifi_icon.add_css_class("settings-row-icon");
                hbox.append(&wifi_icon);

                let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
                text_box.set_valign(gtk4::Align::Center);
                text_box.set_hexpand(true);

                let ssid_lbl = gtk4::Label::new(Some(ssid));
                ssid_lbl.add_css_class("settings-row-title");
                ssid_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&ssid_lbl);

                let sec_text = if security == "open" {
                    format!("Mạng mở • Tín hiệu {}%", strength)
                } else {
                    format!("Bảo mật ({}) • Tín hiệu {}%", security.to_uppercase(), strength)
                };
                let sec_lbl = gtk4::Label::new(Some(&sec_text));
                sec_lbl.add_css_class("settings-row-desc");
                sec_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&sec_lbl);

                hbox.append(&text_box);

                if *is_connected {
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
                } else {
                    let connect_btn = gtk4::Button::with_label("Kết nối");
                    connect_btn.set_valign(gtk4::Align::Center);
                    connect_btn.add_css_class("connect-pill-btn");

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
                    hbox.append(&connect_btn);
                }

                row.set_child(Some(&hbox));
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
