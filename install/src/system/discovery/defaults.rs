use super::manifest::InstallManifest;
use crate::models::GenericOptionItem;

pub fn initial_package_options(manifest: &InstallManifest) -> Vec<GenericOptionItem> {
    let pacman_summary = if manifest.pacman_packages.is_empty() {
        "no pacman packages declared by the source branch".to_owned()
    } else {
        manifest.pacman_packages.join(" ")
    };
    let aur_summary = if manifest.aur_packages.is_empty() {
        "no AUR packages declared by the source branch".to_owned()
    } else {
        manifest.aur_packages.join(" ")
    };
    let mut options = Vec::new();
    if !manifest.pacman_packages.is_empty() {
        options.push(GenericOptionItem {
            id: "pacman_packages".to_string(),
            title: "1. Install Arch Linux Pacman Packages".to_string(),
            description: "Installs the pacman dependencies declared by the selected source branch."
                .to_string(),
            detail: format!("pacman -Syu --needed {pacman_summary}"),
            selected: true,
            requires_root: true,
        });
    }
    if !manifest.aur_packages.is_empty() {
        options.push(GenericOptionItem {
            id: "install_yay".to_string(),
            title: "2. Install yay AUR Helper (if missing)".to_string(),
            description: "Clones and builds yay-bin from AUR if yay is not found on the system.".to_string(),
            detail: "git clone https://aur.archlinux.org/yay-bin.git /tmp/yay-bin && makepkg -si --noconfirm".to_string(),
            selected: true,
            requires_root: false,
        });
        options.push(GenericOptionItem {
            id: "aur_packages".to_string(),
            title: "Install AUR Packages via yay".to_string(),
            description: "Installs the AUR dependencies declared by the selected source branch."
                .to_string(),
            detail: format!("yay -S --needed {aur_summary}"),
            selected: true,
            requires_root: false,
        });
    }
    options.extend([
        GenericOptionItem {
            id: "build_wtype".to_string(),
            title: "Build optional Wayland input helper (if missing)".to_string(),
            description: "Builds the source-defined input helper with meson and ninja when it is not installed.".to_string(),
            detail: "git clone https://github.com/atx/wtype.git /tmp/wtype && meson setup build && ninja -C build".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "kernel_permissions".to_string(),
            title: "Configure hardware and input permissions".to_string(),
            description: "Loads the optional hardware module, configures CPU permissions, and updates input-group access.".to_string(),
            detail: "Configures /etc/modules-load.d/i2c.conf, /etc/tmpfiles.d/babydra-perf.conf, and runs sudo usermod -aG input $USER.".to_string(),
            selected: true,
            requires_root: true,
        },
    ]);
    options
}

pub fn initial_varlib_options() -> Vec<GenericOptionItem> {
    vec![
        GenericOptionItem {
            id: "stage_binaries".to_string(),
            title: "1. Stage All Built Binaries to /var/lib/babydra/bin/".to_string(),
            description: "Copies the selected compiled executables into /var/lib/babydra/bin/ for shared access.".to_string(),
            detail: "Central system binary storage accessible by all user sessions, display manager, and background daemons.".to_string(),
            selected: true,
            requires_root: true,
        },
        GenericOptionItem {
            id: "set_varlib_permissions".to_string(),
            title: "2. Configure /var/lib/babydra Permissions (chmod 777)".to_string(),
            description: "Sets access permissions so services running under a separate system account can use staged resources.".to_string(),
            detail: "Keeps the staging directory accessible to the sessions that use installed binaries.".to_string(),
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
            description: "Stops active processes whose binaries are about to be replaced.".to_string(),
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
            detail: "Runs update-desktop-database and applies MIME associations declared by source desktop files.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "dotfiles_gtk_terminal".to_string(),
            title: "Sync application and desktop configuration".to_string(),
            description: "Copies configuration directories found under configs/ to their user configuration locations.".to_string(),
            detail: "The source tree determines which application configuration is deployed.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "themes_icons_cursors".to_string(),
            title: "5. Extract & Install Themes, Icons & Cursors".to_string(),
            description: "Copies source theme directories and extracts theme archives into the user data directory.".to_string(),
            detail: "Archives under configs/themes/ are unpacked into ~/.local/share/icons/.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "gsettings_fontcache".to_string(),
            title: "6. Apply GNOME GSettings & Rebuild Font Cache".to_string(),
            description: "Applies GSettings declared by the source branch and rebuilds the font cache.".to_string(),
            detail: "Reads schema/key pairs from [gsettings] in workspace.toml.".to_string(),
            selected: true,
            requires_root: false,
        },
        GenericOptionItem {
            id: "restart_services".to_string(),
            title: "Reload compositor and activate source-defined services".to_string(),
            description: "Reloads the compositor and enables user services discovered in the source tree.".to_string(),
            detail: "Service names and executable names are read from source files.".to_string(),
            selected: true,
            requires_root: false,
        },
    ]
}

pub fn initial_display_manager_options() -> Vec<GenericOptionItem> {
    vec![
        GenericOptionItem {
            id: "greetd_config".to_string(),
            title: "Configure the greetd login session".to_string(),
            description: "Copies the source-provided greetd configuration or generates one from a system-scoped binary.".to_string(),
            detail: "The generated command uses cage and the system-scoped binary declared by the source branch.".to_string(),
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
