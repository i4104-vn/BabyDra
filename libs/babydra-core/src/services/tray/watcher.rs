//! DBus watcher object for org.kde.StatusNotifierWatcher.

pub use crate::models::TrayItem;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use zbus::interface;

static TRAY_ITEMS: OnceLock<Arc<Mutex<Vec<TrayItem>>>> = OnceLock::new();

/// Returns a cloned copy of all currently registered system tray items.
pub fn get_tray_items() -> Vec<TrayItem> {
    let registry = TRAY_ITEMS.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
    registry.lock().unwrap().clone()
}

/// Helper to parse service registration string and sender header into bus name and object path.
pub fn parse_service_and_path(service: &str, sender: &str) -> (String, String) {
    if service.starts_with('/') {
        (sender.to_string(), service.to_string())
    } else if let Some(slash_idx) = service.find('/') {
        let bus = &service[..slash_idx];
        let path = &service[slash_idx..];
        (bus.to_string(), path.to_string())
    } else {
        (service.to_string(), "/StatusNotifierItem".to_string())
    }
}

#[zbus::proxy(
    interface = "org.kde.StatusNotifierItem",
    default_path = "/StatusNotifierItem"
)]
pub trait StatusNotifierItem {
    fn activate(&self, x: i32, y: i32) -> zbus::Result<()>;
    fn secondary_activate(&self, x: i32, y: i32) -> zbus::Result<()>;
    fn context_menu(&self, x: i32, y: i32) -> zbus::Result<()>;

    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn title(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn icon_theme_path(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn attention_icon_name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn menu(&self) -> zbus::Result<zbus::zvariant::ObjectPath<'_>>;
}

/// DBus watcher object for org.kde.StatusNotifierWatcher.
pub struct StatusNotifierWatcher;

#[interface(name = "org.kde.StatusNotifierWatcher")]
impl StatusNotifierWatcher {
    #[zbus(name = "RegisterStatusNotifierItem")]
    async fn register_status_notifier_item(
        &self,
        service: String,
        #[zbus(header)] header: zbus::message::Header<'_>,
    ) {
        let sender = header.sender().map(|s| s.to_string()).unwrap_or_default();
        let (bus_name_str, object_path) = parse_service_and_path(&service, &sender);

        if bus_name_str.is_empty() {
            return;
        }

        let connection = match zbus::Connection::session().await {
            Ok(conn) => conn,
            Err(_) => return,
        };

        let bus_name = match zbus::names::BusName::try_from(bus_name_str.clone()) {
            Ok(name) => name,
            Err(_) => return,
        };

        let proxy = match StatusNotifierItemProxy::builder(&connection)
            .destination(bus_name)
            .unwrap()
            .path(object_path.clone())
            .unwrap()
            .build()
            .await
        {
            Ok(p) => p,
            Err(_) => return,
        };

        let icon_name = match proxy.icon_name().await {
            Ok(name) if !name.is_empty() => name,
            _ => proxy.attention_icon_name().await.unwrap_or_default(),
        };

        let id = proxy.id().await.unwrap_or_default();
        let title = proxy.title().await.unwrap_or_else(|_| id.clone());

        let final_icon = if !icon_name.is_empty() {
            if let Ok(theme_path) = proxy.icon_theme_path().await {
                if !theme_path.is_empty() {
                    let p = std::path::PathBuf::from(&theme_path).join(&icon_name);
                    if p.exists() {
                        p.to_string_lossy().to_string()
                    } else {
                        icon_name
                    }
                } else {
                    icon_name
                }
            } else {
                icon_name
            }
        } else if !id.is_empty() {
            id
        } else {
            "application-x-executable".to_string()
        };

        let item = TrayItem {
            service: bus_name_str,
            path: object_path,
            icon_name: final_icon,
            title,
        };

        let registry = TRAY_ITEMS.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
        let mut lock = registry.lock().unwrap();
        lock.retain(|x| !(x.service == item.service && x.path == item.path));
        lock.push(item);
    }

    #[zbus(name = "RegisterStatusNotifierHost")]
    async fn register_status_notifier_host(&self, _service: String) {}

    #[zbus(property)]
    async fn registered_status_notifier_items(&self) -> Vec<String> {
        get_tray_items()
            .into_iter()
            .map(|x| {
                if x.path == "/StatusNotifierItem" {
                    x.service
                } else {
                    format!("{}/{}", x.service, x.path.trim_start_matches('/'))
                }
            })
            .collect()
    }

    #[zbus(property)]
    async fn is_status_notifier_host_registered(&self) -> bool {
        true
    }

    #[zbus(property)]
    async fn protocol_version(&self) -> i32 {
        0
    }
}

/// Spawns the D-Bus StatusNotifierWatcher service in a background tokio thread.
/// Also starts a periodic health check loop to remove disconnected tray icons.
pub fn spawn_watcher() {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let watcher = StatusNotifierWatcher;

            let conn = match zbus::connection::Builder::session()
                .unwrap()
                .name("org.kde.StatusNotifierWatcher")
                .unwrap()
                .serve_at("/StatusNotifierWatcher", watcher)
                .unwrap()
                .build()
                .await
            {
                Ok(c) => c,
                Err(_) => return,
            };

            loop {
                tokio::time::sleep(Duration::from_millis(500)).await;
                let registry = TRAY_ITEMS.get_or_init(|| Arc::new(Mutex::new(Vec::new())));
                let current_items = {
                    let lock = registry.lock().unwrap();
                    lock.clone()
                };

                let mut active_items = Vec::new();
                if let Ok(dbus_proxy) = zbus::fdo::DBusProxy::new(&conn).await {
                    let mut handles = Vec::new();
                    for item in current_items {
                        let conn_clone = conn.clone();
                        let dbus_proxy_clone = dbus_proxy.clone();
                        let handle = tokio::spawn(async move {
                            if let Ok(bus_name) =
                                zbus::names::BusName::try_from(item.service.clone())
                            {
                                let has_owner = tokio::time::timeout(
                                    Duration::from_millis(200),
                                    dbus_proxy_clone.name_has_owner(bus_name.clone()),
                                )
                                .await;

                                match has_owner {
                                    Ok(Ok(true)) => {
                                        let item_clone = item.clone();
                                        let conn_clone2 = conn_clone.clone();

                                        let query_fut = async move {
                                            if let Ok(proxy) =
                                                StatusNotifierItemProxy::builder(&conn_clone2)
                                                    .destination(bus_name)
                                                    .unwrap()
                                                    .path(item_clone.path.clone())
                                                    .unwrap()
                                                    .build()
                                                    .await
                                            {
                                                let icon_name = match proxy.icon_name().await {
                                                    Ok(name) if !name.is_empty() => name,
                                                    _ => proxy.attention_icon_name().await.unwrap_or_default(),
                                                };
                                                let id = proxy.id().await.unwrap_or_default();
                                                let title = proxy.title().await.unwrap_or_else(|_| id.clone());

                                                let final_icon = if !icon_name.is_empty() {
                                                    if let Ok(theme_path) = proxy.icon_theme_path().await {
                                                        if !theme_path.is_empty() {
                                                            let p = std::path::PathBuf::from(&theme_path).join(&icon_name);
                                                            if p.exists() {
                                                                p.to_string_lossy().to_string()
                                                            } else {
                                                                icon_name
                                                            }
                                                        } else {
                                                            icon_name
                                                        }
                                                    } else {
                                                        icon_name
                                                    }
                                                } else if !id.is_empty() {
                                                    id
                                                } else if !item_clone.icon_name.is_empty() {
                                                    item_clone.icon_name.clone()
                                                } else {
                                                    "application-x-executable".to_string()
                                                };

                                                Some((final_icon, title))
                                            } else {
                                                None
                                            }
                                        };

                                        if let Ok(Some((new_icon, new_title))) =
                                            tokio::time::timeout(
                                                Duration::from_millis(250),
                                                query_fut,
                                            )
                                            .await
                                        {
                                            return Some(TrayItem {
                                                service: item.service,
                                                path: item.path,
                                                icon_name: new_icon,
                                                title: new_title,
                                            });
                                        } else {
                                            return Some(item);
                                        }
                                    }
                                    _ => {
                                        return None;
                                    }
                                }
                            }
                            Some(item)
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        if let Ok(Some(updated_item)) = handle.await {
                            active_items.push(updated_item);
                        }
                    }
                }

                let mut lock = registry.lock().unwrap();
                *lock = active_items;
            }
        });
    });
}
