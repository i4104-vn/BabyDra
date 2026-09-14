//! Standard hover popover for the panel clock bell icon showing app names and unread counts.

use super::group::group_notifs_by_app;
use super::popup::NotificationPopup;
use babydra_core::models::ActiveNotification;
use babydra_ui_kit::components::popovers::{TooltipPopover, TooltipRow};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up the standard hover popover for the clock notification bell icon.
pub fn setup_bell_popover(
    bell_anchor: &impl IsA<gtk4::Widget>,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    notification_popup: Rc<NotificationPopup>,
) -> TooltipPopover {
    let tooltip = TooltipPopover::new(bell_anchor, gtk4::PositionType::Bottom);

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

        let mut rows = Vec::new();
        if notifications.is_empty() {
            rows.push(TooltipRow::new(
                &babydra_core::i18n::trans("panel.no_notifications"),
                "",
                None,
            ));
        } else {
            let (grouped, app_order) = group_notifs_by_app(&notifications);
            for app_key in app_order {
                if let Some(list) = grouped.get(&app_key) {
                    let count = list.len();
                    let display_name = if app_key == "system" || app_key == "babydra" {
                        babydra_core::i18n::trans("panel.system")
                    } else {
                        let mut chars = app_key.chars();
                        match chars.next() {
                            None => babydra_core::i18n::trans("panel.notification"),
                            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    };
                    rows.push(TooltipRow::new(&display_name, &count.to_string(), None));
                }
            }
        }

        let card = TooltipPopover::build_card(
            &babydra_core::i18n::trans("panel.notifications"),
            &rows,
        );
        tt_c.popover.set_child(Some(&card));
    });

    tooltip.attach_hover(bell_anchor, Some(update_fn.clone()));

    tooltip
}
