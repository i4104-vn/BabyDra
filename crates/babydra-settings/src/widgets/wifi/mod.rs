//! Wi-Fi configurations control panel.

use std::cell::RefCell;
use std::rc::Rc;
use babydra_common::models::wifi::WifiNetwork;
use gtk4::prelude::*;

mod handler;
mod render;

pub struct WifiState {
    pub enabled: bool,
    pub networks: Vec<WifiNetwork>,
    pub is_loading: bool,
}

pub fn create_wifi_widget() -> gtk4::Widget {
    let (overlay, wifi_switch, list_box, info_dialog, password_dialog, config_dialog) = render::build_wifi_ui();

    let info_dialog = Rc::new(info_dialog);
    let password_dialog = Rc::new(password_dialog);
    let config_dialog = Rc::new(config_dialog);

    let state = Rc::new(RefCell::new(WifiState {
        enabled: false,
        networks: Vec::new(),
        is_loading: true,
    }));

    // Async fetch initial Wi-Fi switch status off main thread
    let (tx_status, rx_status) = std::sync::mpsc::channel::<bool>();
    std::thread::spawn(move || {
        let status = babydra_common::services::system::wifi::get_wifi_state().0;
        let _ = tx_status.send(status);
    });

    let wifi_switch_c = wifi_switch.clone();
    let state_c_init = state.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        match rx_status.try_recv() {
            Ok(status) => {
                wifi_switch_c.set_active(status);
                state_c_init.borrow_mut().enabled = status;
                glib::ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
        }
    });

    let render_networks = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        let info_dlg_c = info_dialog.clone();
        let pwd_dlg_c = password_dialog.clone();
        let cfg_dlg_c = config_dialog.clone();
        move || {
            let st = state_clone.borrow();
<<<<<<< HEAD
            handler::render_network_list(
                &list_box_clone,
                &st,
                &info_dlg_c,
                &pwd_dlg_c,
                &cfg_dlg_c,
            );
=======
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
>>>>>>> hard-develop
        }
    };

    // Wire Password Dialog submit
    let pwd_dlg_inner = password_dialog.clone();
    password_dialog.connect_submit(move |pwd, username| {
        let ssid = pwd_dlg_inner.ssid_lbl.text().to_string();
        let ssid_clean = ssid.trim_start_matches("Connect to ").to_string();
        pwd_dlg_inner.hide();
        std::thread::spawn(move || {
            let user_ref = username.as_deref();
            let pwd_ref = if pwd.is_empty() { None } else { Some(pwd.as_str()) };
            let _ = babydra_common::services::system::wifi::connect_wifi(&ssid_clean, user_ref, pwd_ref);
        });
    });

    // Wire Info Dialog configure button click
    let info_dlg_inner = info_dialog.clone();
    let cfg_dlg_inner = config_dialog.clone();
    info_dialog.connect_configure(move || {
        let ssid = info_dlg_inner.ssid_lbl.text().to_string();
        let cfg_dlg_target = cfg_dlg_inner.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let ssid_clone = ssid.clone();
        std::thread::spawn(move || {
            let config = babydra_common::services::system::wifi::get_wifi_config(&ssid_clone);
            let _ = tx.send(config);
        });
        glib::timeout_add_local(std::time::Duration::from_millis(30), move || {
            if let Ok(config) = rx.try_recv() {
                cfg_dlg_target.show_for(&ssid, &config);
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    });

    // Wire Config Dialog save button click
    config_dialog.connect_save(move |ssid, config| {
        std::thread::spawn(move || {
            let _ = babydra_common::services::system::wifi::set_wifi_config(&ssid, &config);
        });
    });

    // Background thread scanning channel
    let (tx_scan, rx_scan) = std::sync::mpsc::channel::<Vec<WifiNetwork>>();
    let list_box_mapped_check = list_box.clone();
    let render_nets_loading = render_networks.clone();
    let trigger_wifi_scan = {
        let tx_c = tx_scan.clone();
        let state_c = state.clone();
        let list_box_c = list_box_mapped_check.clone();
        let render_c = render_nets_loading.clone();
        move || {
            // ONLY fetch when tab is active/mapped!
            if !list_box_c.is_mapped() {
                return;
            }

            let (enabled, is_empty) = {
                let st = state_c.borrow();
                (st.enabled, st.networks.is_empty())
            };

            if enabled {
                if is_empty {
                    state_c.borrow_mut().is_loading = true;
                    render_c();
                }
                let tx_sub = tx_c.clone();
                std::thread::spawn(move || {
                    let nets = babydra_common::services::system::wifi::scan_networks();
                    let _ = tx_sub.send(nets);
                });
            }
        }
    };

    let state_scan_render = state.clone();
    let render_nets = render_networks.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
        let mut updated = false;
        while let Ok(nets) = rx_scan.try_recv() {
            let mut st = state_scan_render.borrow_mut();
            st.networks = nets;
            st.is_loading = false;
            updated = true;
        }
        if updated {
            render_nets();
        }
        glib::ControlFlow::Continue
    });

    // Trigger scan instantly when tab becomes active/mapped
    let trigger_map = trigger_wifi_scan.clone();
    list_box.connect_map(move |_| {
        trigger_map();
    });

    // Trigger initial scan
    let trigger_init = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        trigger_init();
        glib::ControlFlow::Break
    });

    // Trigger periodic scan (every 6s) ONLY when tab is mapped
    let trigger_periodic = trigger_wifi_scan.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(6), move || {
        trigger_periodic();
        glib::ControlFlow::Continue
    });

    let trigger_switch = trigger_wifi_scan.clone();
    let state_switch = state.clone();
    let render_switch = render_networks.clone();
    wifi_switch.connect_state_set(move |_, is_active| {
        let is_active_bool = is_active;
        {
            let mut st = state_switch.borrow_mut();
            st.enabled = is_active_bool;
            if !is_active_bool {
                st.networks.clear();
                st.is_loading = false;
            }
        }
        std::thread::spawn(move || {
            babydra_common::services::system::wifi::set_wifi_enabled(is_active_bool);
        });
        if is_active_bool {
            trigger_switch();
        } else {
            render_switch();
        }
        glib::Propagation::Proceed
    });

    overlay.into()
}
