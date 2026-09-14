//! Desktop notification overlay feature for the Dynamic Island.
//!
//! Displays a compact notification indicator in the notch capsule and a
//! glassmorphic badge popover below, dismissing when clicking the island capsule.
//!
pub mod service;
pub mod ui;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle, NotchWidget};
use service::spawn_notif_dbus;
use ui::{render_popover_notification, NotificationPopover};

pub const PRIORITY: u8 = 90;
const POPUP_LIFETIME: Duration = Duration::from_secs(5);
const HOVER_REFRESH: Duration = Duration::from_secs(1);

/// Notification overlay feature: displays a compact notification indicator
/// in the notch capsule and a glassmorphic badge popover below.
pub struct NotificationFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotchWidget,
    popover: Rc<RefCell<Option<NotificationPopover>>>,
    last_timestamp: Option<std::time::Instant>,
    last_hover_refresh: Option<std::time::Instant>,
}

impl NotificationFeature {
    pub fn new() -> Self {
        spawn_notif_dbus();
        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets: NotchWidget::new(
                "bell",
                "#f59e0b",
                &babydra_core::i18n::trans("island.notification"),
            ),
            popover: Rc::new(RefCell::new(None)),
            last_timestamp: None,
            last_hover_refresh: None,
        }
    }
}

impl Default for NotificationFeature {
    fn default() -> Self {
        Self::new()
    }
}

/// Activates the desktop window of the app that emitted the notification.
fn activate_sender_app(app_name: Option<String>) {
    if let Some(app_name) = app_name {
        let apps = babydra_core::find_desktop_apps();
        let mut found_app = None;
        let lower_name = app_name.to_lowercase();

        for app in &apps {
            if app.name.to_lowercase() == lower_name {
                found_app = Some(app.clone());
                break;
            }
        }

        if found_app.is_none() {
            for app in &apps {
                if app.name.to_lowercase().contains(&lower_name)
                    || lower_name.contains(&app.name.to_lowercase())
                {
                    found_app = Some(app.clone());
                    break;
                }
            }
        }

        if let Some(app) = found_app {
            babydra_core::services::window::focus_app(
                &app.name,
                &app.exec,
                app.app_id.as_deref(),
                app.window_title.as_deref(),
            );
        } else {
            babydra_core::services::window::focus_app(&app_name, "", Some(&app_name), None);
        }
    }
}

/// Dismisses the current notification without allowing an old animation callback
/// to clear a newer notification.
fn dismiss_notification(popover: Option<&NotificationPopover>, handle: Option<&IslandViewHandle>) {
    let dismissed_at = crate::widgets::notification::SHARED_NOTIFICATION
        .with(|sn| sn.borrow().as_ref().map(|n| n.timestamp));
    let cleared = crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| {
        let mut current = sn.borrow_mut();
        if current.as_ref().map(|n| n.timestamp) == dismissed_at {
            current.take();
            true
        } else {
            false
        }
    });
    if !cleared {
        return;
    }

    let handle_cloned = handle.cloned();
    if let Some(p) = popover {
        p.popdown_animated(move || {
            if let Some(h) = handle_cloned {
                h.release_override();
                h.hide();
            }
        });
    } else {
        if let Some(h) = handle {
            h.release_override();
            h.hide();
        }
    }
}

impl IslandFeature for NotificationFeature {
    fn id(&self) -> &str {
        "notification"
    }

    fn priority(&self) -> u8 {
        PRIORITY
    }

    fn size(&self) -> (i32, i32) {
        (CAPSULE_WIDTH, CAPSULE_HEIGHT)
    }

    fn hover_keep(&self) -> bool {
        false
    }

    fn focus(&self) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        let has_notif =
            crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| sn.borrow().is_some());
        let popover_open = self
            .popover
            .borrow()
            .as_ref()
            .map(|p| p.is_visible())
            .unwrap_or(false);
        has_notif || popover_open
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.container.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        *self.handle_rc.borrow_mut() = Some(handle.clone());
    }

    fn attach(&mut self, ctx: &IslandCtx) {
        let popover = NotificationPopover::new(&ctx.capsule());

        // Clicking notification content: activate app and dismiss
        {
            let handle_rc = self.handle_rc.clone();
            let pop_c = popover.clone();
            popover.click_gesture.connect_pressed(move |_, _, _, _| {
                let app_to_activate = crate::widgets::notification::SHARED_NOTIFICATION
                    .with(|sn| sn.borrow().as_ref().map(|n| n.app_name.clone()));
                activate_sender_app(app_to_activate);
                dismiss_notification(Some(&pop_c), handle_rc.borrow().as_ref());
            });
        }

        self.popover.replace(Some(popover));
    }

    fn on_show(&mut self) {
        self.open_badge();
    }

    fn on_click(&mut self) {
        let is_visible = self
            .popover
            .borrow()
            .as_ref()
            .map(|p| p.is_visible())
            .unwrap_or(false);

        if is_visible {
            dismiss_notification(
                self.popover.borrow().as_ref(),
                self.handle_rc.borrow().as_ref(),
            );
            self.last_timestamp = None;
            self.last_hover_refresh = None;
            self.widgets.set_icon("bell", "#f59e0b");
            self.widgets
                .set_title(&babydra_core::i18n::trans("island.notification"));
        } else {
            self.open_badge();
        }
    }

    fn open_badge(&mut self) {
        let notif =
            crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| sn.borrow().clone());
        if let Some(n) = notif {
            if let Some(popover) = self.popover.borrow().as_ref() {
                render_popover_notification(popover, &n);
                if !popover.is_visible() || popover.is_animating() {
                    popover.popup();
                }
            }
        }
    }

    fn on_hide(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            if popover.is_visible() {
                popover.popdown();
            }
        }
        self.last_hover_refresh = None;
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        let timestamp = crate::widgets::notification::SHARED_NOTIFICATION
            .with(|sn| sn.borrow().as_ref().map(|n| n.timestamp));
        let timestamp = match timestamp {
            Some(timestamp) => timestamp,
            None => {
                self.last_timestamp = None;
                self.last_hover_refresh = None;
                self.widgets.set_icon("bell", "#f59e0b");
                self.widgets
                    .set_title(&babydra_core::i18n::trans("island.notification"));
                if let Some(popover) = self.popover.borrow().as_ref() {
                    if popover.is_visible() {
                        popover.popdown();
                    }
                }
                if let Some(h) = self.handle_rc.borrow().as_ref() {
                    h.release_override();
                    h.hide();
                }
                return;
            }
        };

        if self.last_timestamp != Some(timestamp) {
            self.last_timestamp = Some(timestamp);
            let notif =
                crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| sn.borrow().clone());
            let Some(n) = notif else { return };
            let app_name = if n.app_name.is_empty() {
                babydra_core::i18n::trans("island.notification")
            } else {
                n.app_name.clone()
            };
            self.widgets.set_icon("bell", "#f59e0b");
            self.widgets.set_title(&app_name);

            dismiss_other_popovers(self.popover.borrow().as_ref());

            if let Some(h) = self.handle_rc.borrow().as_ref() {
                h.override_show_for(POPUP_LIFETIME + Duration::from_millis(500));
            }

            if let Some(popover) = self.popover.borrow().as_ref() {
                render_popover_notification(popover, &n);
                if ctx.is_current() {
                    popover.popup();
                }
            }
        }

        let is_hovered = self
            .popover
            .borrow()
            .as_ref()
            .map(|p| p.is_hovered.get())
            .unwrap_or(false);

        if is_hovered {
            let should_refresh = self
                .last_hover_refresh
                .map(|at| at.elapsed() >= HOVER_REFRESH)
                .unwrap_or(true);
            if should_refresh {
                self.last_hover_refresh = Some(std::time::Instant::now());
            }
            if should_refresh {
                if let Some(h) = self.handle_rc.borrow().as_ref() {
                    h.override_show_for(POPUP_LIFETIME + Duration::from_millis(500));
                }
            }
        } else {
            self.last_hover_refresh = None;
        }

        if !is_hovered && timestamp.elapsed() >= POPUP_LIFETIME {
            dismiss_notification(
                self.popover.borrow().as_ref(),
                self.handle_rc.borrow().as_ref(),
            );
            return;
        }

        if ctx.is_current() {
            let badge_visible = self
                .popover
                .borrow()
                .as_ref()
                .map(|popover| popover.is_visible())
                .unwrap_or(false);
            if !badge_visible {
                self.open_badge();
            }
        } else if let Some(h) = self.handle_rc.borrow().as_ref() {
            h.show();
        }
    }
}

fn dismiss_other_popovers(current: Option<&NotificationPopover>) {
    let Some(island) = crate::island::default_island() else {
        return;
    };
    let capsule = island.capsule();
    let current = current.map(|p| p.popover.clone());
    let mut child = capsule.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        let Some(popover) = widget.downcast_ref::<gtk4::Popover>() else {
            continue;
        };
        let is_current = current
            .as_ref()
            .map(|p| p.upcast_ref::<gtk4::Widget>() == popover.upcast_ref::<gtk4::Widget>())
            .unwrap_or(false);
        if !is_current && popover.is_visible() {
            popover.popdown();
        }
    }
}
