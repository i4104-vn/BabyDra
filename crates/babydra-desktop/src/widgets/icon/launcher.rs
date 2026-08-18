//! Launching helpers for desktop files and shortcuts.

use babydra_core::models::explore::{FileEntry, FileType};
use babydra_core::services::apps::parse_desktop_file;
use gtk4::gio::prelude::AppInfoExt;

/// Launches a file entry: executes .desktop app, opens directory in file manager, or launches default app for file.
pub fn launch_entry(entry: &FileEntry) {
    if entry.file_type == FileType::Directory {
        let path_str = entry.path.to_string_lossy();
        if std::process::Command::new("babydra-explore")
            .arg(&*path_str)
            .spawn()
            .is_err()
        {
            let uri = format!("file://{}", path_str);
            let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                &uri,
                gtk4::gio::AppLaunchContext::NONE,
            );
        }
        return;
    }

    // Check if .desktop application entry
    if entry
        .path
        .extension()
        .map(|e| e == "desktop")
        .unwrap_or(false)
    {
        if let Some(app_info) = gtk4::gio::DesktopAppInfo::from_filename(&entry.path) {
            if app_info.launch(&[], gtk4::gio::AppLaunchContext::NONE).is_ok() {
                return;
            }
        }

        if let Some(app) = parse_desktop_file(&entry.path) {
            let clean_exec = app
                .exec
                .split_whitespace()
                .filter(|a| !a.starts_with('%'))
                .collect::<Vec<&str>>()
                .join(" ");

            if !clean_exec.is_empty() {
                let _ = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("{} &", clean_exec))
                    .spawn();
                return;
            }
        }
    }

    let uri = format!("file://{}", entry.path.to_string_lossy());
    let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
}
