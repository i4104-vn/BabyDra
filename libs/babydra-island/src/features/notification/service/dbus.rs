//! Notification D-Bus service hosting.

use crate::models::NotificationMsg;

/// Hosts the D-Bus notification daemon and the main-thread message bridge.
pub fn spawn_notif_dbus() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NotificationMsg>();
    crate::widgets::notification::spawn_dbus_listener(tx);
    glib::MainContext::default().spawn_local(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                NotificationMsg::New {
                    summary,
                    body,
                    icon,
                    app_name,
                    timeout,
                } => {
                    crate::widgets::notification::show_notif_popup(
                        &summary, &body, &icon, &app_name, timeout,
                    );
                }
                NotificationMsg::Close => {
                    crate::widgets::notification::close_notif_popup();
                }
            }
        }
    });
}
