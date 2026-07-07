//! System desktop notifications D-Bus daemon and storage.

use std::cell::RefCell;
use std::collections::HashMap;
use std::thread;
use zbus::interface;

/// Representation of an active desktop notification.
#[derive(Clone, Debug)]
pub struct ActiveNotification {
    /// Summary or title of the notification message.
    pub title: String,
    /// Detailed description body of the notification.
    pub body: String,
    /// Icon key or file path representing the sender app.
    pub icon: String,
    /// Friendly name of the app sending the notification.
    pub app_name: String,
    /// Time instant when the notification was created.
    pub timestamp: std::time::Instant,
}

/// Dynamic Island DBus communication command messages.
#[derive(Debug)]
pub enum NotificationMsg {
    /// Triggered on receiving a new notification.
    New {
        summary: String,
        body: String,
        icon: String,
        app_name: String,
        timeout: i32,
    },
    /// Command to close the current active popup.
    Close,
}

thread_local! {
    /// Holds reference to the active single dynamic popup notification.
    pub static SHARED_NOTIFICATION: RefCell<Option<ActiveNotification>> = RefCell::new(None);
    /// Holds rolling history of past system notifications.
    pub static HISTORICAL_NOTIFICATIONS: RefCell<Vec<ActiveNotification>> = RefCell::new(Vec::new());
}

fn get_dnd_file_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    std::path::Path::new(&home).join(".config/babydra/dnd")
}

/// Checks if DND mode is active.
pub fn is_dnd_active() -> bool {
    get_dnd_file_path().exists()
}

/// Sets DND mode state.
pub fn set_dnd_active(active: bool) {
    let path = get_dnd_file_path();
    if active {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::File::create(&path);
    } else {
        let _ = std::fs::remove_file(&path);
    }
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
        let id = self.current_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        let mut icon = app_icon.to_string();
        if icon.is_empty() {
            let lower_name = app_name.to_lowercase();
            let apps = crate::desktop::apps::find_desktop_apps();
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
        
        let _ = self.sender.send(NotificationMsg::New {
            summary: summary.to_string(),
            body: body.to_string(),
            icon,
            app_name: app_name.to_string(),
            timeout: expire_timeout,
        });
        
        id
    }

    async fn close_notification(&self, _id: u32) {
        let _ = self.sender.send(NotificationMsg::Close);
    }

    async fn get_capabilities(&self) -> Vec<String> {
        vec!["body".to_string(), "icon-static".to_string()]
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

            match conn_result {
                Ok(conn) => {
                    use zbus::fdo::RequestNameFlags;
                    let _ = conn.request_name_with_flags(
                        "org.freedesktop.Notifications",
                        RequestNameFlags::ReplaceExisting | RequestNameFlags::DoNotQueue,
                    ).await;
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                    }
                }
                Err(_) => {}
            }
        });
    });
}

/// Dismisses popup display window.
pub fn close_notification_popup() {
    // Managed inside notch capsules, no-op
}

/// Registers the incoming desktop notification, caching it to the rolling historical notifications log.
pub fn show_notification_popup(summary: &str, body: &str, icon_name: &str, app_name: &str, _timeout_ms: i32) {
    let notif = ActiveNotification {
        title: summary.to_string(),
        body: body.to_string(),
        icon: icon_name.to_string(),
        app_name: app_name.to_string(),
        timestamp: std::time::Instant::now(),
    };

    if !is_dnd_active() {
        SHARED_NOTIFICATION.with(|sn| {
            *sn.borrow_mut() = Some(notif.clone());
        });
    }

    HISTORICAL_NOTIFICATIONS.with(|list| {
        let mut list_borrow = list.borrow_mut();
        list_borrow.push(notif);
        if list_borrow.len() > 50 {
            list_borrow.remove(0);
        }
    });
}

/// Sends a desktop notification using the default theme/common logo.
pub fn send_notification(title: &str, body: &str) {
    let logo_path = crate::desktop::icon::get_logo_path();
    let logo_str = logo_path.to_string_lossy();
    let _ = std::process::Command::new("notify-send")
        .args(&["-i", &logo_str, title, body])
        .spawn();
}
