//! D-Bus IPC server hosting `org.babydra.Island` over the user session bus.

use tokio::sync::mpsc::UnboundedSender;
use zbus::interface;

pub struct IslandDbusService {
    tx: UnboundedSender<()>,
}

#[interface(name = "org.babydra.Island")]
impl IslandDbusService {
    /// Shows or focuses the clipboard history view.
    async fn show_clipboard(&self) {
        let _ = self.tx.send(());
    }

    /// Toggles the clipboard history view.
    async fn toggle_clipboard(&self) {
        let _ = self.tx.send(());
    }
}

pub fn spawn_island_dbus() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(()) = rx.recv().await {
            super::fire_trigger();
        }
    });

    std::thread::Builder::new()
        .name("babydra-island-dbus".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();
            if let Ok(rt) = rt {
                rt.block_on(async move {
                    let service = IslandDbusService { tx };
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
