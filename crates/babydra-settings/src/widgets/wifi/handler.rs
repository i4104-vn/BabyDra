use super::WifiState;
use babydra_core::i18n::t;
use babydra_ui_kit::components::modals::{WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog};
use gtk4::prelude::*;
use std::rc::Rc;

/// Renders `network list`.
pub fn render_network_list(
    container: &gtk4::Box,
    st: &WifiState,
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    _config_dialog: &Rc<WifiConfigDialog>,
    tx_connect_req: std::sync::mpsc::Sender<(String, Option<String>, Option<String>)>,
) {
    crate::widgets::helpers::clear_box(container);

    let create_placeholder = |row| {
        let lb = gtk4::ListBox::new();
        lb.set_selection_mode(gtk4::SelectionMode::None);
        lb.add_css_class("settings-card");
        lb.append(&row);
        lb
    };

    if !st.enabled {
        container.append(&create_placeholder(
            crate::widgets::helpers::create_placeholder_row(
                crate::widgets::helpers::PlaceholderState::Disabled {
                    title_key: "settings.wifi_disabled",
                    desc_key: "settings.wifi_disabled_sub",
                    icon_name: "wifi",
                },
            ),
        ));
        return;
    }

    if st.enabled && st.is_loading && st.networks.is_empty() {
        container.append(&create_placeholder(
            crate::widgets::helpers::create_placeholder_row(
                crate::widgets::helpers::PlaceholderState::Loading,
            ),
        ));
        return;
    }

    if st.networks.is_empty() {
        container.append(&create_placeholder(
            crate::widgets::helpers::create_placeholder_row(
                crate::widgets::helpers::PlaceholderState::Empty {
                    title_key: "settings.wifi_no_networks",
                    desc_key: None,
                    icon_name: "wifi",
                },
            ),
        ));
        return;
    }

    let mut display_networks = st.networks.clone();
    display_networks.sort_by(|a, b| {
        let a_conn = a.is_connected || st.connecting_ssid.as_ref() == Some(&a.ssid);
        let b_conn = b.is_connected || st.connecting_ssid.as_ref() == Some(&b.ssid);
        if a_conn != b_conn {
            return b_conn.cmp(&a_conn);
        }
        if a.is_saved != b.is_saved {
            return b.is_saved.cmp(&a.is_saved);
        }
        b.signal.cmp(&a.signal)
    });

    let mut current_section = 0;
    let mut current_lb: Option<gtk4::ListBox> = None;

    for net in &display_networks {
        let section = if net.is_connected || st.connecting_ssid.as_ref() == Some(&net.ssid) {
            1
        } else if net.is_saved {
            2
        } else {
            3
        };

        if section != current_section {
            current_section = section;
            let title_key = match section {
                1 => "settings.wifi_connected",
                2 => "settings.wifi_saved",
                3 => "settings.wifi_available",
                _ => "",
            };

            let header_lbl = gtk4::Label::new(Some(&babydra_core::i18n::t(title_key)));
            header_lbl.add_css_class("settings-row-desc");
            header_lbl.set_halign(gtk4::Align::Start);
            header_lbl.set_margin_start(12);
            header_lbl.set_margin_top(12);
            header_lbl.set_margin_bottom(4);
            container.append(&header_lbl);

            let lb = gtk4::ListBox::new();
            lb.set_selection_mode(gtk4::SelectionMode::None);
            lb.add_css_class("settings-card");
            container.append(&lb);
            current_lb = Some(lb);
        }

        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
        hbox.set_margin_top(4);
        hbox.set_margin_bottom(4);
        hbox.set_margin_start(8);
        hbox.set_margin_end(8);

        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Start);
        icon_badge.set_hexpand(false);

        let wifi_icon = babydra_ui_kit::components::create_wifi_signal_icon_for_network(
            net.signal as u32,
            net.is_connected,
            18,
            Some("#3B82F6"),
        );
        wifi_icon.set_valign(gtk4::Align::Center);
        wifi_icon.set_halign(gtk4::Align::Center);
        wifi_icon.set_vexpand(true);
        icon_badge.append(&wifi_icon);

        let name_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        name_box.set_valign(gtk4::Align::Center);
        name_box.set_halign(gtk4::Align::Start);
        name_box.set_hexpand(true);

        let ssid_lbl = gtk4::Label::new(Some(&net.ssid));
        ssid_lbl.add_css_class("settings-row-title");
        ssid_lbl.set_halign(gtk4::Align::Start);
        ssid_lbl.set_valign(gtk4::Align::Center);
        name_box.append(&ssid_lbl);

        if net.security != "open" {
            let lock_icon = babydra_ui_kit::ui::icon::get_icon("lock", 12);
            lock_icon.set_pixel_size(12);
            lock_icon.add_css_class("settings-row-desc");
            lock_icon.set_valign(gtk4::Align::Center);
            lock_icon.set_tooltip_text(Some(&net.security.to_uppercase()));
            name_box.append(&lock_icon);
        }

        let popover = gtk4::Popover::new();
        popover.set_position(gtk4::PositionType::Top);
        popover.set_autohide(false);
        let pop_vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        pop_vbox.set_margin_top(8);
        pop_vbox.set_margin_bottom(8);
        pop_vbox.set_margin_start(12);
        pop_vbox.set_margin_end(12);

        let sig_lbl = gtk4::Label::new(Some(&format!("Signal: {}%", net.signal)));
        sig_lbl.set_halign(gtk4::Align::Start);
        sig_lbl.add_css_class("settings-row-desc");
        let sec_lbl = gtk4::Label::new(Some(&format!("Security: {}", net.security.to_uppercase())));
        sec_lbl.set_halign(gtk4::Align::Start);
        sec_lbl.add_css_class("settings-row-desc");

        pop_vbox.append(&sig_lbl);
        pop_vbox.append(&sec_lbl);
        popover.set_child(Some(&pop_vbox));
        popover.set_parent(&name_box);

        let motion = gtk4::EventControllerMotion::new();
        let pop_c1 = popover.clone();
        motion.connect_enter(move |_, _, _| {
            pop_c1.popup();
        });
        let pop_c2 = popover.clone();
        motion.connect_leave(move |_| {
            pop_c2.popdown();
        });
        name_box.add_controller(motion);

        let click_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
        click_box.set_hexpand(true);
        click_box.set_cursor_from_name(Some("pointer"));
        click_box.append(&icon_badge);
        click_box.append(&name_box);
        hbox.append(&click_box);

        let is_connecting_this = st.connecting_ssid.as_ref() == Some(&net.ssid);
        let is_connecting_other = st.connecting_ssid.is_some() && !is_connecting_this;

        if is_connecting_this {
            let conn_lbl = gtk4::Label::new(Some(&t("wifi.connecting")));
            conn_lbl.add_css_class("settings-row-desc");
            conn_lbl.set_valign(gtk4::Align::Center);
            conn_lbl.set_margin_end(8);
            hbox.append(&conn_lbl);

            let spinner = gtk4::Spinner::new();
            spinner.start();
            spinner.set_valign(gtk4::Align::Center);
            hbox.append(&spinner);
        } else if net.is_connected {
            let check_icon = babydra_ui_kit::ui::icon::get_icon("check", 18);
            check_icon.set_pixel_size(18);
            check_icon.set_valign(gtk4::Align::Center);
            check_icon.add_css_class("connected-text");
            check_icon.set_tooltip_text(Some(&babydra_core::i18n::t("settings.wifi_connected")));
            hbox.append(&check_icon);
        }

        let info_btn = gtk4::Button::new();
        info_btn.add_css_class("icon-btn");
        info_btn.set_valign(gtk4::Align::Center);

        let info_icon = babydra_ui_kit::ui::icon::get_icon("info", 16);
        info_icon.set_pixel_size(16);
        info_btn.set_child(Some(&info_icon));
        hbox.append(&info_btn);

        let net_info = net.clone();
        let info_dlg_c = info_dialog.clone();
        info_btn.connect_clicked(move |_| {
            let ssid = net_info.ssid.clone();
            let net_clone = net_info.clone();
            let info_dlg_inner = info_dlg_c.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let config = babydra_core::services::system::wifi::get_wifi_config(&ssid);
                let _ = tx.send(config);
            });
            glib::timeout_add_local(std::time::Duration::from_millis(30), move || {
                if let Ok(config) = rx.try_recv() {
                    info_dlg_inner.show_for(&net_clone, Some(&config));
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        });

        // Wire row click for connection
        let net_conn = net.clone();
        let pwd_dlg_c = password_dialog.clone();
        let tx_req = tx_connect_req.clone();

        if !st.connecting_ssid.is_some() {
            let gesture = gtk4::GestureClick::new();
            gesture.connect_pressed(move |_, _, _, _| {
                if net_conn.is_connected {
                    return;
                }
                if net_conn.security != "open" && !net_conn.is_saved {
                    pwd_dlg_c.show_for(&net_conn.ssid, &net_conn.security);
                } else {
                    let _ = tx_req.send((net_conn.ssid.clone(), None, None));
                }
            });
            click_box.add_controller(gesture);
        } else if is_connecting_other {
            row.set_sensitive(false);
            info_btn.set_sensitive(false);
        }

        row.set_child(Some(&hbox));
        if let Some(lb) = &current_lb {
            lb.append(&row);
        }
    }
}
