//! D-Bus IPC server hosting `org.babydra.Island` over the user session bus.

use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::UnboundedSender;
use zbus::interface;

pub use crate::features::clipboard::models::IslandDbusCommand;

pub struct IslandDbusService {
    tx: UnboundedSender<IslandDbusCommand>,
}

#[interface(name = "org.babydra.Island")]
impl IslandDbusService {
    /// Shows or focuses the clipboard history view.
    async fn show_clipboard(&self) {
        let _ = self.tx.send(IslandDbusCommand::ShowClipboard);
    }

    /// Toggles the clipboard history view.
    async fn toggle_clipboard(&self) {
        let _ = self.tx.send(IslandDbusCommand::ToggleClipboard);
    }

    /// Shows or focuses the power options view.
    async fn show_power(&self) {
        let _ = self.tx.send(IslandDbusCommand::ShowPower);
    }

    /// Toggles the power options view.
    async fn toggle_power(&self) {
        let _ = self.tx.send(IslandDbusCommand::TogglePower);
    }

    /// Opens the screen recording controls in the Dynamic Island.
    async fn show_recording(&self) {
        let _ = self.tx.send(IslandDbusCommand::ShowRecording);
    }
}

static DBUS_SPAWNED: AtomicBool = AtomicBool::new(false);

pub fn spawn_island_dbus() {
    if DBUS_SPAWNED.swap(true, Ordering::SeqCst) {
        return;
    }
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<IslandDbusCommand>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                IslandDbusCommand::ShowClipboard | IslandDbusCommand::ToggleClipboard => {
                    super::fire_trigger();
                }
                IslandDbusCommand::ShowPower | IslandDbusCommand::TogglePower => {
                    crate::features::power::service::fire_trigger();
                }
                IslandDbusCommand::ShowRecording => {
                    crate::features::recording::service::fire_trigger();
                }
            }
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
