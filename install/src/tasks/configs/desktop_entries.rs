use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::models::{BinaryItem, LogLevel};
use crate::system::{binary_target_path, get_user_home, SudoSession};

/// Installs source-owned desktop entries and generates entries explicitly
/// requested by `export_desktop` in the branch manifest.
///
/// Not every binary is a graphical application, so the installer must not
/// infer a `.desktop` file from a discovered executable. The manifest flag is
/// false by default and only selected binaries with that flag are exported.
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
    let mut installed = 0;
    let mut desktop_entries_installed = 0;
    let mut source_desktop_entries_installed = 0;
    let mut mime_associations = Vec::new();

    let source_entries = find_files(workspace_root, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("desktop")
    });
    let source_entry_names = source_entries
        .iter()
        .filter_map(|path| path.file_name().and_then(|name| name.to_str()))
        .map(str::to_owned)
        .collect::<HashSet<_>>();

    if source_entries.is_empty() {
        log(
            LogLevel::Info,
            "No source-owned .desktop entries declared by this branch.".into(),
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
                desktop_entries_installed += 1;
                source_desktop_entries_installed += 1;
                mime_associations
                    .extend(desktop_mime_types(&file_name.to_string_lossy(), &content));
            }
        }
        log(
            LogLevel::Info,
            format!("Installed {source_desktop_entries_installed} source-owned desktop entries."),
        );
    }

    let mut generated_desktop_entries = 0;
    for binary in binaries
        .iter()
        .filter(|binary| should_generate_desktop_entry(binary, &source_entry_names))
    {
        let file_name = format!("{}.desktop", binary.name);
        let destination = apps_dir.join(&file_name);
        let content = generated_desktop_entry(binary);
        if write_desktop_file(&destination, &content).is_ok() {
            installed += 1;
            desktop_entries_installed += 1;
            generated_desktop_entries += 1;
        }
    }
    if generated_desktop_entries > 0 {
        log(
            LogLevel::Info,
            format!(
                "Generated {generated_desktop_entries} desktop entries requested by workspace.toml."
            ),
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

    if desktop_entries_installed > 0 {
        let _ = sudo.run(
            "update-desktop-database",
            &[apps_dir.to_str().unwrap_or_default()],
        );
    }
    for (desktop_id, mime_types) in mime_associations {
        for mime in mime_types {
            let _ = sudo.run("xdg-mime", &["default", &desktop_id, &mime]);
        }
    }

    if installed > 0 {
        log(
            LogLevel::Success,
            format!(
                "Registered {installed} desktop/DBus integration file(s) and MIME associations."
            ),
        );
    } else {
        log(
            LogLevel::Info,
            "No desktop or D-Bus integration files declared by this branch.".into(),
        );
    }
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

fn should_generate_desktop_entry(
    binary: &BinaryItem,
    source_entry_names: &HashSet<String>,
) -> bool {
    binary.selected
        && binary.export_desktop
        && !source_entry_names.contains(&format!("{}.desktop", binary.name))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::BinaryLocation;

    fn test_binary(name: &str, selected: bool, export_desktop: bool) -> BinaryItem {
        BinaryItem {
            name: name.to_owned(),
            source_name: name.to_owned(),
            description: "Test application".to_owned(),
            crate_path: "test".to_owned(),
            default_dest: BinaryLocation::UserLocalBin,
            export_desktop,
            selected,
            exists_in_source: true,
            source_size_bytes: None,
            exists_in_target: false,
        }
    }

    #[test]
    fn generates_entries_only_for_selected_exported_binaries() {
        let source_entry_names = HashSet::new();

        assert!(should_generate_desktop_entry(
            &test_binary("graphical-app", true, true),
            &source_entry_names
        ));
        assert!(!should_generate_desktop_entry(
            &test_binary("daemon", true, false),
            &source_entry_names
        ));
        assert!(!should_generate_desktop_entry(
            &test_binary("unselected-app", false, true),
            &source_entry_names
        ));

        let source_entry_names = ["graphical-app.desktop".to_owned()].into_iter().collect();
        assert!(!should_generate_desktop_entry(
            &test_binary("graphical-app", true, true),
            &source_entry_names
        ));
    }

    #[test]
    fn does_not_register_anything_when_source_has_no_desktop_entry() {
        let root = std::env::temp_dir().join(format!(
            "babydra_desktop_entries_test_{}_empty",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();

        let sudo = SudoSession::new(None);
        let mut messages = Vec::new();
        let registered = register_desktop_entries(&root, &[], &sudo, |_, message| {
            messages.push(message);
        });

        assert_eq!(registered, 0);
        assert!(messages
            .iter()
            .any(|message| message.contains("No source-owned .desktop entries")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ignores_desktop_files_inside_build_output() {
        let root = std::env::temp_dir().join(format!(
            "babydra_desktop_entries_test_{}_target",
            std::process::id()
        ));
        fs::create_dir_all(root.join("target/release")).unwrap();
        fs::write(root.join("configs.desktop"), "[Desktop Entry]\n").unwrap();
        fs::write(
            root.join("target/release/generated.desktop"),
            "[Desktop Entry]\n",
        )
        .unwrap();

        let entries = find_files(&root, |path| {
            path.extension().and_then(|ext| ext.to_str()) == Some("desktop")
        });

        assert_eq!(entries, vec![root.join("configs.desktop")]);

        let _ = fs::remove_dir_all(root);
    }
}
