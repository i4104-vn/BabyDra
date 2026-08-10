use gtk4::prelude::*;
use std::rc::Rc;
use babydra_utils::components::popovers::hover::{
    HoverPopoverRow as PopoverRow, build_hover_popover_card as build_popover_card,
};

pub fn build_vpn_update_fn(
    vpn_icon: &gtk4::Image,
    vpn_popover: &gtk4::Popover,
) -> Rc<dyn Fn()> {
    let vpn_icon_c = vpn_icon.clone();
    let vpn_popover_c = vpn_popover.clone();

    Rc::new(move || {
        if let Some(active_vpn) = babydra_common::services::system::vpn::get_active_vpn_fast() {
            let proto_str = if !active_vpn.cipher.is_empty() {
                format!("{} ({})", active_vpn.conn_type.to_uppercase(), active_vpn.cipher)
            } else {
                active_vpn.conn_type.to_uppercase()
            };

            let display_name = if active_vpn.name.chars().count() > 30 {
                let truncated: String = active_vpn.name.chars().take(30).collect();
                format!("{}...", truncated)
            } else {
                active_vpn.name.clone()
            };

            let mut rows = vec![
                PopoverRow::new("Status", "Active", None),
                PopoverRow::new("Name", &display_name, None),
                PopoverRow::new("Type", &proto_str, None),
            ];

            if !active_vpn.remote_server.is_empty() {
                rows.push(PopoverRow::new("Server", &active_vpn.remote_server, None));
            } else if !active_vpn.gateway.is_empty() {
                rows.push(PopoverRow::new("Gateway", &active_vpn.gateway, None));
            }

            if !active_vpn.ip_address.is_empty() {
                rows.push(PopoverRow::new("VPN IP", &active_vpn.ip_address, None));
            }

            if !active_vpn.username.is_empty() {
                rows.push(PopoverRow::new("User", &active_vpn.username, None));
            }

            if !active_vpn.dev_iface.is_empty() {
                rows.push(PopoverRow::new("Interface", &active_vpn.dev_iface, None));
            }

            let card = build_popover_card("VPN Connection", rows);
            vpn_popover_c.set_child(Some(&card));
            vpn_icon_c.set_visible(true);
        } else {
            vpn_icon_c.set_visible(false);
        }
    })
}
