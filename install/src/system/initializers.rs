use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{BinaryItem, BinaryLocation, GenericOptionItem};
use super::{get_user_local_bin};

pub fn initial_binaries_list(source_dir: &Path) -> Vec<BinaryItem> {
    let raw_items = vec![
        (
            "babydra-panel",
            "Core Desktop Island, Dock, Status Panel & Notification Bar",
            "crates/babydra-panel",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-switcher",
            "Alt-Tab Window Switcher with App Icons & Window Previews",
            "crates/babydra-switcher",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-screenshot",
            "Interactive Region, Active Window & Fullscreen Capture Tool",
            "crates/babydra-screenshot",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-lock",
            "Fast & Modern Desktop Lock Screen with PAM Authentication",
            "crates/babydra-lock",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-launcher",
            "Fast Application Grid Launcher & Live Fuzzy Search Menu",
            "libs/babydra-launcher",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-preview",
            "Hardware-Accelerated Image & Media Quick-Viewer",
            "crates/babydra-preview",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-settings",
            "Full System Settings & Control Center (GTK4 + Layer Shell)",
            "crates/babydra-settings",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-explore",
            "Modern GTK4 File & Directory Explorer with Quick Actions",
            "crates/babydra-explore",
            BinaryLocation::UserLocalBin,
        ),
        (
            "babydra-greeter",
            "Display Manager & Login Greeter UI for greetd / cage",
            "crates/babydra-greeter",
            BinaryLocation::SystemBin,
        ),
    ];

    let user_bin = get_user_local_bin();

    raw_items
        .into_iter()
        .map(|(name, desc, crate_path, def_loc)| {
            let src_file = source_dir.join(name);
            let exists_in_src = src_file.is_file();
            let size = if exists_in_src {
                fs::metadata(&src_file).map(|m| m.len()).ok()
            } else {
                None
            };

            let target_path = match def_loc {
                BinaryLocation::UserLocalBin => user_bin.join(name),
                BinaryLocation::SystemBin => PathBuf::from("/usr/bin").join(name),
            };

            let exists_in_target = target_path.exists();

            let status_note = if exists_in_src {
                if exists_in_target {
                    "Installed (Ready to update)".to_string()
                } else {
                    "Available (New install)".to_string()
                }
            } else {
                "Missing in source folder".to_string()
            };

            BinaryItem {
                name: name.to_string(),
                description: desc.to_string(),
                crate_path: crate_path.to_string(),
                default_dest: def_loc,
                selected: exists_in_src,
                exists_in_source: exists_in_src,
                source_size_bytes: size,
                exists_in_target,
                status_note,
            }
        })
        .collect()
}

pub fn update_binaries_status(items: &mut [BinaryItem], source_dir: &Path) {
    let user_bin = get_user_local_bin();
    for item in items.iter_mut() {
        let src_file = source_dir.join(&item.name);
        item.exists_in_source = src_file.is_file();
        item.source_size_bytes = if item.exists_in_source {
            fs::metadata(&src_file).map(|m| m.len()).ok()
        } else {
            None
        };

        let target_path = match item.default_dest {
            BinaryLocation::UserLocalBin => user_bin.join(&item.name),
            BinaryLocation::SystemBin => PathBuf::from("/usr/bin").join(&item.name),
        };

        item.exists_in_target = target_path.exists();
        item.status_note = if item.exists_in_source {
            if item.exists_in_target {
                "Installed (Ready to update)".to_string()
            } else {
                "Available (New install)".to_string()
            }
        } else {
            "Missing in source folder".to_string()
        };
    }
}

pub fn initial_package_options() -> Vec<GenericOptionItem> {
    vec![
        GenericOptionItem {
            id: "pacman_packages".to_string(),
            title: "1. Install Arch Linux Pacman Packages".to_string(),
            description: "Installs GTK4, layer-shell, labwc, pipewire, playerctl, ddcutil, greetd, cage, etc.".to_string(),
            detail: "sudo pacman -Syu --needed base-devel git pkgconf gtk4 gtk4-layer-shell labwc pipewire ddcutil greetd cage ...".to_string(),
            selected: false,
            requires_root: true,
        },
        GenericOptionItem {
            id: "aur_packages".to_string(),
            title: "2. Install AUR Packages via yay".to_string(),
            description: "Installs kitty, neovim, fastfetch, wlrctl, Segoe UI & Cascadia Code fonts.".to_string(),
            detail: "yay -S --noconfirm github-desktop fastfetch neovim awww kitty ttf-segoe-ui-variable wlrctl ...".to_string(),
            selected: false,
            requires_root: false,
        },
        GenericOptionItem {
            id: "kernel_permissions".to_string(),
            title: "3. Configure i2c-dev & CPU Performance Permissions".to_string(),
            description: "Loads i2c-dev module for DDC/CI brightness and allows CPU governor switching.".to_string(),
            detail: "Configures /etc/modules-load.d/i2c.conf and /etc/tmpfiles.d/babydra-perf.conf for non-root CPU control.".to_string(),
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
            description: "Copies all 9 compiled binary executables into /var/lib/babydra/bin/ for system & greeter access.".to_string(),
            detail: "Central system binary storage accessible by all user sessions, display manager, and background daemons.".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "stage_wallpapers".to_string(),
            title: "2. Copy System Wallpapers to /var/lib/babydra/ & /usr/share/".to_string(),
            description: "Installs wallpaper.png as greeter_wallpaper.png and /usr/share/babydra/wallpaper.png.".to_string(),
            detail: "Provides the default wallpaper for the greeter login screen and the user desktop session.".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "stage_logos".to_string(),
            title: "3. Copy Brand Logos & Icons to /var/lib/babydra/".to_string(),
            description: "Installs logo.png and preview icons to /var/lib/babydra/logo.png and /usr/share/babydra/.".to_string(),
            detail: "Used by fastfetch, application launcher, greeter, and system notification overlays.".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "set_varlib_permissions".to_string(),
            title: "4. Configure /var/lib/babydra Permissions (chmod 777)".to_string(),
            description: "Sets read/write/execute permissions so unprivileged greeter & desktop users can access assets.".to_string(),
            detail: "Prevents permission denied errors when greetd (user 'greeter') loads wallpapers and icons.".to_string(),
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
