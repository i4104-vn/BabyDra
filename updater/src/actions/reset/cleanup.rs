use crate::actions::runner::CommandRunner;
use crate::utils::fs::{remove_dir_all_if_exists, remove_file_if_exists};
use crate::utils::system::{
    get_applications_dir, get_babydra_dir, get_home_dir, get_local_bin_dir, gsettings_reset,
    update_desktop_db,
};
use std::fs;

pub fn clean_system_and_user_files(runner: &CommandRunner, is_dry_run: bool) -> Result<(), String> {
    let home = get_home_dir();
    let local_bin = get_local_bin_dir();
    let apps_dir = get_applications_dir();

    runner.step("[Step 3/6] Cleaning system-wide BabyDra directories and modules...");
    if !is_dry_run {
        runner.run_sudo("rm", &["-f", "/usr/bin/babydra-greeter"], None)?;
        runner.run_sudo(
            "rm",
            &["-rf", "/usr/share/babydra", "/var/lib/babydra"],
            None,
        )?;
        runner.run_sudo(
            "rm",
            &[
                "-f",
                "/etc/modules-load.d/i2c.conf",
                "/etc/tmpfiles.d/babydra-perf.conf",
            ],
            None,
        )?;
        runner.run_sudo(
            "rm",
            &[
                "-f",
                "/etc/modules-load.d/babydra-keymap.conf",
                "/etc/udev/rules.d/99-babydra-keymap.rules",
            ],
            None,
        )?;
    } else {
        runner.log("[dry-run] Would remove system files in /usr/share/babydra, /var/lib/babydra");
    }

    runner.step("[Step 4/6] Cleaning user configurations, caches, and installed binaries...");
    if !is_dry_run {
        if let Ok(entries) = fs::read_dir(&local_bin) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("babydra-") || name == "wtype" {
                    remove_file_if_exists(&entry.path());
                }
            }
        }

        remove_dir_all_if_exists(&get_babydra_dir());
        remove_dir_all_if_exists(&home.join(".cache/babydra"));
        remove_dir_all_if_exists(&home.join(".config/labwc"));
        remove_dir_all_if_exists(&home.join(".local/share/themes/BabyDra"));
        remove_file_if_exists(
            &home.join(".local/share/dbus-1/services/org.freedesktop.FileManager1.service"),
        );

        if let Ok(entries) = fs::read_dir(&apps_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("babydra-") {
                    remove_file_if_exists(&entry.path());
                }
            }
        }

        gsettings_reset("org.gnome.desktop.interface", "font-name");
        gsettings_reset("org.gnome.desktop.interface", "document-font-name");
        gsettings_reset("org.gnome.desktop.interface", "monospace-font-name");
        gsettings_reset("org.gnome.desktop.interface", "icon-theme");
        gsettings_reset("org.gnome.desktop.interface", "cursor-theme");
        update_desktop_db(&apps_dir);
    } else {
        runner.log(
            "[dry-run] Would clean ~/.babydra, ~/.config/labwc, desktop entries, reset gsettings",
        );
    }

    Ok(())
}
