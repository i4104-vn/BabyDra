use std::path::PathBuf;
use zbus::interface;

pub struct ExploreDbusService {
    nav_tx: tokio::sync::mpsc::UnboundedSender<(PathBuf, Option<PathBuf>)>,
}

#[interface(name = "org.babydra.Explore")]
impl ExploreDbusService {
    async fn show_folder(&self, path: String) -> zbus::fdo::Result<()> {
        let (target_dir, focus_item) = crate::services::explore::resolve_target_from_uri(&path);
        let _ = self.nav_tx.send((target_dir, focus_item));
        Ok(())
    }
}

pub struct FileManager1Service {
    nav_tx: tokio::sync::mpsc::UnboundedSender<(PathBuf, Option<PathBuf>)>,
}

#[interface(name = "org.freedesktop.FileManager1")]
impl FileManager1Service {
    async fn show_folders(&self, uris: Vec<String>, _startup_id: String) -> zbus::fdo::Result<()> {
        for uri in uris {
            let (target_dir, focus_item) = crate::services::explore::resolve_target_from_uri(&uri);
            if self.nav_tx.send((target_dir.clone(), focus_item.clone())).is_err() {
                let mut cmd = std::process::Command::new("babydra-explore");
                if let Some(focus) = focus_item {
                    cmd.arg(focus);
                } else {
                    cmd.arg(target_dir);
                }
                let _ = cmd.spawn();
            }
        }
        Ok(())
    }

    async fn show_items(&self, uris: Vec<String>, _startup_id: String) -> zbus::fdo::Result<()> {
        for uri in uris {
            let (target_dir, focus_item) = crate::services::explore::resolve_target_from_uri(&uri);
            if self.nav_tx.send((target_dir.clone(), focus_item.clone())).is_err() {
                let mut cmd = std::process::Command::new("babydra-explore");
                if let Some(focus) = focus_item {
                    cmd.arg(focus);
                } else {
                    cmd.arg(target_dir);
                }
                let _ = cmd.spawn();
            }
        }
        Ok(())
    }

    async fn show_item_properties(
        &self,
        uris: Vec<String>,
        _startup_id: String,
    ) -> zbus::fdo::Result<()> {
        for uri in uris {
            let (target_dir, focus_item) = crate::services::explore::resolve_target_from_uri(&uri);
            if self.nav_tx.send((target_dir.clone(), focus_item.clone())).is_err() {
                let mut cmd = std::process::Command::new("babydra-explore");
                if let Some(focus) = focus_item {
                    cmd.arg(focus);
                } else {
                    cmd.arg(target_dir);
                }
                let _ = cmd.spawn();
            }
        }
        Ok(())
    }
}

pub async fn start_dbus_service(
    nav_tx: tokio::sync::mpsc::UnboundedSender<(PathBuf, Option<PathBuf>)>,
) -> Result<(), zbus::Error> {
    let _conn = zbus::connection::Builder::session()?
        .name("org.babydra.Explore")?
        .serve_at(
            "/org/babydra/Explore",
            ExploreDbusService {
                nav_tx: nav_tx.clone(),
            },
        )?
        .serve_at("/org/freedesktop/FileManager1", FileManager1Service { nav_tx })?
        .build()
        .await?;

    let _ = _conn.request_name("org.freedesktop.FileManager1").await;

    // Keep D-Bus session connection alive while the explore window/process runs
    std::future::pending::<()>().await;

    Ok(())
}


