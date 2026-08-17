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

pub async fn start_dbus_service(
    nav_tx: tokio::sync::mpsc::UnboundedSender<PathBuf>,
) -> Result<(), zbus::Error> {
    let _conn = zbus::connection::Builder::session()?
        .name("org.babydra.Explore")?
        .serve_at("/org/babydra/Explore", ExploreDbusService { nav_tx })?
        .build()
        .await?;

    Ok(())
}
