use std::path::PathBuf;
use zbus::interface;

pub struct ExploreDbusService {
    nav_tx: tokio::sync::mpsc::UnboundedSender<PathBuf>,
}

#[interface(name = "org.babydra.Explore")]
impl ExploreDbusService {
    async fn show_folder(&self, path: String) -> zbus::fdo::Result<()> {
        let path_buf = PathBuf::from(path);
        let _ = self.nav_tx.send(path_buf);
        Ok(())
    }
}

pub struct FileManager1Service;

#[interface(name = "org.freedesktop.FileManager1")]
impl FileManager1Service {
    async fn show_folders(&self, uris: Vec<String>, _startup_id: String) -> zbus::fdo::Result<()> {
        for uri in uris {
            let _ = std::process::Command::new("babydra-explore")
                .arg(uri)
                .spawn();
        }
        Ok(())
    }

    async fn show_items(&self, uris: Vec<String>, _startup_id: String) -> zbus::fdo::Result<()> {
        for uri in uris {
            let _ = std::process::Command::new("babydra-explore")
                .arg(uri)
                .spawn();
        }
        Ok(())
    }

    async fn show_item_properties(
        &self,
        uris: Vec<String>,
        _startup_id: String,
    ) -> zbus::fdo::Result<()> {
        for uri in uris {
            let _ = std::process::Command::new("babydra-explore")
                .arg(uri)
                .spawn();
        }
        Ok(())
    }
}

pub async fn start_dbus_service(
    nav_tx: tokio::sync::mpsc::UnboundedSender<PathBuf>,
) -> Result<(), zbus::Error> {
    let _conn = zbus::connection::Builder::session()?
        .name("org.babydra.Explore")?
        .serve_at("/org/babydra/Explore", ExploreDbusService { nav_tx })?
        .serve_at("/org/freedesktop/FileManager1", FileManager1Service)?
        .build()
        .await?;

    let _ = _conn.request_name("org.freedesktop.FileManager1").await;

    Ok(())
}
