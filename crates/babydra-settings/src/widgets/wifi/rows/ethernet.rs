//! Ethernet status row component.

use babydra_core::i18n::trans;
use babydra_core::models::network::ActiveNetworkInfo;
use gtk4::prelude::*;

/// Creates an active Ethernet status card row.
pub fn create_ethernet_row(active_net: &ActiveNetworkInfo) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
    hbox.set_margin_top(4);
    hbox.set_margin_bottom(4);
    hbox.set_margin_start(8);
    hbox.set_margin_end(8);

    // Blue Ethernet icon badge
    let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    icon_badge.add_css_class("blue-icon-badge-sm");
    icon_badge.set_valign(gtk4::Align::Center);
    icon_badge.set_halign(gtk4::Align::Start);
    icon_badge.set_hexpand(false);

    let eth_icon = babydra_ui_kit::ui::icon::get_icon_colored("ethernet", 18, "#FFFFFF");
    eth_icon.set_valign(gtk4::Align::Center);
    eth_icon.set_halign(gtk4::Align::Center);
    eth_icon.set_vexpand(true);
    icon_badge.append(&eth_icon);

    // Text details (interface name & IP address)
    let name_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    name_box.set_valign(gtk4::Align::Center);
    name_box.set_halign(gtk4::Align::Start);
    name_box.set_hexpand(true);

    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    title_row.set_valign(gtk4::Align::Center);

    let title_lbl = gtk4::Label::new(Some(&active_net.name));
    title_lbl.add_css_class("settings-row-title");
    title_lbl.set_halign(gtk4::Align::Start);
    title_row.append(&title_lbl);

    let badge = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.badge_connected")));
    badge.add_css_class("wifi-connected-badge");
    badge.set_valign(gtk4::Align::Center);
    title_row.append(&badge);

    name_box.append(&title_row);

    let sub_text = if !active_net.interface.is_empty() {
        format!(
            "{} • IP: {} ({})",
            trans("settings.ethernet_connected"),
            active_net.ip_address,
            active_net.interface
        )
    } else {
        format!(
            "{} • IP: {}",
            trans("settings.ethernet_connected"),
            active_net.ip_address
        )
    };
    let sub_lbl = gtk4::Label::new(Some(&sub_text));
    sub_lbl.add_css_class("settings-row-desc");
    sub_lbl.set_halign(gtk4::Align::Start);
    name_box.append(&sub_lbl);

    hbox.append(&icon_badge);
    hbox.append(&name_box);

    row.set_child(Some(&hbox));
    row
}
