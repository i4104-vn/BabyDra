//! Launching helpers for desktop files and shortcuts.

use babydra_core::models::explore::{FileEntry, FileType};
use babydra_core::services::apps::parse_desktop_file;

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
        if let Some(app) = parse_desktop_file(&entry.path) {
            let exec_cmd = app.exec.split_whitespace().collect::<Vec<&str>>();
            if let Some((prog, args)) = exec_cmd.split_first() {
                // Filter out desktop field codes (%f, %F, %u, %U, %i, %c, %k)
                let clean_args: Vec<&str> = args
                    .iter()
                    .filter(|a| !a.starts_with('%'))
                    .copied()
                    .collect();
                let _ = std::process::Command::new(prog).args(&clean_args).spawn();
                return;
            }
        }
    }

    let uri = format!("file://{}", entry.path.to_string_lossy());
    let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
}
