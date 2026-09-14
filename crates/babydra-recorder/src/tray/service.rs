//! StatusNotifierItem implementation for system tray integration.

use crate::dbus::RecorderCommand;
use babydra_core::services::recording::is_recording;
use tokio::sync::mpsc::UnboundedSender;
use zbus::interface;

pub struct StatusNotifierItemService {
    pub tx: UnboundedSender<RecorderCommand>,
}

#[interface(name = "org.kde.StatusNotifierItem")]
impl StatusNotifierItemService {
    #[zbus(property)]
    fn category(&self) -> String {
        "ApplicationStatus".to_string()
    }

    #[zbus(property)]
    fn id(&self) -> String {
        "babydra-recorder".to_string()
    }

    #[zbus(property)]
    fn title(&self) -> String {
        babydra_core::i18n::trans("recorder.title")
    }

    #[zbus(property)]
    fn status(&self) -> String {
        "Active".to_string()
    }

    #[zbus(property)]
    fn icon_name(&self) -> String {
        if is_recording() {
            "media-record".to_string()
        } else {
            "camera-video".to_string()
        }
    }

    #[zbus(property)]
    fn icon_theme_path(&self) -> String {
        String::new()
    }

    #[zbus(property)]
    fn menu(&self) -> zbus::zvariant::ObjectPath<'_> {
        zbus::zvariant::ObjectPath::from_static_str_unchecked("/NO_MENU")
    }

    /// Primary left-click activation: opens recorder control window.
    async fn activate(&self, _x: i32, _y: i32) {
        let _ = self.tx.send(RecorderCommand::ShowUI);
    }

    /// Secondary click (middle click or shortcut): toggle recording.
    async fn secondary_activate(&self, _x: i32, _y: i32) {
        let _ = self.tx.send(RecorderCommand::Toggle);
    }

    /// Context menu right click: opens recorder control window.
    async fn context_menu(&self, _x: i32, _y: i32) {
        let _ = self.tx.send(RecorderCommand::ShowUI);
    }
}

/// Registers this tray item with the session `StatusNotifierWatcher`.
pub async fn register_with_watcher(conn: &zbus::Connection) -> zbus::Result<()> {
    let _ = conn
        .call_method(
            Some("org.kde.StatusNotifierWatcher"),
            "/StatusNotifierWatcher",
            Some("org.kde.StatusNotifierWatcher"),
            "RegisterStatusNotifierItem",
            &"/StatusNotifierItem",
        )
        .await?;
    Ok(())
}
