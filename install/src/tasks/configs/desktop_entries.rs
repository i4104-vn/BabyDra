use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::models::{BinaryItem, LogLevel};
use crate::system::{binary_target_path, get_user_home, SudoSession};

/// Installs source-owned desktop entries. When a branch does not provide
/// entries yet, a generic entry is generated for each selected executable so
/// newly added binaries are still discoverable without an installer edit.
pub fn register_desktop_entries<F>(
    workspace_root: &Path,
    binaries: &[BinaryItem],
    sudo: &SudoSession,
    mut log: F,
) -> usize
where
    F: FnMut(LogLevel, String),
{
    let home = get_user_home();
    let apps_dir = home.join(".local/share/applications");
    let _ = fs::create_dir_all(&apps_dir);
    let mut installed = 0;
    let mut mime_associations = Vec::new();

    let source_entries = find_files(workspace_root, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("desktop")
    });

    if source_entries.is_empty() {
        for binary in binaries.iter().filter(|binary| binary.selected) {
            let content = generated_desktop_entry(binary);
            let path = apps_dir.join(format!("{}.desktop", binary.name));
            if write_desktop_file(&path, &content).is_ok() {
                installed += 1;
            }
        }
        log(
            LogLevel::Info,
            format!("Generated {installed} desktop entries from discovered binaries."),
        );
    } else {
        for source in source_entries {
            let Some(file_name) = source.file_name() else {
                continue;
            };
            let destination = apps_dir.join(file_name);
            let Ok(content) = fs::read_to_string(&source) else {
                log(
                    LogLevel::Warn,
                    format!("Could not read desktop entry {}.", source.display()),
                );
                continue;
            };
            if write_desktop_file(&destination, &content).is_ok() {
                installed += 1;
                mime_associations
                    .extend(desktop_mime_types(&file_name.to_string_lossy(), &content));
            }
        }
        log(
            LogLevel::Info,
            format!("Installed {installed} source-owned desktop entries."),
        );
    }

    // D-Bus activation files are source-owned too. This handles any service
    // name (not only FileManager1) and keeps desktop integration extensible.
    let dbus_dir = home.join(".local/share/dbus-1/services");
    for source in find_files(workspace_root, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("service")
            && fs::read_to_string(path)
                .map(|content| content.contains("[D-BUS Service]"))
                .unwrap_or(false)
    }) {
        let Some(file_name) = source.file_name() else {
            continue;
        };
        if let Ok(content) = fs::read_to_string(&source) {
            let destination = dbus_dir.join(file_name);
            let _ = fs::create_dir_all(&dbus_dir);
            if fs::write(destination, content).is_ok() {
                installed += 1;
            }
        }
    }

    let _ = sudo.run(
        "update-desktop-database",
        &[apps_dir.to_str().unwrap_or_default()],
    );
    for (desktop_id, mime_types) in mime_associations {
        for mime in mime_types {
            let _ = sudo.run("xdg-mime", &["default", &desktop_id, &mime]);
        }
    }

    log(
        LogLevel::Success,
        format!("Registered {installed} desktop/DBus integration file(s) and MIME associations."),
    );
    usize::from(installed > 0)
}

fn generated_desktop_entry(binary: &BinaryItem) -> String {
    let title = display_name(&binary.name);
    let executable = binary_target_path(&binary.name, &binary.default_dest);
    format!(
        "[Desktop Entry]\nType=Application\nName={title}\nComment={}\nExec={}\nIcon=application-x-executable\nTerminal=false\nCategories=Utility;\nNoDisplay=false\n",
        binary.description,
        executable.display()
    )
}

fn display_name(name: &str) -> String {
    name.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn write_desktop_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    Ok(())
}

fn desktop_mime_types(desktop_id: &str, content: &str) -> Vec<(String, Vec<String>)> {
    let mime_types = value_for_key(content, "MimeType")
        .unwrap_or_default()
        .split(';')
        .filter(|mime| !mime.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if mime_types.is_empty() {
        Vec::new()
    } else {
        vec![(desktop_id.to_owned(), mime_types)]
    }
}

fn value_for_key(content: &str, key: &str) -> Option<String> {
    content.lines().find_map(|line| {
        line.strip_prefix(&format!("{key}="))
            .or_else(|| line.strip_prefix(&format!("{key} =")))
            .map(str::trim)
            .map(str::to_owned)
    })
}

fn find_files<F>(root: &Path, predicate: F) -> Vec<PathBuf>
where
    F: Fn(&Path) -> bool + Copy,
{
    let mut found = Vec::new();
    collect_files(root, &mut found, predicate);
    found
}

fn collect_files<F>(root: &Path, found: &mut Vec<PathBuf>, predicate: F)
where
    F: Fn(&Path) -> bool + Copy,
{
    if !root.is_dir() {
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let skip = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    matches!(name, ".git" | "target" | "branches" | "node_modules")
                });
            if !skip {
                collect_files(&path, found, predicate);
            }
        } else if predicate(&path) {
            found.push(path);
        }
    }
}
