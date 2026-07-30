//! Wi-Fi network list renderer.

use gtk4::prelude::*;
use super::WifiState;
use babydra_utils::components::modal::{WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog};
use std::rc::Rc;

pub fn render_network_list(
    list_box: &gtk4::ListBox,
    st: &WifiState,
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    _config_dialog: &Rc<WifiConfigDialog>,
) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

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

        let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.wifi_disabled")));
        lbl.add_css_class("settings-row-title");
        placeholder_box.append(&lbl);

        let desc = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.wifi_disabled_sub")));
        desc.add_css_class("settings-row-desc");
        placeholder_box.append(&desc);

        row.set_child(Some(&placeholder_box));
        list_box.append(&row);
        return;
    }

    if st.enabled && st.is_loading && st.networks.is_empty() {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        placeholder_box.set_valign(gtk4::Align::Center);
        placeholder_box.set_halign(gtk4::Align::Center);
        placeholder_box.set_margin_top(30);
        placeholder_box.set_margin_bottom(30);

        let spinner = gtk4::Spinner::new();
        spinner.set_size_request(32, 32);
        spinner.set_halign(gtk4::Align::Center);
        spinner.start();
        placeholder_box.append(&spinner);

        let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.loading")));
        lbl.add_css_class("settings-row-title");
        placeholder_box.append(&lbl);

        row.set_child(Some(&placeholder_box));
        list_box.append(&row);
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

        let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.wifi_no_networks")));
        lbl.add_css_class("settings-row-title");
        placeholder_box.append(&lbl);

        row.set_child(Some(&placeholder_box));
        list_box.append(&row);
        return;
    }

    for net in &st.networks {
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

        let wifi_icon = babydra_utils::ui::icon::get_icon("wifi", 18);
        wifi_icon.set_pixel_size(18);
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
            let lock_icon = babydra_utils::ui::icon::get_icon("lock", 12);
            lock_icon.set_pixel_size(12);
            lock_icon.add_css_class("settings-row-desc");
            lock_icon.set_valign(gtk4::Align::Center);
            name_box.append(&lock_icon);
        }

        let click_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
        click_box.set_hexpand(true);
        click_box.set_cursor_from_name(Some("pointer"));
        click_box.append(&icon_badge);
        click_box.append(&name_box);
        hbox.append(&click_box);

        if net.is_connected {
            let check_icon = babydra_utils::ui::icon::get_icon("check", 18);
            check_icon.set_pixel_size(18);
            check_icon.set_valign(gtk4::Align::Center);
            check_icon.add_css_class("connected-text");
            hbox.append(&check_icon);
        }

        let info_btn = gtk4::Button::new();
        info_btn.add_css_class("icon-btn");
        info_btn.set_valign(gtk4::Align::Center);

        let info_icon = babydra_utils::ui::icon::get_icon("info", 16);
        info_icon.set_pixel_size(16);
        info_btn.set_child(Some(&info_icon));
        hbox.append(&info_btn);

        // Connect info button
        let net_info = net.clone();
        let info_dlg_c = info_dialog.clone();
        info_btn.connect_clicked(move |_| {
            let ssid = net_info.ssid.clone();
            let net_clone = net_info.clone();
            let info_dlg_inner = info_dlg_c.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let config = babydra_common::services::system::wifi::get_wifi_config(&ssid);
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
        let gesture = gtk4::GestureClick::new();
        gesture.connect_pressed(move |_, _, _, _| {
            if net_conn.is_connected {
                return;
            }
            if net_conn.security != "open" {
                pwd_dlg_c.show_for(&net_conn.ssid, &net_conn.security);
            } else {
                let ssid = net_conn.ssid.clone();
                std::thread::spawn(move || {
                    babydra_common::services::system::wifi::connect_wifi(&ssid, None, None);
                });
            }
        });
        click_box.add_controller(gesture);

        row.set_child(Some(&hbox));
        list_box.append(&row);
    }
}
