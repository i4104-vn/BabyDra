//! Wi-Fi network row component.

use babydra_core::i18n::trans;
use babydra_core::models::settings::wifi::WifiNetwork;
use babydra_core::models::settings::WifiState;
use babydra_ui_kit::components::modals::{WifiInfoDialog, WifiPasswordDialog};
use gtk4::prelude::*;
use std::rc::Rc;
use std::sync::mpsc::Sender;

/// Creates a single Wi-Fi network card row.
pub fn create_wifi_row(
    net: &WifiNetwork,
    state_ref: &WifiState,
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    tx_connect_req: &Sender<(String, Option<String>, Option<String>)>,
) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
    hbox.set_margin_top(4);
    hbox.set_margin_bottom(4);
    hbox.set_margin_start(8);
    hbox.set_margin_end(8);

    // Left icon badge with Wi-Fi signal
    let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    icon_badge.add_css_class("blue-icon-badge-sm");
    icon_badge.set_valign(gtk4::Align::Center);
    icon_badge.set_halign(gtk4::Align::Start);
    icon_badge.set_hexpand(false);

    let wifi_icon = babydra_ui_kit::components::create_wifi_net_icon(
        net.signal,
        net.is_connected,
        18,
        Some("#FFFFFF"),
    );
    wifi_icon.set_valign(gtk4::Align::Center);
    wifi_icon.set_halign(gtk4::Align::Center);
    wifi_icon.set_vexpand(true);
    icon_badge.append(&wifi_icon);

    // Text details (SSID & subtitle info)
    let text_col = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    text_col.set_valign(gtk4::Align::Center);
    text_col.set_halign(gtk4::Align::Start);
    text_col.set_hexpand(true);

    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    title_row.set_valign(gtk4::Align::Center);

    let ssid_lbl = gtk4::Label::new(Some(&net.ssid));
    ssid_lbl.add_css_class("settings-row-title");
    ssid_lbl.set_halign(gtk4::Align::Start);
    title_row.append(&ssid_lbl);

    if net.is_connected {
        let badge = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.badge_connected")));
        badge.add_css_class("wifi-connected-badge");
        badge.set_valign(gtk4::Align::Center);
        title_row.append(&badge);
    }
    text_col.append(&title_row);

    let is_connecting_this = state_ref.connecting_ssid.as_ref() == Some(&net.ssid);
    let sub_text = format_subtitle(net, is_connecting_this);
    let desc_lbl = gtk4::Label::new(Some(&sub_text));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(gtk4::Align::Start);
    text_col.append(&desc_lbl);

    // Clickable container for connecting
    let click_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
    click_box.set_hexpand(true);
    click_box.set_cursor_from_name(Some("pointer"));
    click_box.append(&icon_badge);
    click_box.append(&text_col);
    hbox.append(&click_box);

    // Security lock icon if protected
    if net.security != "open" {
        let lock_icon = babydra_ui_kit::ui::icon::get_icon("lock", 14);
        lock_icon.set_pixel_size(14);
        lock_icon.add_css_class("settings-row-desc");
        lock_icon.set_valign(gtk4::Align::Center);
        lock_icon.set_tooltip_text(Some(&net.security.to_uppercase()));
        hbox.append(&lock_icon);
    }

    // Status indicator (loading spinner if connecting)
    if is_connecting_this {
        let loading_icon = crate::widgets::helpers::create_loading_icon(22);
        loading_icon.set_valign(gtk4::Align::Center);
        hbox.append(&loading_icon);
    }

    // Info button
    let info_btn = create_info_button(net, info_dialog);
    hbox.append(&info_btn);

    // Connect gesture
    let is_connecting_other = state_ref.connecting_ssid.is_some() && !is_connecting_this;
    if state_ref.connecting_ssid.is_none() {
        wire_connect_gesture(&click_box, net, password_dialog, tx_connect_req);
    } else if is_connecting_other {
        row.set_sensitive(false);
        info_btn.set_sensitive(false);
    }

    row.set_child(Some(&hbox));
    row
}

fn format_subtitle(net: &WifiNetwork, is_connecting_this: bool) -> String {
    if is_connecting_this {
        trans("wifi.connecting")
    } else if net.is_connected {
        format!(
            "{} • Signal: {}%",
            trans("settings.wifi_connected"),
            net.signal
        )
    } else if net.is_saved {
        format!("{} • Signal: {}%", trans("settings.wifi_saved"), net.signal)
    } else {
        format!("Signal: {}% • {}", net.signal, net.security.to_uppercase())
    }
}

fn create_info_button(net: &WifiNetwork, info_dialog: &Rc<WifiInfoDialog>) -> gtk4::Button {
    let info_btn = gtk4::Button::new();
    info_btn.add_css_class("flat");
    info_btn.add_css_class("wifi-info-btn");
    info_btn.set_valign(gtk4::Align::Center);
    info_btn.set_cursor_from_name(Some("pointer"));
    info_btn.set_tooltip_text(Some(&trans("settings.wifi_details")));

    let info_icon = babydra_ui_kit::ui::icon::get_icon("info", 16);
    info_icon.set_pixel_size(16);
    info_btn.set_child(Some(&info_icon));

    let ssid = net.ssid.clone();
    let net_clone = net.clone();
    let info_dlg_c = info_dialog.clone();
    info_btn.connect_clicked(move |_| {
        let s = ssid.clone();
        let n = net_clone.clone();
        let dlg = info_dlg_c.clone();
        crate::widgets::helpers::spawn_async_task(
            move || babydra_core::services::system::wifi::get_wifi_config(&s),
            move |config| dlg.show_for(&n, Some(&config)),
            30,
        );
    });

    info_btn
}

fn wire_connect_gesture(
    widget: &gtk4::Box,
    net: &WifiNetwork,
    password_dialog: &Rc<WifiPasswordDialog>,
    tx_connect_req: &Sender<(String, Option<String>, Option<String>)>,
) {
    let net_conn = net.clone();
    let pwd_dlg_c = password_dialog.clone();
    let tx_req = tx_connect_req.clone();

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
    widget.add_controller(gesture);
}
