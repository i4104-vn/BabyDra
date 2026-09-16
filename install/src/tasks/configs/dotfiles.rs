use std::fs;
use std::path::Path;

use crate::models::LogLevel;
use crate::system::{copy_recursive, get_user_home};

pub fn sync_dotfiles<F>(workspace_root: &Path, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let configs_src = workspace_root.join("configs");
    let mut synced = Vec::new();
    if let Ok(entries) = fs::read_dir(&configs_src) {
        for entry in entries.flatten() {
            let source = entry.path();
            let name = entry.file_name();
            // Theme archives are deployed by the theme task, not as an
            // application config directory.
            if source.is_dir() && name.to_string_lossy() != "themes" {
                let destination = home.join(".config").join(&name);
                if copy_recursive(&source, &destination).is_ok() {
                    synced.push(name.to_string_lossy().to_string());
                }
            }
        }
    }

    // GTK/fontconfig consume these files from their own standard locations;
    // the source still remains the single labwc config tree.
    let labwc_src = configs_src.join("labwc");
    let gtk3 = home.join(".config/gtk-3.0");
    let gtk4 = home.join(".config/gtk-4.0");
    let fontconfig = home.join(".config/fontconfig");
    let _ = fs::create_dir_all(&gtk3);
    let _ = fs::create_dir_all(&gtk4);
    let _ = fs::create_dir_all(&fontconfig);
    if let Some(settings_ini) = existing_file(&labwc_src, "settings.ini") {
        let _ = fs::copy(&settings_ini, gtk3.join("settings.ini"));
        let _ = fs::copy(&settings_ini, gtk4.join("settings.ini"));
    }
    if let Some(fonts_conf) = existing_file(&labwc_src, "fonts.conf") {
        let _ = fs::copy(fonts_conf, fontconfig.join("fonts.conf"));
    }

    log(
        LogLevel::Success,
        format!(
            "Synced source-defined config directories: {}.",
            synced.join(", ")
        ),
    );
    copied += 1;

    copied
}

fn existing_file(directory: &Path, name: &str) -> Option<std::path::PathBuf> {
    let path = directory.join(name);
    path.is_file().then_some(path)
}
