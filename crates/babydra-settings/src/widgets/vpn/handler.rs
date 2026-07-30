//! VPN list renderer and event handlers.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use babydra_common::services::system::vpn::{
    connect_vpn, disconnect_vpn, get_vpn_details, VpnConn,
};
use babydra_utils::components::modal::{VpnConfigDialog, VpnLogDialog};

pub fn render_vpn_list<F: Fn() + Clone + 'static>(
    list_box: &gtk4::ListBox,
    vpns: &[VpnConn],
    is_loading: bool,
    connecting_vpns: &Rc<RefCell<HashSet<String>>>,
    tx_action: &Sender<(String, bool)>,
    config_dialog: &VpnConfigDialog,
    log_dialog: &VpnLogDialog,
    _trigger_refresh: F,
) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    if is_loading && vpns.is_empty() {
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

    if vpns.is_empty() {
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

        let shield_icon = babydra_utils::ui::icon::get_icon("shield", 24);
        shield_icon.set_pixel_size(24);
        shield_icon.set_valign(gtk4::Align::Center);
        shield_icon.set_halign(gtk4::Align::Center);
        shield_icon.set_vexpand(true);
        icon_badge.append(&shield_icon);
        placeholder_box.append(&icon_badge);

        let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.vpn_no_profiles")));
        lbl.add_css_class("settings-row-title");
        placeholder_box.append(&lbl);

        let desc = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.vpn_no_profiles_sub")));
        desc.add_css_class("settings-row-desc");
        placeholder_box.append(&desc);

        row.set_child(Some(&placeholder_box));
        list_box.append(&row);
        return;
    }

    for vpn in vpns.iter() {
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

        let shield_icon = babydra_utils::ui::icon::get_icon("shield", 18);
        shield_icon.set_pixel_size(18);
        shield_icon.set_valign(gtk4::Align::Center);
        shield_icon.set_halign(gtk4::Align::Center);
        shield_icon.set_vexpand(true);
        icon_badge.append(&shield_icon);
        hbox.append(&icon_badge);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        text_box.set_valign(gtk4::Align::Center);
        text_box.set_halign(gtk4::Align::Start);
        text_box.set_hexpand(true);

        let name_lbl = gtk4::Label::new(Some(&vpn.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&name_lbl);

        let sub_text = if vpn.active {
            let mut info_parts = vec![format!("{}: {}", babydra_common::i18n::t("settings.vpn_type"), vpn.conn_type.to_uppercase())];
            if !vpn.ip_address.is_empty() {
                info_parts.push(format!("IP: {}", vpn.ip_address));
            }
            if !vpn.remote_server.is_empty() {
                info_parts.push(format!("Server: {}", vpn.remote_server));
            } else if !vpn.gateway.is_empty() {
                info_parts.push(format!("Gateway: {}", vpn.gateway));
            }
            if !vpn.dev_iface.is_empty() {
                info_parts.push(format!("Interface: {}", vpn.dev_iface));
            }
            info_parts.join(" • ")
        } else {
            format!("{}: {}", babydra_common::i18n::t("settings.vpn_type"), vpn.conn_type.to_uppercase())
        };

        let desc_lbl = gtk4::Label::new(Some(&sub_text));
        desc_lbl.add_css_class("settings-row-desc");
        desc_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&desc_lbl);

        hbox.append(&text_box);

        // View Logs Button
        let log_btn = gtk4::Button::new();
        log_btn.add_css_class("icon-btn");
        log_btn.set_valign(gtk4::Align::Center);
        log_btn.set_cursor_from_name(Some("pointer"));
        log_btn.set_tooltip_text(Some(&babydra_common::i18n::t("settings.vpn_view_logs")));

        let log_icon = babydra_utils::ui::icon::get_icon("terminal", 14);
        log_icon.set_pixel_size(14);
        log_btn.set_child(Some(&log_icon));

        let name_log = vpn.name.clone();
        let log_dialog_c = log_dialog.clone();
        log_btn.connect_clicked(move |_| {
            log_dialog_c.show_for_vpn(&name_log);
        });
        hbox.append(&log_btn);

        // Edit / Customize Button
        let edit_btn = gtk4::Button::new();
        edit_btn.add_css_class("icon-btn");
        edit_btn.set_valign(gtk4::Align::Center);
        edit_btn.set_cursor_from_name(Some("pointer"));

        let cog_icon = babydra_utils::ui::icon::get_icon("cog", 14);
        cog_icon.set_pixel_size(14);
        edit_btn.set_child(Some(&cog_icon));

        let name_edit = vpn.name.clone();
        let config_dialog_edit = config_dialog.clone();
        edit_btn.connect_clicked(move |_| {
            let details = get_vpn_details(&name_edit);
            config_dialog_edit.show_for_edit(&details);
        });
        hbox.append(&edit_btn);

        let is_busy = connecting_vpns.borrow().contains(&vpn.name);

        if is_busy {
            let spinner = gtk4::Spinner::new();
            spinner.set_valign(gtk4::Align::Center);
            spinner.set_visible(true);
            spinner.start();
            hbox.append(&spinner);
        } else if vpn.active {
            let disconnect_btn = gtk4::Button::with_label(&babydra_common::i18n::t("settings.disconnect"));
            disconnect_btn.set_valign(gtk4::Align::Center);
            disconnect_btn.add_css_class("connect-pill-btn");
            disconnect_btn.add_css_class("delete-btn");

            let name_clone = vpn.name.clone();
            let tx_action_c = tx_action.clone();

            disconnect_btn.connect_clicked(move |_| {
                let _ = tx_action_c.send((name_clone.clone(), true));
                let name = name_clone.clone();
                let tx = tx_action_c.clone();
                std::thread::spawn(move || {
                    let _ = disconnect_vpn(&name);
                    let _ = tx.send((name, false));
                });
            });
            hbox.append(&disconnect_btn);
        } else {
            let connect_btn = gtk4::Button::with_label(&babydra_common::i18n::t("settings.connect"));
            connect_btn.set_valign(gtk4::Align::Center);
            connect_btn.add_css_class("suggested-action");

            let name_clone = vpn.name.clone();
            let tx_action_c = tx_action.clone();
            let config_dialog_c = config_dialog.clone();

            connect_btn.connect_clicked(move |_| {
                let name = name_clone.clone();
                let details = get_vpn_details(&name);

                if details.username.is_empty() || details.password.is_empty() {
                    config_dialog_c.show_for_edit(&details);
                    return;
                }

                let _ = tx_action_c.send((name.clone(), true));
                let tx = tx_action_c.clone();
                std::thread::spawn(move || {
                    let _ = connect_vpn(&name);
                    let _ = tx.send((name, false));
                });
            });
            hbox.append(&connect_btn);
        }

        row.set_child(Some(&hbox));
        list_box.append(&row);
    }
}
