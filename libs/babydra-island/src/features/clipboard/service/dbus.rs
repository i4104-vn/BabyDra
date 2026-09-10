//! D-Bus IPC server hosting `org.babydra.Island` over the user session bus.

use zbus::interface;

pub struct IslandDbusService;

#[interface(name = "org.babydra.Island")]
impl IslandDbusService {
    /// Shows or focuses the clipboard history view.
    async fn show_clipboard(&self) {
        gtk4::glib::idle_add_local_once(|| {
            super::fire_trigger();
        });
    }

    /// Toggles the clipboard history view.
    async fn toggle_clipboard(&self) {
        gtk4::glib::idle_add_local_once(|| {
            super::fire_trigger();
        });
    }
}

pub fn spawn_island_dbus() {
    std::thread::Builder::new()
        .name("babydra-island-dbus".into())
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();
            if let Ok(rt) = rt {
                rt.block_on(async {
                    let service = IslandDbusService;
                    let conn_builder = zbus::connection::Builder::session();
                    if let Ok(builder) = conn_builder {
                        if let Ok(builder) = builder.name("org.babydra.Island") {
                            if let Ok(builder) = builder.serve_at("/org/babydra/Island", service) {
                                if let Ok(conn) = builder.build().await {
                                    let _ = std::future::pending::<()>().await;
                                    drop(conn);
                                }
                            }
                        }
                    }
                });
            }
        })
        .ok();
}
