use crate::services::utils::{get_home_dir, run_cmd};
use std::fs;
use std::path::Path;

/// Applies `appearance`.
pub fn apply_appearance(
    gtk_theme: &str,
    icon_theme: &str,
    cursor_theme: &str,
    cursor_size: u32,
) -> Result<(), String> {
    let size_str = cursor_size.to_string();

    let _ = run_cmd(&[
        "gsettings",
        "set",
        "org.gnome.desktop.interface",
        "gtk-theme",
        gtk_theme,
    ]);
    let _ = run_cmd(&[
        "gsettings",
        "set",
        "org.gnome.desktop.interface",
        "icon-theme",
        icon_theme,
    ]);
    let _ = run_cmd(&[
        "gsettings",
        "set",
        "org.gnome.desktop.interface",
        "cursor-theme",
        cursor_theme,
    ]);
    let _ = run_cmd(&[
        "gsettings",
        "set",
        "org.gnome.desktop.interface",
        "cursor-size",
        &size_str,
    ]);

    // Update labwc settings.ini (for GTK apps) and environment file (for Wayland cursor)
    let home = get_home_dir();

    // 1. Update settings.ini files (affects GTK apps)
    let ini_paths = vec![
        Path::new(&home).join(".config/labwc/settings.ini"),
        Path::new(&home).join("BabyDra/configs/labwc/settings.ini"),
    ];
    for path in &ini_paths {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                let mut new_content = String::new();
                for line in content.lines() {
                    if line.starts_with("gtk-icon-theme-name=") {
                        new_content.push_str(&format!("gtk-icon-theme-name={}\n", icon_theme));
                    } else if line.starts_with("gtk-cursor-theme-name=") {
                        new_content.push_str(&format!("gtk-cursor-theme-name={}\n", cursor_theme));
                    } else if line.starts_with("gtk-cursor-theme-size=") {
                        new_content.push_str(&format!("gtk-cursor-theme-size={}\n", cursor_size));
                    } else {
                        new_content.push_str(line);
                        new_content.push('\n');
                    }
                }
                let _ = fs::write(path, new_content);
            }
        }
    }

    // 2. Update labwc environment file (this is what labwc actually reads for XCURSOR)
    let env_path = Path::new(&home).join(".config/labwc/environment");
    let mut env_vars: Vec<(String, String)> = Vec::new();
    let mut has_xcursor_theme = false;
    let mut has_xcursor_size = false;

    if env_path.exists() {
        if let Ok(content) = fs::read_to_string(&env_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    env_vars.push((line.to_string(), String::new()));
                    continue;
                }
                if let Some((key, _)) = trimmed.split_once('=') {
                    match key {
                        "XCURSOR_THEME" => {
                            env_vars.push(("XCURSOR_THEME".to_string(), cursor_theme.to_string()));
                            has_xcursor_theme = true;
                        }
                        "XCURSOR_SIZE" => {
                            env_vars.push(("XCURSOR_SIZE".to_string(), size_str.clone()));
                            has_xcursor_size = true;
                        }
                        _ => {
                            env_vars.push((
                                key.to_string(),
                                trimmed.split_once('=').unwrap().1.to_string(),
                            ));
                        }
                    }
                } else {
                    env_vars.push((line.to_string(), String::new()));
                }
            }
        }
    }

    if !has_xcursor_theme {
        env_vars.push(("XCURSOR_THEME".to_string(), cursor_theme.to_string()));
    }
    if !has_xcursor_size {
        env_vars.push(("XCURSOR_SIZE".to_string(), size_str.clone()));
    }

    let mut env_content = String::new();
    for (key, val) in &env_vars {
        if val.is_empty() {
            // Comment or blank line preserved as-is
            env_content.push_str(key);
            env_content.push('\n');
        } else {
            env_content.push_str(&format!("{}={}\n", key, val));
        }
    }
    let _ = fs::write(&env_path, &env_content);

    // 3. Tell labwc to reconfigure (reloads environment + rc.xml)
    let _ = run_cmd(&["labwc", "--reconfigure"]);

    Ok(())
}
