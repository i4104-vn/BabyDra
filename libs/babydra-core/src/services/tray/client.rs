//! StatusNotifierItem activation handler.

use super::watcher::StatusNotifierItemProxy;

/// Sends an Activate or ContextMenu signal to the item's D-Bus service, letting the application open its menu or window.
pub fn activate_item(service: &str, x: i32, y: i32, is_right_click: bool) {
    let service_str = service.to_string();
    glib::spawn_future_local(async move {
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
}

pub async fn fetch_menu_path(service: &str) -> Option<String> {
    let conn = zbus::Connection::session().await.ok()?;
    let bus_name = zbus::names::BusName::try_from(service.to_string()).ok()?;

    let proxy = StatusNotifierItemProxy::builder(&conn)
        .destination(bus_name)
        .unwrap()
        .path("/StatusNotifierItem")
        .unwrap()
        .build()
        .await
        .ok()?;

    proxy.menu().await.ok().map(|p| p.to_string())
}

/// Returns the current `dbus menu`.
pub fn get_dbus_menu(service: &str) -> Option<Vec<crate::models::MenuItem>> {
    let service_str = service.to_string();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let menu_path = fetch_menu_path(&service_str).await?;
        if menu_path.is_empty() {
            return None;
        }

        let conn = zbus::Connection::session().await.ok()?;
        let bus_name = zbus::names::BusName::try_from(service_str).ok()?;

        let proxy = super::dbusmenu::DbusMenuProxy::builder(&conn)
            .destination(bus_name)
            .unwrap()
            .path(menu_path)
            .unwrap()
            .build()
            .await
            .ok()?;

        let layout_res = proxy
            .get_layout(
                0,
                2,
                &["type", "label", "visible", "enabled", "children-display"],
            )
            .await;
        if let Ok((_, layout_item)) = layout_res {
            let menu = super::dbusmenu::parse_layout_item(&layout_item);
            Some(menu.children)
        } else {
            None
        }
    })
}

/// Activate menu item.
pub fn activate_menu_item(service: &str, item_id: i32) {
    let service_str = service.to_string();
    glib::spawn_future_local(async move {
        let menu_path = fetch_menu_path(&service_str).await.unwrap_or_default();
        if menu_path.is_empty() {
            return;
        }

        if let Ok(conn) = zbus::Connection::session().await {
            if let Ok(bus_name) = zbus::names::BusName::try_from(service_str) {
                if let Ok(proxy) = super::dbusmenu::DbusMenuProxy::builder(&conn)
                    .destination(bus_name)
                    .unwrap()
                    .path(menu_path)
                    .unwrap()
                    .build()
                    .await
                {
                    let empty_str = zbus::zvariant::Value::from("");
                    let _ = proxy.event(item_id, "clicked", &empty_str, 0).await;
                }
            }
        }
    });
}
