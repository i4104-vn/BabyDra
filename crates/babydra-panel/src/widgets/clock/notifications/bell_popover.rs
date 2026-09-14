//! Hover popover for the panel clock bell icon showing app names and unread counts.

use super::group::group_notifs_by_app;
use super::icon::resolve_notification_icon;
use super::popup::NotificationPopup;
use babydra_core::models::ActiveNotification;
use babydra_ui_kit::components::popovers::TooltipPopover;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up the hover popover for the clock notification bell icon.
pub fn setup_bell_popover(
    bell_anchor: &gtk4::Box,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    notification_popup: Rc<NotificationPopup>,
) -> TooltipPopover {
    let tooltip = TooltipPopover::new(bell_anchor, gtk4::PositionType::Bottom);
    tooltip.popover.add_css_class("clock-bell-popover");

    let cw_c = calendar_window.clone();
    let ccw_c = control_center_window.clone();
    let lw_c = launcher_window.clone();
    let notif_popup_c = notification_popup.clone();

    tooltip.set_suppress_fn(move || {
        cw_c.borrow().is_some()
            || ccw_c.borrow().is_some()
            || lw_c.borrow().is_some()
            || notif_popup_c.is_visible()
    });

    let tt_c = tooltip.clone();
    let update_fn: Rc<dyn Fn()> = Rc::new(move || {
        let notifications: Vec<ActiveNotification> =
            babydra_core::services::notification::service::HISTORICAL_NOTIFICATIONS
                .with(|list| list.borrow().iter().cloned().collect());
        let card = build_bell_card(&notifications);
        tt_c.popover.set_child(Some(&card));
    });

    tooltip.attach_hover(bell_anchor, Some(update_fn.clone()));

    tooltip
}

/// Builds the card widget displaying unread notification summary grouped by app.
fn build_bell_card(notifications: &[ActiveNotification]) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    card.add_css_class("status-popover-card");
    card.add_css_class("tooltip-popover-card");
    card.add_css_class("clock-bell-popover-card");
    card.set_margin_top(4);
    card.set_margin_bottom(4);
    card.set_margin_start(6);
    card.set_margin_end(6);

    // Header: Title & Total badge
    let header = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    header.set_hexpand(true);
    header.set_valign(gtk4::Align::Center);

    let title_lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans("panel.notifications")));
    title_lbl.add_css_class("status-popover-header");
    title_lbl.set_halign(gtk4::Align::Start);
    title_lbl.set_hexpand(true);
    header.append(&title_lbl);

    let total = notifications.len();
    if total > 0 {
        let total_badge = gtk4::Label::new(Some(&total.to_string()));
        total_badge.add_css_class("clock-bell-popover-total-badge");
        total_badge.set_halign(gtk4::Align::End);
        total_badge.set_valign(gtk4::Align::Center);
        total_badge.set_xalign(0.5);
        header.append(&total_badge);
    }
    card.append(&header);

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("status-popover-sep");
    card.append(&sep);

    if notifications.is_empty() {
        let empty_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        empty_box.add_css_class("clock-bell-popover-empty");
        empty_box.set_margin_top(2);
        empty_box.set_margin_bottom(2);

        let empty_lbl =
            gtk4::Label::new(Some(&babydra_core::i18n::trans("panel.no_notifications")));
        empty_lbl.add_css_class("status-popover-key");
        empty_lbl.set_opacity(0.65);
        empty_lbl.set_halign(gtk4::Align::Start);
        empty_box.append(&empty_lbl);

        card.append(&empty_box);
    } else {
        let (grouped, app_order) = group_notifs_by_app(notifications);
        for app_key in app_order {
            if let Some(list) = grouped.get(&app_key) {
                let count = list.len();
                let latest = list.last().unwrap_or(&list[0]);

                let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
                row.add_css_class("clock-bell-popover-row");
                row.set_valign(gtk4::Align::Center);

                let icon = resolve_notification_icon(&latest.icon, &app_key, 16);
                icon.add_css_class("clock-bell-popover-icon");
                icon.set_valign(gtk4::Align::Center);
                row.append(&icon);

                let display_name = if app_key == "system" || app_key == "babydra" {
                    babydra_core::i18n::trans("panel.system")
                } else {
                    let mut chars = app_key.chars();
                    match chars.next() {
                        None => babydra_core::i18n::trans("panel.notification"),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                };

                let name_lbl = gtk4::Label::new(Some(&display_name));
                name_lbl.add_css_class("status-popover-key");
                name_lbl.add_css_class("clock-bell-popover-name");
                name_lbl.set_halign(gtk4::Align::Start);
                name_lbl.set_hexpand(true);
                name_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                name_lbl.set_max_width_chars(20);
                row.append(&name_lbl);

                let badge = gtk4::Label::new(Some(&count.to_string()));
                badge.add_css_class("clock-bell-popover-badge");
                badge.set_halign(gtk4::Align::End);
                badge.set_valign(gtk4::Align::Center);
                badge.set_xalign(0.5);
                row.append(&badge);

                card.append(&row);
            }
        }
    }

    card
}
