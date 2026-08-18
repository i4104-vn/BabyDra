//! Desktop notification overlay feature for the Dynamic Island.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct + constructor + `IslandFeature` impl (vòng đời + tick) |
//! | `view.rs` | Xây dựng cây widget (`NotificationView::build`) |
//! | `render.rs` | Đẩy dữ liệu notification vào widget (`render`) |
//! | `service.rs` | Service nền: hosting D-Bus notification daemon |

mod render;
mod service;
mod view;

use std::time::{Duration, Instant};

use gtk4::prelude::*;

use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use view::NotificationView;

pub const PRIORITY: u8 = 90;
const POPUP_LIFETIME: Duration = Duration::from_secs(5);

/// Notification overlay feature: shows the latest notification in the capsule
/// for 5 seconds (extended while the pointer hovers) and focuses the sender
/// app when clicked.
pub struct NotificationFeature {
    handle: Option<IslandViewHandle>,
    view: NotificationView,
    last_key: String,
    desired: (i32, i32),
}

impl NotificationFeature {
    pub fn new() -> Self {
        service::spawn_notification_dbus_service();
        Self {
            handle: None,
            view: NotificationView::build(),
            last_key: String::new(),
            desired: (280, 52),
        }
    }
}

impl Default for NotificationFeature {
    fn default() -> Self {
        Self::new()
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
        self.desired
    }

    fn hover_keep(&self) -> bool {
        true
    }

    fn capsule_class(&self) -> Option<String> {
        Some("notification-mode".to_string())
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.view.root.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        self.handle = Some(handle.clone());
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        let notif =
            crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| sn.borrow().clone());
        let n = match notif {
            Some(n) => n,
            None => {
                if let Some(h) = &self.handle {
                    h.hide();
                }
                return;
            }
        };

        let key = format!("{}|{}|{}", n.title, n.body, n.icon);
        if key != self.last_key {
            self.last_key = key;
            self.render(&n);
        }

        if ctx.is_hovered() {
            // Hovering keeps the popup alive (refreshes the expiry timestamp).
            crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| {
                if let Some(cur) = sn.borrow_mut().as_mut() {
                    cur.timestamp = Instant::now();
                }
            });
        }

        let expired = !ctx.is_hovered() && n.timestamp.elapsed() >= POPUP_LIFETIME;
        if expired {
            crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| *sn.borrow_mut() = None);
            if let Some(h) = &self.handle {
                h.hide();
            }
        } else if let Some(h) = &self.handle {
            h.show();
        }
    }

    fn on_click(&mut self) {
        // Focus the app that sent the notification.
        let app_to_activate = crate::widgets::notification::SHARED_NOTIFICATION
            .with(|sn| sn.borrow().as_ref().map(|n| n.app_name.clone()));

        if let Some(app_name) = app_to_activate {
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
}
