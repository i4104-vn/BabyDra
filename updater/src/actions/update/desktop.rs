use crate::actions::runner::CommandRunner;
use crate::utils::fs::{mkdir_p, set_executable};
use crate::utils::system::{
    get_applications_dir, get_home_dir, get_local_bin_dir, set_xdg_mime, update_desktop_db,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn install_desktop_integrations(
    runner: &CommandRunner,
    repo_root: &Path,
    mime_defaults: &HashMap<String, String>,
) -> Result<(), String> {
    runner.step("Installing desktop application entries...");
    let apps_dir = get_applications_dir();
    mkdir_p(&apps_dir)?;

    let desktops_src = if repo_root.join("assets/desktop").is_dir() {
        repo_root.join("assets/desktop")
    } else if repo_root.join("assets/desktops").is_dir() {
        repo_root.join("assets/desktops")
    } else {
        repo_root.join("desktops")
    };
    if desktops_src.exists() {
        for entry in fs::read_dir(&desktops_src)
            .map_err(|e| format!("Failed to read {}: {}", desktops_src.display(), e))?
        {
            let p = entry
                .map_err(|e| format!("Failed to read desktop entries: {}", e))?
                .path();
            if p.extension().is_some_and(|ext| ext == "desktop") {
                let file_name = p
                    .file_name()
                    .ok_or_else(|| format!("Invalid desktop entry path: {}", p.display()))?;
                let dest = apps_dir.join(file_name);
                fs::copy(&p, &dest)
                    .map_err(|e| format!("Failed to install {}: {}", p.display(), e))?;
                set_executable(&dest)
                    .map_err(|e| format!("Failed to make {} executable: {}", dest.display(), e))?;
            }
        }
        update_desktop_db(&apps_dir);
    }

    runner.step("Configuring default MIME application associations...");
    for (mime, desktop) in mime_defaults {
        set_xdg_mime(mime, desktop);
    }

    runner.step("Registering DBus service for FileManager1...");
    let dbus_services = get_home_dir().join(".local/share/dbus-1/services");
    mkdir_p(&dbus_services)?;
    let service_file = dbus_services.join("org.freedesktop.FileManager1.service");
    let service_content = format!(
        "[D-BUS Service]\nName=org.freedesktop.FileManager1\nExec={}/babydra-explore\n",
        get_local_bin_dir().display()
    );
    fs::write(&service_file, service_content).map_err(|e| {
        format!(
            "Failed to install DBus service {}: {}",
            service_file.display(),
            e
        )
    })?;

    Ok(())
}
