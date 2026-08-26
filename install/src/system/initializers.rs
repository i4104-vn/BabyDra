use super::get_user_local_bin;
use crate::models::{BinaryItem, BinaryLocation, GenericOptionItem, VariantItem};
use std::fs;
use std::path::{Path, PathBuf};

/// The greeter installs to `/usr/bin` (greetd runs it as the `greeter` user);
/// everything else lands in `~/.local/bin`.
fn default_loc(name: &str) -> BinaryLocation {
    if name == "babydra-greeter" {
        BinaryLocation::SystemBin
    } else {
        BinaryLocation::UserLocalBin
    }
}

/// Resolves the install target path of a binary.
fn binary_target_path(name: &str, loc: &BinaryLocation) -> PathBuf {
    match loc {
        BinaryLocation::UserLocalBin => get_user_local_bin().join(name),
        BinaryLocation::SystemBin => PathBuf::from("/usr/bin").join(name),
    }
}

pub fn initial_binaries_list(workspace_root: &Path, source_dir: &Path) -> Vec<BinaryItem> {
    let mut discovered: Vec<(String, String, String, BinaryLocation)> = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    // 1. Scan crates/ directory in workspace_root if it exists
    let crates_dir = workspace_root.join("crates");
    if crates_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&crates_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = entry.file_name().to_string_lossy().to_string();
                    let cargo_toml = path.join("Cargo.toml");
                    let (name, desc) = parse_crate_cargo_toml(&cargo_toml, &dir_name);
                    let def_loc = default_loc(&name);
                    seen_names.insert(name.clone());
                    discovered.push((name, desc, format!("crates/{dir_name}"), def_loc));
                }
            }
        }
    }

    // 2. If crates/ was empty on disk (e.g. while on main branch), query git ls-tree from branches
    if discovered.is_empty() {
        for ref_target in &[
            "origin/release",
            "release",
            "origin/develop",
            "develop",
            "HEAD",
        ] {
            if let Ok(out) = std::process::Command::new("git")
                .current_dir(workspace_root)
                .args(["ls-tree", "--name-only", &format!("{ref_target}:crates")])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .output()
            {
                if out.status.success() {
                    let out_str = String::from_utf8_lossy(&out.stdout);
                    for line in out_str.lines() {
                        let name = line.trim().to_string();
                        if !name.is_empty() && !seen_names.contains(&name) {
                            let desc = default_crate_description(&name);
                            let def_loc = default_loc(&name);
                            seen_names.insert(name.clone());
                            discovered.push((
                                name.clone(),
                                desc,
                                format!("crates/{name}"),
                                def_loc,
                            ));
                        }
                    }
                    if !discovered.is_empty() {
                        break;
                    }
                }
            }
        }
    }

    // 3. Also check source_dir (target/release) for any extra babydra-* binaries
    if source_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(source_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("babydra-")
                        && !name.contains('.')
                        && !seen_names.contains(&name)
                    {
                        let desc = default_crate_description(&name);
                        let def_loc = default_loc(&name);
                        seen_names.insert(name.clone());
                        discovered.push((name.clone(), desc, format!("crates/{name}"), def_loc));
                    }
                }
            }
        }
    }

    // Sort deterministically: panel first, desktop second, then alphabetical, greeter last
    discovered.sort_by(|(a, _, _, _), (b, _, _, _)| {
        fn rank(s: &str) -> i32 {
            match s {
                "babydra-panel" => 0,
                "babydra-desktop" => 1,
                "babydra-switcher" => 2,
                "babydra-launcher" => 3,
                "babydra-settings" => 4,
                "babydra-explore" => 5,
                "babydra-keymap" => 6,
                "babydra-screenshot" => 7,
                "babydra-preview" => 8,
                "babydra-lock" => 9,
                "babydra-greeter" => 100,
                _ => 50,
            }
        }
        rank(a).cmp(&rank(b)).then_with(|| a.cmp(b))
    });

    discovered
        .into_iter()
        .map(|(name, desc, crate_path, def_loc)| {
            let src_file = source_dir.join(&name);
            let exists_in_src = src_file.is_file();
            let size = if exists_in_src {
                fs::metadata(&src_file).map(|m| m.len()).ok()
            } else {
                None
            };
            let exists_in_target = binary_target_path(&name, &def_loc).exists();

            BinaryItem {
                name,
                description: desc,
                crate_path,
                default_dest: def_loc,
                selected: true,
                exists_in_source: exists_in_src,
                source_size_bytes: size,
                exists_in_target,
            }
        })
        .collect()
}

fn parse_crate_cargo_toml(path: &Path, fallback_dir_name: &str) -> (String, String) {
    if path.is_file() {
        if let Ok(content) = fs::read_to_string(path) {
            let mut name = String::new();
            let mut desc = String::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("name =") && name.is_empty() {
                    name = trimmed
                        .trim_start_matches("name =")
                        .trim()
                        .trim_matches('"')
                        .to_string();
                } else if trimmed.starts_with("description =") && desc.is_empty() {
                    desc = trimmed
                        .trim_start_matches("description =")
                        .trim()
                        .trim_matches('"')
                        .to_string();
                }
            }
            let final_name = if !name.is_empty() {
                name
            } else {
                fallback_dir_name.to_string()
            };
            let final_desc = if !desc.is_empty() {
                desc
            } else {
                default_crate_description(&final_name)
            };
            return (final_name, final_desc);
        }
    }
    (
        fallback_dir_name.to_string(),
        default_crate_description(fallback_dir_name),
    )
}

fn default_crate_description(name: &str) -> String {
    match name {
        "babydra-panel" => "Core Desktop Island, Dock, Status Panel & Notification Bar".to_string(),
        "babydra-desktop" => {
            "Desktop Layer, Wallpaper, Desktop Icons & File Context Menu".to_string()
        }
        "babydra-switcher" => {
            "Alt-Tab Window Switcher with App Icons & Window Previews".to_string()
        }
        "babydra-screenshot" => {
            "Interactive Region, Active Window & Fullscreen Capture Tool".to_string()
        }
        "babydra-lock" => "Fast & Modern Desktop Lock Screen with PAM Authentication".to_string(),
        "babydra-launcher" => "Fast Application Grid Launcher & Live Fuzzy Search Menu".to_string(),
        "babydra-preview" => "Hardware-Accelerated Image & Media Quick-Viewer".to_string(),
        "babydra-settings" => {
            "Full System Settings & Control Center (GTK4 + Layer Shell)".to_string()
        }
        "babydra-keymap" => "Global Keyboard Shortcuts Daemon & Hotkey Manager".to_string(),
        "babydra-explore" => "Modern GTK4 File & Directory Explorer with Quick Actions".to_string(),
        "babydra-greeter" => "Display Manager & Login Greeter UI for greetd / cage".to_string(),
        _ => {
            let clean = name.strip_prefix("babydra-").unwrap_or(name);
            format!("BabyDra {} component", clean)
        }
    }
}

pub fn update_binaries_status(items: &mut [BinaryItem], source_dir: &Path) {
    for item in items.iter_mut() {
        let src_file = source_dir.join(&item.name);
        item.exists_in_source = src_file.is_file();
        item.source_size_bytes = if item.exists_in_source {
            fs::metadata(&src_file).map(|m| m.len()).ok()
        } else {
            None
        };
        item.exists_in_target = binary_target_path(&item.name, &item.default_dest).exists();
    }
}

/// Reads `variants/*/variant.toml` and builds selectable variant options.
/// The `default` variant is pre-selected; others start deselected.
pub fn initial_variant_options(workspace_root: &Path) -> Vec<VariantItem> {
    let variants_dir = workspace_root.join("variants");
    let mut items = Vec::new();

    if let Ok(entries) = fs::read_dir(&variants_dir) {
        for entry in entries.flatten() {
            let dir = entry.path();
            let toml_path = dir.join("variant.toml");
            if !toml_path.is_file() {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&toml_path) {
                if let Ok(table) = content.parse::<toml::Table>() {
                    let name = table
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    if name.is_empty() {
                        continue;
                    }
                    let theme = table
                        .get("theme")
                        .and_then(|v| v.as_str())
                        .unwrap_or("babydra-default")
                        .to_string();
                    let apps = table
                        .get("apps")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    items.push(VariantItem {
                        name,
                        theme,
                        apps,
                        selected: false,
                    });
                }
            }
        }
    }

    if items.is_empty() {
        items.push(VariantItem {
            name: "default".to_string(),
            theme: "babydra-default".to_string(),
            apps: Vec::new(),
            selected: true,
        });
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    if let Some(default) = items.iter_mut().find(|v| v.name == "default") {
        default.selected = true;
    }
    items
}

pub fn initial_package_options() -> Vec<GenericOptionItem> {
    vec![
        GenericOptionItem {
            id: "pacman_packages".to_string(),
            title: "1. Install Arch Linux Pacman Packages".to_string(),
            description: "Installs GTK4, layer-shell, labwc, pipewire, playerctl, ddcutil, greetd, cage, etc.".to_string(),
            detail: "sudo pacman -Syu --needed base-devel git pkgconf gtk4 gtk4-layer-shell labwc pipewire ddcutil greetd cage ...".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "install_yay".to_string(),
            title: "2. Install yay AUR Helper (if missing)".to_string(),
            description: "Clones and builds yay-bin from AUR if yay is not found on the system.".to_string(),
            detail: "git clone https://aur.archlinux.org/yay-bin.git /tmp/yay-bin && makepkg -si --noconfirm".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "aur_packages".to_string(),
            title: "3. Install AUR Packages via yay".to_string(),
            description: "Installs kitty, neovim, fastfetch, wlrctl, Segoe UI & Cascadia Code fonts.".to_string(),
            detail: "yay -S --noconfirm github-desktop fastfetch neovim awww kitty ttf-segoe-ui-variable wlrctl ...".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "build_wtype".to_string(),
            title: "4. Build wtype from source (if missing)".to_string(),
            description: "Compiles wtype with meson + ninja to ~/.local/bin/wtype for Alt-key release handling.".to_string(),
            detail: "git clone https://github.com/atx/wtype.git /tmp/wtype && meson setup build && ninja -C build".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "kernel_permissions".to_string(),
            title: "5. Configure i2c-dev, CPU Performance & Input Permissions".to_string(),
            description: "Loads i2c-dev module, configures CPU governor permissions, and adds user to input group for keymap daemon.".to_string(),
            detail: "Configures /etc/modules-load.d/i2c.conf, /etc/tmpfiles.d/babydra-perf.conf, and runs sudo usermod -aG input $USER.".to_string(),
            selected: true,
            requires_root: true,
        },
    ]
}

pub fn initial_varlib_options() -> Vec<GenericOptionItem> {
    vec![
        GenericOptionItem {
            id: "stage_binaries".to_string(),
            title: "1. Stage All Built Binaries to /var/lib/babydra/bin/".to_string(),
            description: "Copies all compiled binary executables into /var/lib/babydra/bin/ for system & greeter access.".to_string(),
            detail: "Central system binary storage accessible by all user sessions, display manager, and background daemons.".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "set_varlib_permissions".to_string(),
            title: "2. Configure /var/lib/babydra Permissions (chmod 777)".to_string(),
            description: "Sets read/write/execute permissions so unprivileged greeter & desktop users can access system resources.".to_string(),
            detail: "Prevents permission denied errors when greetd (user 'greeter') runs components.".to_string(),
            selected: true,
            requires_root: true,
        },
    ]
}

pub fn initial_configs_themes_options() -> Vec<GenericOptionItem> {
    vec![
        GenericOptionItem {
            id: "terminate_processes".to_string(),
            title: "1. Terminate Old Running Instances (killall)".to_string(),
            description: "Gracefully stops active panel, switcher, lock, and explore processes before overwriting.".to_string(),
            detail: "Prevents Linux 'Text file busy' (ETXTBSY) errors when updating existing executables.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "labwc_configs".to_string(),
            title: "2. Sync Labwc Compositor Configuration".to_string(),
            description: "Syncs autostart, rc.xml, scripts, and theme configuration to ~/.config/labwc/.".to_string(),
            detail: "Sets executable bit (0755) on ~/.config/labwc/autostart and scripts.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "desktop_entries".to_string(),
            title: "3. Register .desktop Entries & MIME Associations".to_string(),
            description: "Creates desktop entries for Preview, Settings, and Explore; binds image & folder MIME types.".to_string(),
            detail: "Runs update-desktop-database and xdg-mime default babydra-preview.desktop image/png ...".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "dotfiles_gtk_terminal".to_string(),
            title: "4. Sync GTK-3/4, Fontconfig, Kitty, Neovim & Fastfetch".to_string(),
            description: "Deploys settings.ini, fonts.conf, kitty terminal, neovim config, and fastfetch profile.".to_string(),
            detail: "Ensures consistent Segoe UI font rendering, theme tokens, and terminal styling.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "themes_icons_cursors".to_string(),
            title: "5. Extract & Install Themes, Icons & Cursors".to_string(),
            description: "Installs BabyDra GTK theme, extracts We10X icons, and Twilight/AOSP cursors to ~/.local/share/.".to_string(),
            detail: "Unpacks .tar archives into ~/.local/share/icons/ and ~/.local/share/themes/.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "gsettings_fontcache".to_string(),
            title: "6. Apply GNOME GSettings & Rebuild Font Cache".to_string(),
            description: "Applies interface fonts (Segoe UI 13), icon theme (We10X), cursor theme, and runs fc-cache -f.".to_string(),
            detail: "Sets org.gnome.desktop.interface font-name and updates fontconfig cache.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "restart_services".to_string(),
            title: "7. Reload Labwc & Launch babydra-panel Service".to_string(),
            description: "Executes labwc --reconfigure and spawns ~/.local/bin/babydra-panel in background.".to_string(),
            detail: "Immediately activates the new desktop panel and window manager settings.".to_string(),
            selected: true,
            requires_root: false,
        },
    ]
}

pub fn initial_display_manager_options() -> Vec<GenericOptionItem> {
    vec![
        GenericOptionItem {
            id: "greetd_config".to_string(),
            title: "1. Configure /etc/greetd/config.toml (cage + babydra-greeter)".to_string(),
            description: "Sets cage -s -- /usr/bin/babydra-greeter as the default greetd login session.".to_string(),
            detail: "Launches the GTK4 login greeter seamlessly inside a dedicated Wayland cage compositor.".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "mask_gettys".to_string(),
            title: "2. Mask Secondary VTs (tty2-6 gettys) to Eliminate Screen Flash".to_string(),
            description: "Stops and masks getty@tty2..6.service to keep login handover completely seamless.".to_string(),
            detail: "Prevents terminal login prompts from flashing for 1-2s during display manager -> desktop handover.".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "enable_greetd".to_string(),
            title: "3. Enable greetd.service on Boot (systemctl enable greetd)".to_string(),
            description: "Enables the greetd systemd service so BabyDra greeter starts on system startup.".to_string(),
            detail: "Runs sudo systemctl enable greetd.service.".to_string(),
            selected: true,
            requires_root: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_binaries_list_discovery() {
        let root = crate::system::find_workspace_root();
        let target_release = root.join("target/release");
        let list = initial_binaries_list(&root, &target_release);
        assert!(!list.is_empty(), "Binaries list must not be empty");
        assert!(list.iter().any(|b| b.name == "babydra-panel"));
        assert!(list.iter().any(|b| b.name == "babydra-keymap"));
        assert!(list.iter().any(|b| b.name == "babydra-greeter"));
    }

    #[test]
    fn test_default_crate_description() {
        assert_eq!(
            default_crate_description("babydra-keymap"),
            "Global Keyboard Shortcuts Daemon & Hotkey Manager"
        );
        assert_eq!(
            default_crate_description("babydra-custom"),
            "BabyDra custom component"
        );
    }
}
