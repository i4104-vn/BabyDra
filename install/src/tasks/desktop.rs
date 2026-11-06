use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::core::manifest::InstallManifest;
use crate::models::LogLevel;
use crate::runtime::{get_user_home, SudoSession};

/// Installs desktop entries located in the branch `desktops/` folder,
/// installs custom MIME XML definition packages,
/// and configures declared MIME associations.
pub fn register_desktop_entries<F>(
    workspace_root: &Path,
    manifest: &InstallManifest,
    sudo: &SudoSession,
    log: F,
) -> usize
where
    F: FnMut(LogLevel, String),
{
    register_desktop_entries_at(workspace_root, &get_user_home(), manifest, sudo, log)
}

pub fn register_desktop_entries_at<F>(
    workspace_root: &Path,
    home: &Path,
    manifest: &InstallManifest,
    sudo: &SudoSession,
    mut log: F,
) -> usize
where
    F: FnMut(LogLevel, String),
{
    let apps_dir = home.join(".local/share/applications");
    let mut installed = 0;
    let mut desktop_entries_installed = 0;
    let mut mime_associations = Vec::new();

    // 1. Install desktop entries from desktops/ directory
    let desktops_dir = manifest
        .desktop
        .entries_dir
        .as_deref()
        .map(|d| workspace_root.join(d))
        .unwrap_or_else(|| {
            let assets_desktop = workspace_root.join("assets").join("desktop");
            if assets_desktop.is_dir() {
                assets_desktop
            } else {
                workspace_root.join("desktops")
            }
        });

    let desktop_files = if desktops_dir.is_dir() {
        find_files(&desktops_dir, |path| {
            path.extension().and_then(|ext| ext.to_str()) == Some("desktop")
        })
    } else {
        find_files(workspace_root, |path| {
            path.extension().and_then(|ext| ext.to_str()) == Some("desktop")
        })
    };

    if desktop_files.is_empty() {
        log(
            LogLevel::Info,
            "No .desktop entries found in desktops directory.".into(),
        );
    } else {
        for source in desktop_files {
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
                mime_associations
                    .extend(desktop_mime_types(&file_name.to_string_lossy(), &content));
            }
        }
        log(
            LogLevel::Info,
            format!(
                "Installed {desktop_entries_installed} desktop entries from desktops directory."
            ),
        );
    }

    // 2. Install any custom MIME XML packages
    let mime_packages_dir = home.join(".local/share/mime/packages");
    let mut custom_mime_installed = 0;
    for rel_dir in &manifest.desktop.mime_packages {
        let candidate_dir = workspace_root.join(rel_dir);
        if !candidate_dir.is_dir() {
            continue;
        }
        for xml_file in find_files(&candidate_dir, |path| {
            path.extension().and_then(|ext| ext.to_str()) == Some("xml")
        }) {
            let Some(file_name) = xml_file.file_name() else {
                continue;
            };
            let destination = mime_packages_dir.join(file_name);
            let Ok(content) = fs::read_to_string(&xml_file) else {
                continue;
            };
            let _ = fs::create_dir_all(&mime_packages_dir);
            if fs::write(&destination, content).is_ok() {
                installed += 1;
                custom_mime_installed += 1;
            }
        }
    }
    if custom_mime_installed > 0 {
        let mime_root = home.join(".local/share/mime");
        let _ = sudo.run(
            "update-mime-database",
            &[mime_root.to_str().unwrap_or_default()],
        );
        log(
            LogLevel::Info,
            format!("Installed {custom_mime_installed} custom MIME definition package(s)."),
        );
    }

    // 3. Include manifest-declared MIME associations from workspace.toml
    for (mime_type, desktop_id) in &manifest.mime {
        mime_associations.push((desktop_id.clone(), vec![mime_type.clone()]));
    }
    for (desktop_id, mime_types) in &manifest.mime_associations {
        mime_associations.push((desktop_id.clone(), mime_types.clone()));
    }

    // 4. D-Bus activation files
    if manifest.desktop.dbus_services {
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
    }

    // 4b. Install BabyDra XDG Desktop Portal configs & service
    let portal_dir = home.join(".local/share/xdg-desktop-portal/portals");
    let _ = fs::create_dir_all(&portal_dir);
    let portal_src = workspace_root.join("assets/configs/portals/babydra.portal");
    if portal_src.is_file() {
        let _ = fs::copy(&portal_src, portal_dir.join("babydra.portal"));
        let _ = sudo.run_root_quiet(&["mkdir", "-p", "/usr/share/xdg-desktop-portal/portals"]);
        if let Some(s) = portal_src.to_str() {
            let _ = sudo.run_root_quiet(&["cp", s, "/usr/share/xdg-desktop-portal/portals/babydra.portal"]);
        }
        log(
            LogLevel::Success,
            "Installed babydra.portal definition".into(),
        );
        installed += 1;
    }

    // Portal configuration files (portals.conf, labwc-portals.conf)
    let portal_cfg_user = home.join(".config/xdg-desktop-portal");
    let _ = fs::create_dir_all(&portal_cfg_user);
    let portal_conf_src = workspace_root.join("assets/configs/xdg-desktop-portal");
    if portal_conf_src.is_dir() {
        if let Ok(entries) = fs::read_dir(&portal_conf_src) {
            for entry in entries.flatten() {
                let p = entry.path();
                if let Some(file_name) = p.file_name() {
                    let _ = fs::copy(&p, portal_cfg_user.join(file_name));
                    let _ = sudo.run_root_quiet(&["mkdir", "-p", "/etc/xdg/xdg-desktop-portal"]);
                    if let Some(s) = p.to_str() {
                        let dest = format!("/etc/xdg/xdg-desktop-portal/{}", file_name.to_string_lossy());
                        let _ = sudo.run_root_quiet(&["cp", s, &dest]);
                    }
                }
            }
        }
    }

    // Environment portal config (GTK_USE_PORTAL)
    let env_dir = home.join(".config/environment.d");
    let _ = fs::create_dir_all(&env_dir);
    let env_src = workspace_root.join("assets/configs/environment.d/portal.conf");
    if env_src.is_file() {
        let _ = fs::copy(&env_src, env_dir.join("portal.conf"));
        let _ = sudo.run_root_quiet(&["mkdir", "-p", "/etc/environment.d"]);
        if let Some(s) = env_src.to_str() {
            let _ = sudo.run_root_quiet(&["cp", s, "/etc/environment.d/portal.conf"]);
        }
    }

    let user_bin = crate::runtime::get_user_local_bin();
    let portal_service_content = format!(
        "[D-BUS Service]\nName=org.freedesktop.impl.portal.desktop.babydra\nExec={}/babydra-filedialog --portal\n",
        user_bin.display()
    );
    let dbus_dir = home.join(".local/share/dbus-1/services");
    let _ = fs::create_dir_all(&dbus_dir);
    if fs::write(dbus_dir.join("org.freedesktop.impl.portal.desktop.babydra.service"), &portal_service_content).is_ok() {
        installed += 1;
    }

    // 5. Update desktop database
    if desktop_entries_installed > 0 {
        let _ = sudo.run(
            "update-desktop-database",
            &[apps_dir.to_str().unwrap_or_default()],
        );
    }

    // 6. Bind default MIME associations
    let mut bound = HashSet::new();
    for (desktop_id, mime_types) in mime_associations {
        for mime in mime_types {
            if bound.insert((desktop_id.clone(), mime.clone())) {
                let _ = sudo.run("xdg-mime", &["default", &desktop_id, &mime]);
            }
        }
    }

    if installed > 0 {
        log(
            LogLevel::Success,
            format!(
                "Registered {installed} desktop/MIME/DBus integration file(s) and associations."
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

fn collect_files<F>(dir: &Path, found: &mut Vec<PathBuf>, predicate: F)
where
    F: Fn(&Path) -> bool + Copy,
{
    let Ok(entries) = fs::read_dir(dir) else {
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
pub mod tests {
    use super::*;

    #[test]
    fn installs_desktop_entries_from_desktops_directory() {
        let root = std::env::temp_dir().join(format!(
            "babydra_desktop_entries_test_{}_desktops",
            std::process::id()
        ));
        let desktops = root.join("desktops");
        fs::create_dir_all(&desktops).unwrap();
        fs::write(
            desktops.join("babydra-notepad.desktop"),
            "[Desktop Entry]\nType=Application\nName=Notepad\nExec=babydra-notepad\nMimeType=text/plain;\n",
        )
        .unwrap();

        let manifest = InstallManifest::default();
        let sudo = SudoSession::new(None);
        let mut messages = Vec::new();
        let registered = register_desktop_entries_at(&root, &root, &manifest, &sudo, |_, message| {
            messages.push(message);
        });

        assert_eq!(registered, 1);
        assert!(messages
            .iter()
            .any(|m| m.contains("from desktops directory")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn installs_custom_mime_packages_and_manifest_associations() {
        let root = std::env::temp_dir().join(format!(
            "babydra_desktop_entries_test_{}_custom_mime",
            std::process::id()
        ));
        let mime_dir = root.join("desktops/mime");
        fs::create_dir_all(&mime_dir).unwrap();
        fs::write(
            mime_dir.join("x-babydra.xml"),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><mime-info></mime-info>",
        )
        .unwrap();

        let mut manifest = InstallManifest::default();
        manifest.mime.insert(
            "application/x-babydra".to_string(),
            "babydra-notepad.desktop".to_string(),
        );

        let sudo = SudoSession::new(None);
        let mut messages = Vec::new();
        let registered = register_desktop_entries_at(&root, &root, &manifest, &sudo, |_, message| {
            messages.push(message);
        });

        assert_eq!(registered, 1);
        assert!(messages
            .iter()
            .any(|m| m.contains("custom MIME definition package")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn does_not_register_anything_when_source_has_no_desktop_entry() {
        let root = std::env::temp_dir().join(format!(
            "babydra_desktop_entries_test_{}_empty",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();

        let manifest = InstallManifest::default();
        let sudo = SudoSession::new(None);
        let mut messages = Vec::new();
        let registered = register_desktop_entries_at(&root, &root, &manifest, &sudo, |_, message| {
            messages.push(message);
        });

        assert_eq!(registered, 0);
        assert!(messages
            .iter()
            .any(|message| message.contains("No .desktop entries found")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ignores_desktop_files_inside_build_output() {
        let root = std::env::temp_dir().join(format!(
            "babydra_desktop_entries_test_{}_target",
            std::process::id()
        ));
        fs::create_dir_all(root.join("target/release")).unwrap();
        fs::write(
            root.join("target/release/generated.desktop"),
            "[Desktop Entry]\n",
        )
        .unwrap();

        let desktops = root.join("desktops");
        fs::create_dir_all(&desktops).unwrap();
        fs::write(desktops.join("configs.desktop"), "[Desktop Entry]\n").unwrap();

        let entries = find_files(&desktops, |path| {
            path.extension().and_then(|ext| ext.to_str()) == Some("desktop")
        });

        assert_eq!(entries, vec![desktops.join("configs.desktop")]);

        let _ = fs::remove_dir_all(root);
    }
}
