//! System desktop notifications D-Bus daemon and storage.

use std::cell::RefCell;
use std::collections::HashMap;
use std::thread;
use zbus::fdo::RequestNameFlags;
use zbus::interface;

pub use crate::models::{ActiveNotification, NotificationMsg};

thread_local! {
    /// Holds reference to the active single dynamic popup notification.
    pub static SHARED_NOTIFICATION: RefCell<Option<ActiveNotification>> = const { RefCell::new(None) };
    /// Holds rolling history of past system notifications.
    pub static HISTORICAL_NOTIFICATIONS: RefCell<std::collections::VecDeque<ActiveNotification>> = const { RefCell::new(std::collections::VecDeque::new()) };
}

/// Checks if DND mode is active.
pub fn is_dnd_active() -> bool {
    let conf = crate::config::load_babydra_config();
    conf.notification.dnd
}

/// Sets DND mode state.
pub fn set_dnd_active(active: bool) {
    let mut conf = crate::config::load_babydra_config();
    conf.notification.dnd = active;
    crate::config::save_babydra_config(&conf);
}

/// DBus Notifications interface server object.
pub struct NotificationService {
    sender: tokio::sync::mpsc::UnboundedSender<NotificationMsg>,
    current_id: std::sync::atomic::AtomicU32,
}

impl NotificationService {
    pub fn new(sender: tokio::sync::mpsc::UnboundedSender<NotificationMsg>) -> Self {
        Self {
            sender,
            current_id: std::sync::atomic::AtomicU32::new(1),
        }
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationService {
    #[allow(clippy::too_many_arguments)]
    async fn notify(
        &self,
        app_name: &str,
        _replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        _actions: Vec<&str>,
        _hints: HashMap<&str, zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> u32 {
        let id = self
            .current_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let mut icon = app_icon.to_string();
        if icon.is_empty() {
            // Hot path: only query desktop apps cache when icon is missing
            let lower_name = app_name.to_lowercase();
            let apps = crate::services::apps::find_desktop_apps();
            for app in apps {
                if app.name.to_lowercase() == lower_name {
                    if let Some(app_icon) = app.icon {
                        icon = app_icon;
                    }
                    break;
                }
            }
        }
        if icon.is_empty() {
            icon = app_name.to_lowercase();
        }

        let command = _hints
            .get("command")
            .or_else(|| _hints.get("x-babydra-command"))
            .and_then(|v| match v {
                zbus::zvariant::Value::Str(s) => Some(s.to_string()),
                _ => None,
            });

        let _ = self.sender.send(NotificationMsg::New {
            summary: summary.to_string(),
            body: body.to_string(),
            icon,
            app_name: app_name.to_string(),
            timeout: expire_timeout,
            command,
        });

        id
    }

    async fn close_notification(&self, _id: u32) {
        let _ = self.sender.send(NotificationMsg::Close);
    }

    async fn get_capabilities(&self) -> Vec<String> {
        vec!["body".to_string(), "icon-static".to_string()]
    }

    async fn get_server_information(&self) -> (String, String, String, String) {
        (
            "BabyDra Notifications".to_string(),
            "BabyDra".to_string(),
            "1.0".to_string(),
            "1.2".to_string(),
        )
    }
}

/// Spawns a background thread running Tokio to serve the org.freedesktop.Notifications DBus daemon.
pub fn spawn_dbus_listener(tx: tokio::sync::mpsc::UnboundedSender<NotificationMsg>) {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let service = NotificationService::new(tx);
            let conn_result = zbus::connection::Builder::session()
                .unwrap()
                .serve_at("/org/freedesktop/Notifications", service)
                .unwrap()
                .build()
                .await;

            if let Ok(conn) = conn_result {
                let _ = conn
                    .request_name_with_flags(
                        "org.freedesktop.Notifications",
                        RequestNameFlags::ReplaceExisting | RequestNameFlags::DoNotQueue,
                    )
                    .await;
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                }
            }
        });
    });
}

/// Dismisses popup display window.
pub fn close_notif_popup() {
    SHARED_NOTIFICATION.with(|sn| {
        let _ = sn.borrow_mut().take();
    });
}

/// Registers the incoming desktop notification, caching it to the rolling historical notifications log.
pub fn show_notif_popup(
    summary: &str,
    body: &str,
    icon_name: &str,
    app_name: &str,
    timeout_ms: i32,
) {
    show_notif_popup_with_cmd(summary, body, icon_name, app_name, timeout_ms, None);
}

/// Registers the incoming desktop notification with an optional click command.
pub fn show_notif_popup_with_cmd(
    summary: &str,
    body: &str,
    icon_name: &str,
    app_name: &str,
    _timeout_ms: i32,
    command: Option<String>,
) {
    let notif = ActiveNotification {
        title: summary.to_string(),
        body: body.to_string(),
        icon: icon_name.to_string(),
        app_name: app_name.to_string(),
        command,
        timestamp: std::time::Instant::now(),
    };

    if !is_dnd_active() {
        SHARED_NOTIFICATION.with(|sn| {
            *sn.borrow_mut() = Some(notif.clone());
        });
    }

    HISTORICAL_NOTIFICATIONS.with(|list| {
        let mut list_borrow = list.borrow_mut();
        list_borrow.push_back(notif);
        if list_borrow.len() > 50 {
            list_borrow.pop_front();
        }
    });
}

#[allow(clippy::too_many_arguments)]
#[zbus::proxy(
    gen_blocking = true,
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: HashMap<&str, zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

/// Sends a desktop notification using the default theme/common logo.
pub fn send_notification(title: &str, body: &str) {
    send_app_notif("BabyDra", title, body, "babydra");
}

/// Sends a desktop notification with an executable click command using default logo.
pub fn send_notification_with_cmd(title: &str, body: &str, command: &str) {
    send_app_notif_with_cmd("BabyDra", title, body, "babydra", command);
}

/// Sends a desktop notification for Settings using the logo as icon.
pub fn send_settings_notif(title: &str, body: &str) {
    send_app_notif("Settings", title, body, "");
}

/// Sends a desktop notification specifying an explicit icon name.
pub fn send_notif_icon(title: &str, body: &str, icon_name: &str) {
    send_app_notif("BabyDra", title, body, icon_name);
}

/// Sends a desktop notification with a custom app name and icon.
pub fn send_app_notif(app_name: &str, title: &str, body: &str, icon_name: &str) {
    send_app_notif_with_cmd(app_name, title, body, icon_name, "");
}

/// Sends a desktop notification with a custom app name, icon, and an executable command when clicked.
pub fn send_app_notif_with_cmd(
    app_name: &str,
    title: &str,
    body: &str,
    icon_name: &str,
    command: &str,
) {
    let system_logo = "/usr/share/babydra/logo.png";

    let logo_str = if icon_name.is_empty() {
        if std::path::Path::new(system_logo).exists() {
            system_logo
        } else {
            "babydra"
        }
    } else {
        icon_name
    };

    let mut hints = HashMap::new();
    if !command.is_empty() {
        hints.insert("command", zbus::zvariant::Value::Str(command.into()));
    }

    if let Ok(conn) = zbus::blocking::Connection::session() {
        if let Ok(proxy) = NotificationsProxyBlocking::new(&conn) {
            let _ = proxy.notify(
                app_name,
                0,
                logo_str,
                title,
                body,
                vec![],
                hints,
                -1,
            );
            return;
        }
    }
    let mut cmd = std::process::Command::new("notify-send");
    cmd.args(["-a", app_name, "-i", logo_str, title, body]);
    if !command.is_empty() {
        cmd.args(["--hint", &format!("string:command:{}", command)]);
    }
    let _ = cmd.spawn();
}
