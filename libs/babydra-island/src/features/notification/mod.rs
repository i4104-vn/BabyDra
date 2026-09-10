//! Desktop notification overlay feature for the Dynamic Island.
//!
//! Displays a compact notification indicator in the notch capsule and a
//! glassmorphic badge popover below, dismissing when clicking the island capsule.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | Thư mục / File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct + constructor + `IslandFeature` impl (vòng đời + tick) |
//! | `ui/notch.rs` | Widget hiển thị trên notch capsule (`NotificationNotchWidgets`) |
//! | `ui/popover.rs` | Popover badge hiển thị chi tiết thông báo (`NotificationPopover`) |
//! | `ui/render.rs` | Đẩy dữ liệu notification vào widget popover |
//! | `service/dbus.rs` | Service nền: hosting D-Bus notification daemon |

pub mod service;
pub mod ui;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gtk4::prelude::*;

use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::spawn_notif_dbus;
use ui::{render_popover_notification, NotificationNotchWidgets, NotificationPopover};

pub const PRIORITY: u8 = 90;
const NOTCH_WIDTH: i32 = 110;
const NOTCH_HEIGHT: i32 = 28;
const POPUP_LIFETIME: Duration = Duration::from_secs(5);

/// Notification overlay feature: displays a compact notification indicator
/// in the notch capsule and a glassmorphic badge popover below.
pub struct NotificationFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotificationNotchWidgets,
    popover: Rc<RefCell<Option<NotificationPopover>>>,
    last_key: String,
}

impl NotificationFeature {
    pub fn new() -> Self {
        spawn_notif_dbus();
        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets: NotificationNotchWidgets::build(),
            popover: Rc::new(RefCell::new(None)),
            last_key: String::new(),
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

/// Dismisses the notification, closes the popover, and hides the island.
fn dismiss_notification(
    popover: Option<&NotificationPopover>,
    handle: Option<&IslandViewHandle>,
) {
    if let Some(p) = popover {
        p.popdown();
    }
    crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| *sn.borrow_mut() = None);
    if let Some(h) = handle {
        h.release_override();
        h.hide();
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
        (NOTCH_WIDTH, NOTCH_HEIGHT)
    }

    fn hover_keep(&self) -> bool {
        true
    }

    fn focus(&self) -> bool {
        false
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.container.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        *self.handle_rc.borrow_mut() = Some(handle.clone());
    }

    fn attach(&mut self, ctx: &IslandCtx) {
        let popover = NotificationPopover::new(&ctx.capsule());

        // 1. Clicking notification content: activate app and dismiss
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

        // 2. Clicking close button [✕]: dismiss notification
        {
            let handle_rc = self.handle_rc.clone();
            let pop_c = popover.clone();
            popover.close_gesture.connect_pressed(move |_, _, _, _| {
                dismiss_notification(Some(&pop_c), handle_rc.borrow().as_ref());
            });
        }

        // 3. Clean up on popover unmap: clear active notification & hide island
        {
            let handle_rc = self.handle_rc.clone();
            popover.popover.connect_unmap(move |_| {
                crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| *sn.borrow_mut() = None);
                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.release_override();
                    h.hide();
                }
            });
        }

        // 4. Clicking the island notch capsule: dismiss notification immediately
        {
            let handle_rc = self.handle_rc.clone();
            let pop_c = popover.clone();
            self.widgets.click_gesture.connect_pressed(move |_, _, _, _| {
                dismiss_notification(Some(&pop_c), handle_rc.borrow().as_ref());
            });
        }

        self.popover.replace(Some(popover));
    }

    fn on_click(&mut self) {
        // Dismiss notification on island capsule click
        dismiss_notification(
            self.popover.borrow().as_ref(),
            self.handle_rc.borrow().as_ref(),
        );
        self.last_key.clear();
    }

    fn on_hide(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.popdown();
        }
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        let notif =
            crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| sn.borrow().clone());
        let n = match notif {
            Some(n) => n,
            None => {
                self.last_key.clear();
                if let Some(popover) = self.popover.borrow().as_ref() {
                    if popover.is_visible() {
                        popover.popdown();
                    }
                }
                if let Some(h) = self.handle_rc.borrow().as_ref() {
                    h.hide();
                }
                return;
            }
        };

        let key = format!("{}|{}|{}", n.title, n.body, n.icon);
        if key != self.last_key {
            self.last_key = key;
            if let Some(popover) = self.popover.borrow().as_ref() {
                render_popover_notification(popover, &n);
                popover.popup();
            }
            if let Some(h) = self.handle_rc.borrow().as_ref() {
                h.override_show_for(POPUP_LIFETIME);
            }
        }

        let is_popover_hovered = self
            .popover
            .borrow()
            .as_ref()
            .map(|p| p.is_hovered.get())
            .unwrap_or(false);

        let hovered = ctx.is_hovered() || is_popover_hovered;

        if hovered {
            // Hovering keeps the popup alive (refreshes the expiry timestamp).
            crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| {
                if let Some(cur) = sn.borrow_mut().as_mut() {
                    cur.timestamp = Instant::now();
                }
            });
        }

        let expired = !hovered && n.timestamp.elapsed() >= POPUP_LIFETIME;
        if expired {
            self.last_key.clear();
            dismiss_notification(
                self.popover.borrow().as_ref(),
                self.handle_rc.borrow().as_ref(),
            );
        } else if let Some(h) = self.handle_rc.borrow().as_ref() {
            h.show();
        }
    }
}
