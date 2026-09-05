use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::models::LogLevel;
use crate::system::{get_user_home, SudoSession};

pub fn register_desktop_entries<F>(sudo: &SudoSession, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let apps_dir = home.join(".local/share/applications");
    let _ = fs::create_dir_all(&apps_dir);

    let entries: &[(&str, &str, &str, &str, &str)] = &[
        (
            "babydra-preview",
            "BabyDra Preview",
            "Viewer for images",
            ".local/bin/babydra-preview %f",
            "/usr/share/babydra/babydra-preview.png",
        ),
        (
            "babydra-settings",
            "BabyDra Settings",
            "Configure system settings",
            ".local/bin/babydra-settings",
            "/usr/share/babydra/babydra-settings.png",
        ),
        (
            "babydra-explore",
            "BabyDra Explore",
            "Explore files and folders",
            ".local/bin/babydra-explore %u",
            "system-file-manager",
        ),
    ];
    let mime_types = [
        (
            "babydra-preview",
            "image/png;image/jpeg;image/gif;image/webp;image/bmp;",
        ),
        ("babydra-explore", "inode/directory;"),
    ];
    let categories = [
        ("babydra-preview", "Graphics;Viewer;GTK;"),
        ("babydra-settings", "Settings;HardwareSettings;GTK;"),
        ("babydra-explore", "System;FileTools;FileManager;GTK;"),
    ];

    for (id, name, comment, exec, icon) in entries {
        let mime = mime_types
            .iter()
            .find(|(m, _)| m == id)
            .map(|(_, m)| *m)
            .unwrap_or("");
        let cats = categories
            .iter()
            .find(|(c, _)| c == id)
            .map(|(_, c)| *c)
            .unwrap_or("");
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName={name}\nComment={comment}\nExec={}/{exec}\nIcon={icon}\nTerminal=false\nCategories={cats}\nMimeType={mime}\nNoDisplay=false\n",
            home.display()
        );
        let path = apps_dir.join(format!("{id}.desktop"));
        let _ = fs::write(&path, desktop);
        #[cfg(unix)]
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o755));
    }

    // Register DBus service for FileManager1
    let dbus_services_dir = home.join(".local/share/dbus-1/services");
    let _ = fs::create_dir_all(&dbus_services_dir);
    let file_manager_service = format!(
        "[D-BUS Service]\nName=org.freedesktop.FileManager1\nExec={}/.local/bin/babydra-explore\n",
        home.display()
    );
    let _ = fs::write(
        dbus_services_dir.join("org.freedesktop.FileManager1.service"),
        file_manager_service,
    );

    let _ = sudo.run("update-desktop-database", &[apps_dir.to_str().unwrap_or("")]);
    let _ = sudo.run(
        "xdg-mime",
        &[
            "default",
            "babydra-preview.desktop",
            "image/png",
            "image/jpeg",
            "image/gif",
            "image/webp",
            "image/bmp",
        ],
    );
    let _ = sudo.run(
        "xdg-mime",
        &["default", "babydra-explore.desktop", "inode/directory"],
    );

    log(
        LogLevel::Success,
        "Registered .desktop files, FileManager1 DBus service & MIME associations.".into(),
    );
    copied += 1;

    copied
}
