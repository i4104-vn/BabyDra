//! VPN card grid renderer and event handlers.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use babydra_core::services::system::vpn::{connect_vpn, disconnect_vpn, get_vpn_details, VpnConn};
use babydra_ui_kit::components::modals::{VpnConfigDialog, VpnLogDialog};
use babydra_ui_kit::components::switch::CustomSwitch;

#[allow(clippy::too_many_arguments)]
pub fn render_vpn_list<F: Fn() + Clone + 'static>(
    flow_box: &gtk4::FlowBox,
    stack: &gtk4::Stack,
    vpns: &[VpnConn],
    is_loading: bool,
    connecting_vpns: &Rc<RefCell<HashSet<String>>>,
    tx_action: &Sender<(String, bool)>,
    config_dialog: &VpnConfigDialog,
    log_dialog: &VpnLogDialog,
    _trigger_refresh: F,
) {
    crate::widgets::helpers::clear_flow_box(flow_box);

    if is_loading && vpns.is_empty() {
        stack.set_visible_child_name("loading");
        return;
    }

    if vpns.is_empty() {
        stack.set_visible_child_name("empty");
        return;
    }

    stack.set_visible_child_name("cards");

    for vpn in vpns.iter() {
        let card = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
        card.add_css_class("settings-card");
        card.add_css_class("device-card");
        card.add_css_class("vpn-card");
        card.set_size_request(240, 130);
        card.set_halign(gtk4::Align::Start);
        card.set_valign(gtk4::Align::Start);

        // ── Top Row: Left icon badge, Right control buttons + switch ──
        let top_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        top_row.set_valign(gtk4::Align::Center);

        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Start);

        let shield_icon = babydra_ui_kit::ui::icon::get_icon("shield", 18);
        shield_icon.set_pixel_size(18);
        shield_icon.set_valign(gtk4::Align::Center);
        shield_icon.set_halign(gtk4::Align::Center);
        shield_icon.set_vexpand(true);
        icon_badge.append(&shield_icon);
        top_row.append(&icon_badge);

        let spacer_top = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        spacer_top.set_hexpand(true);
        top_row.append(&spacer_top);

        // View Logs Button
        let log_btn = gtk4::Button::new();
        log_btn.add_css_class("icon-btn");
        log_btn.add_css_class("circular");
        log_btn.set_valign(gtk4::Align::Center);
        log_btn.set_cursor_from_name(Some("pointer"));
        log_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.vpn_view_logs")));

        let log_icon = babydra_ui_kit::ui::icon::get_icon("terminal", 14);
        log_icon.set_pixel_size(14);
        log_btn.set_child(Some(&log_icon));

        let name_log = vpn.name.clone();
        let log_dialog_c = log_dialog.clone();
        log_btn.connect_clicked(move |_| {
            log_dialog_c.show_for_vpn(&name_log);
        });
        top_row.append(&log_btn);

        // Edit / Customize Button
        let edit_btn = gtk4::Button::new();
        edit_btn.add_css_class("icon-btn");
        edit_btn.add_css_class("circular");
        edit_btn.set_valign(gtk4::Align::Center);
        edit_btn.set_cursor_from_name(Some("pointer"));
        edit_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.vpn_edit")));

        let cog_icon = babydra_ui_kit::ui::icon::get_icon("cog", 14);
        cog_icon.set_pixel_size(14);
        edit_btn.set_child(Some(&cog_icon));

        let name_edit = vpn.name.clone();
        let config_dialog_edit = config_dialog.clone();
        edit_btn.connect_clicked(move |_| {
            let config_dialog_edit_c = config_dialog_edit.clone();
            let name = name_edit.clone();
            crate::widgets::helpers::spawn_async_task(
                move || get_vpn_details(&name),
                move |details| {
                    config_dialog_edit_c.show_for_edit(&details);
                },
                30,
            );
        });
        top_row.append(&edit_btn);

        let is_busy = connecting_vpns.borrow().contains(&vpn.name);

        if is_busy {
            let loading_icon = crate::widgets::helpers::create_loading_icon(22);
            loading_icon.set_valign(gtk4::Align::Center);
            top_row.append(&loading_icon);
        } else {
            let switch = CustomSwitch::new(vpn.active);
            let name_switch = vpn.name.clone();
            let tx_action_c = tx_action.clone();
            let config_dialog_c = config_dialog.clone();
            let sw_clone = switch.clone();

            switch.connect_state_set(move |new_state| {
                let name = name_switch.clone();
                let tx_action_sub = tx_action_c.clone();
                let config_dialog_sub = config_dialog_c.clone();
                let sw = sw_clone.clone();

                if new_state {
                    crate::widgets::helpers::spawn_async_task(
                        move || (get_vpn_details(&name), name),
                        move |(details, name)| {
                            if details.username.is_empty() || details.password.is_empty() {
                                config_dialog_sub.show_for_edit(&details);
                                sw.set_active(false);
                                return;
                            }

                            let _ = tx_action_sub.send((name.clone(), true));
                            let tx = tx_action_sub.clone();
                            std::thread::spawn(move || {
                                let _ = connect_vpn(&name);
                                let _ = tx.send((name, false));
                            });
                        },
                        30,
                    );
                } else {
                    let _ = tx_action_sub.send((name.clone(), true));
                    let tx = tx_action_sub.clone();
                    std::thread::spawn(move || {
                        let _ = disconnect_vpn(&name);
                        let _ = tx.send((name, false));
                    });
                }
            });

            top_row.append(&switch.container);
        }

        card.append(&top_row);

        // Spacer to push footer to bottom
        let mid_spacer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        mid_spacer.set_vexpand(true);
        card.append(&mid_spacer);

        // ── Bottom Row: Title + Subtitle on Left, Status Pill on Right ──
        let bottom_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        bottom_row.set_valign(gtk4::Align::End);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        text_box.set_valign(gtk4::Align::End);
        text_box.set_halign(gtk4::Align::Start);
        text_box.set_hexpand(true);

        let name_lbl = gtk4::Label::new(Some(&vpn.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.add_css_class("device-card-title");
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.set_tooltip_text(Some(&vpn.name));
        name_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        text_box.append(&name_lbl);

        let sub_text = if vpn.active {
            if !vpn.ip_address.is_empty() {
                format!("{} • {}", vpn.conn_type.to_uppercase(), vpn.ip_address)
            } else {
                format!("{} • Connected", vpn.conn_type.to_uppercase())
            }
        } else {
            format!("{} • Inactive", vpn.conn_type.to_uppercase())
        };

        let desc_lbl = gtk4::Label::new(Some(&sub_text));
        desc_lbl.add_css_class("settings-row-desc");
        desc_lbl.add_css_class("device-card-subtitle");
        desc_lbl.set_halign(gtk4::Align::Start);
        desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        text_box.append(&desc_lbl);

        bottom_row.append(&text_box);

        let status_lbl = gtk4::Label::new(Some(if vpn.active { "ON" } else { "OFF" }));
        if vpn.active {
            status_lbl.add_css_class("device-card-status-on");
        } else {
            status_lbl.add_css_class("device-card-status-off");
        }
        status_lbl.set_valign(gtk4::Align::End);
        bottom_row.append(&status_lbl);

        card.append(&bottom_row);
        flow_box.append(&card);
    }
}
