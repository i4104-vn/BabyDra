//! User configuration files, caches, desktop entries, and theme assets cleanup.

use std::fs;
use std::process::Command;
use std::sync::mpsc::Sender;

/// Cleans user configuration files, caches, desktop entries, and theme assets.
pub fn clean_user_files(sender: &Sender<String>) {
    let _ = sender.send("[4/6] Cleaning user configuration, caches, and binaries...".into());
    if let Some(home) = dirs::home_dir() {
        // 1. ~/.babydra
        let babydra_dir = home.join(".babydra");
        if babydra_dir.exists() {
            let _ = fs::remove_dir_all(&babydra_dir);
            let _ = sender.send("  Removed ~/.babydra".into());
        }

        // 2. ~/.cache/babydra
        let babydra_cache = home.join(".cache/babydra");
        if babydra_cache.exists() {
            let _ = fs::remove_dir_all(&babydra_cache);
            let _ = sender.send("  Removed ~/.cache/babydra".into());
        }

        // 3. ~/.config/labwc
        let labwc_dir = home.join(".config/labwc");
        if labwc_dir.exists() {
            let _ = fs::remove_dir_all(&labwc_dir);
            let _ = sender.send("  Removed ~/.config/labwc".into());
        }

        // 4. ~/.local/share/themes/BabyDra
        let theme_dir = home.join(".local/share/themes/BabyDra");
        if theme_dir.exists() {
            let _ = fs::remove_dir_all(&theme_dir);
            let _ = sender.send("  Removed ~/.local/share/themes/BabyDra".into());
        }

        // 5. Binaries in ~/.local/bin
        let local_bin = home.join(".local/bin");
        if let Ok(entries) = fs::read_dir(&local_bin) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("babydra-") || name == "wtype" {
                    let _ = fs::remove_file(entry.path());
                    let _ = sender.send(format!("  Removed ~/.local/bin/{}", name));
                }
            }
        }

        // 6. Desktop files in ~/.local/share/applications
        let app_dir = home.join(".local/share/applications");
        if let Ok(entries) = fs::read_dir(&app_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("babydra-") && name.ends_with(".desktop") {
                    let _ = fs::remove_file(entry.path());
                    let _ = sender.send(format!("  Removed desktop entry {}", name));
                }
            }
        }

        // 7. FileManager1 dbus service
        let dbus_svc =
            home.join(".local/share/dbus-1/services/org.freedesktop.FileManager1.service");
        if dbus_svc.exists() {
            let _ = fs::remove_file(&dbus_svc);
            let _ = sender.send("  Removed FileManager1 DBus service".into());
        }

        // 8. GTK, fontconfig, and CLI configs
        let gtk3_ini = home.join(".config/gtk-3.0/settings.ini");
        if gtk3_ini.exists() {
            let _ = fs::remove_file(gtk3_ini);
        }
        let gtk4_ini = home.join(".config/gtk-4.0/settings.ini");
        if gtk4_ini.exists() {
            let _ = fs::remove_file(gtk4_ini);
        }
        let fonts_conf = home.join(".config/fontconfig/fonts.conf");
        if fonts_conf.exists() {
            let _ = fs::remove_file(fonts_conf);
        }

        let fastfetch_dir = home.join(".config/fastfetch");
        if fastfetch_dir.exists() {
            let _ = fs::remove_dir_all(fastfetch_dir);
        }
        let kitty_dir = home.join(".config/kitty");
        if kitty_dir.exists() {
            let _ = fs::remove_dir_all(kitty_dir);
        }
        let nvim_dir = home.join(".config/nvim");
        if nvim_dir.exists() {
            let _ = fs::remove_dir_all(nvim_dir);
        }

        // Reset GSettings
        for key in &[
            "font-name",
            "document-font-name",
            "monospace-font-name",
            "icon-theme",
            "cursor-theme",
        ] {
            let _ = Command::new("gsettings")
                .args(["reset", "org.gnome.desktop.interface", key])
                .output();
        }
        let _ = sender.send("  Reset GNOME/GTK desktop interface settings to default".into());

        // Update desktop database
        let _ = Command::new("update-desktop-database")
            .arg(app_dir)
            .output();
    }
}
