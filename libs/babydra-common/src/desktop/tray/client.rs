//! StatusNotifierItem activation handler.

use super::watcher::StatusNotifierItemProxy;

/// Sends an Activate or ContextMenu signal to the item's D-Bus service, letting the application open its menu or window.
pub fn activate_item(service: &str, x: i32, y: i32, is_right_click: bool) {
    let service_str = service.to_string();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Ok(conn) = zbus::Connection::session().await {
                let bus_name = match zbus::names::BusName::try_from(service_str.clone()) {
                    Ok(name) => name,
                    Err(_) => return,
                };

                let proxy = match StatusNotifierItemProxy::builder(&conn)
                    .destination(bus_name)
                    .unwrap()
                    .path("/StatusNotifierItem")
                    .unwrap()
                    .build()
                    .await
                {
                    Ok(p) => p,
                    Err(_) => return,
                };

                if is_right_click {
                    if proxy.context_menu(x, y).await.is_err() {
                        if proxy.secondary_activate(x, y).await.is_err() {
                            let _ = proxy.activate(x, y).await;
                        }
                    }
                } else {
                    let _ = proxy.activate(x, y).await;
                }
            }
        });
    });
}
