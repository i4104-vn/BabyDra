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

    runner.step("Registering DBus service and Portal for BabyDra FileDialog...");
    let portal_file = dbus_services.join("org.freedesktop.impl.portal.desktop.babydra.service");
    let portal_content = format!(
        "[D-BUS Service]\nName=org.freedesktop.impl.portal.desktop.babydra\nExec={}/babydra-filedialog --portal\n",
        get_local_bin_dir().display()
    );
    let _ = fs::write(&portal_file, portal_content);

    let portal_def_dir = get_home_dir().join(".local/share/xdg-desktop-portal/portals");
    mkdir_p(&portal_def_dir)?;
    let portal_src = repo_root.join("assets/configs/portals/babydra.portal");
    if portal_src.is_file() {
        let _ = fs::copy(&portal_src, portal_def_dir.join("babydra.portal"));
        let _ = runner.run_sudo("mkdir", &["-p", "/usr/share/xdg-desktop-portal/portals"], None);
        if let Some(src_str) = portal_src.to_str() {
            let _ = runner.run_sudo("cp", &[src_str, "/usr/share/xdg-desktop-portal/portals/babydra.portal"], None);
        }
    }

    // Portal configs
    let portal_cfg_user = get_home_dir().join(".config/xdg-desktop-portal");
    mkdir_p(&portal_cfg_user)?;
    let portal_conf_src = repo_root.join("assets/configs/xdg-desktop-portal");
    if portal_conf_src.is_dir() {
        if let Ok(entries) = fs::read_dir(&portal_conf_src) {
            for entry in entries.flatten() {
                let p = entry.path();
                if let Some(file_name) = p.file_name() {
                    let _ = fs::copy(&p, portal_cfg_user.join(file_name));
                    let _ = runner.run_sudo("mkdir", &["-p", "/etc/xdg/xdg-desktop-portal"], None);
                    if let Some(s) = p.to_str() {
                        let dest = format!("/etc/xdg/xdg-desktop-portal/{}", file_name.to_string_lossy());
                        let _ = runner.run_sudo("cp", &[s, &dest], None);
                    }
                }
            }
        }
    }

    // Environment portal config
    let env_dir = get_home_dir().join(".config/environment.d");
    mkdir_p(&env_dir)?;
    let env_src = repo_root.join("assets/configs/environment.d/portal.conf");
    if env_src.is_file() {
        let _ = fs::copy(&env_src, env_dir.join("portal.conf"));
        let _ = runner.run_sudo("mkdir", &["-p", "/etc/environment.d"], None);
        if let Some(s) = env_src.to_str() {
            let _ = runner.run_sudo("cp", &[s, "/etc/environment.d/portal.conf"], None);
        }
    }

    Ok(())
}
