//! Notification daemon lifecycle and main-thread bridge.

use crate::models::NotificationMsg;
use crate::services::notification::service::{
    close_notif_popup, show_notif_popup, spawn_dbus_listener,
};
use std::sync::atomic::{AtomicBool, Ordering};

static STARTED: AtomicBool = AtomicBool::new(false);

/// Starts the notification daemon once and forwards incoming messages locally.
pub fn spawn_notif_dbus() {
    if STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NotificationMsg>();
    spawn_dbus_listener(tx);

    glib::MainContext::default().spawn_local(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                NotificationMsg::New {
                    summary,
                    body,
                    icon,
                    app_name,
                    timeout,
                } => show_notif_popup(&summary, &body, &icon, &app_name, timeout),
                NotificationMsg::Close => close_notif_popup(),
            }
        }
    });
}
