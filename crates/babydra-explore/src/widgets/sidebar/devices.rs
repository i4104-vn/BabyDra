//! Removable devices (USB, external drives) monitoring and mounting for Explore sidebar.

use gio::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct DeviceItem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub path: PathBuf,
    pub mount: Option<gio::Mount>,
}

/// Discovers currently mounted removable drives, USB flash drives, and external storage.
pub fn get_mounted_devices() -> Vec<DeviceItem> {
    let mut devices = Vec::new();
    let vm = gio::VolumeMonitor::get();

    // 1. Scan Gio VolumeMonitor mounts
    for mount in vm.mounts() {
        if let Some(path) = mount.root().path() {
            // Exclude root "/" and user home
            if path == PathBuf::from("/") || path == glib::home_dir() {
                continue;
            }

            let path_str = path.to_string_lossy();
            let is_media = path_str.starts_with("/media") || path_str.starts_with("/run/media");
            if mount.can_unmount() || is_media {
                let name = mount.name().to_string();
                let id = format!("device_{}", path_str.replace('/', "_"));
                devices.push(DeviceItem {
                    id,
                    name,
                    icon: "drive-harddisk".to_string(),
                    path,
                    mount: Some(mount),
                });
            }
        }
    }

    // 2. Also check file system for /run/media/$USER, /media/$USER, /run/media, /media
    let user = glib::user_name().to_string_lossy().to_string();
    let scan_dirs = [
        format!("/run/media/{}", user),
        format!("/media/{}", user),
        "/run/media".to_string(),
        "/media".to_string(),
    ];

    for scan_dir in scan_dirs {
        let p = PathBuf::from(scan_dir);
        if p.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&p) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() && path != glib::home_dir() {
                        if !devices.iter().any(|d| d.path == path) {
                            let name = path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "USB Drive".to_string());
                            let id = format!("device_{}", path.to_string_lossy().replace('/', "_"));
                            devices.push(DeviceItem {
                                id,
                                name,
                                icon: "drive-harddisk".to_string(),
                                path,
                                mount: None,
                            });
                        }
                    }
                }
            }
        }
    }

    devices
}

/// Automatically mounts unmounted removable volumes when detected.
pub fn auto_mount_volumes(on_mounted: impl Fn() + 'static) {
    let vm = gio::VolumeMonitor::get();
    let on_mounted = Rc::new(on_mounted);

    for volume in vm.volumes() {
        if volume.can_mount() && volume.get_mount().is_none() {
            let on_m = on_mounted.clone();
            volume.mount(
                gio::MountMountFlags::NONE,
                Option::<&gio::MountOperation>::None,
                gio::Cancellable::NONE,
                move |res| {
                    if res.is_ok() {
                        on_m();
                    }
                },
            );
        }
    }
}

/// Sets up real-time volume monitor signals (connects USB plug/unplug, mount/unmount).
pub fn setup_device_monitor(on_change: impl Fn() + 'static) {
    let vm = gio::VolumeMonitor::get();
    let on_change = Rc::new(on_change);

    // Mount added
    {
        let cb = on_change.clone();
        vm.connect_mount_added(move |_, _| {
            cb();
        });
    }

    // Mount removed
    {
        let cb = on_change.clone();
        vm.connect_mount_removed(move |_, _| {
            cb();
        });
    }

    // Mount changed
    {
        let cb = on_change.clone();
        vm.connect_mount_changed(move |_, _| {
            cb();
        });
    }

    // Volume added: auto mount and refresh
    {
        let cb = on_change.clone();
        vm.connect_volume_added(move |_, vol| {
            if vol.can_mount() && vol.get_mount().is_none() {
                let cb_inner = cb.clone();
                vol.mount(
                    gio::MountMountFlags::NONE,
                    Option::<&gio::MountOperation>::None,
                    gio::Cancellable::NONE,
                    move |res| {
                        if res.is_ok() {
                            cb_inner();
                        }
                    },
                );
            } else {
                cb();
            }
        });
    }

    // Volume removed
    {
        let cb = on_change.clone();
        vm.connect_volume_removed(move |_, _| {
            cb();
        });
    }

    // Drive connected
    {
        let cb = on_change.clone();
        vm.connect_drive_connected(move |_, _| {
            cb();
        });
    }

    // Drive disconnected
    {
        let cb = on_change;
        vm.connect_drive_disconnected(move |_, _| {
            cb();
        });
    }
}
