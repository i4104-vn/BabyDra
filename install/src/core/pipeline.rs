use super::manifest::InstallManifest;
use crate::models::{BinaryItem, BinaryLocation};

/// A single step in the installation pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub requires_root: bool,
}

/// Builds the ordered list of installation steps directly from the manifest and selected options.
pub fn build_pipeline(
    manifest: &InstallManifest,
    selected_binaries: &[BinaryItem],
    from_branch: bool,
    branch_name: &str,
) -> Vec<TaskStep> {
    let mut steps = Vec::new();

    // Phase 1: Git checkout & workspace build
    if from_branch {
        steps.push(TaskStep {
            id: "git_checkout".to_string(),
            title: format!("Pull latest code for branch '{branch_name}'"),
            description: format!("Syncing branch '{branch_name}' in branches/{branch_name}"),
            requires_root: false,
        });
        steps.push(TaskStep {
            id: "cargo_build".to_string(),
            title: "Build workspace (cargo build --release)".to_string(),
            description: "Builds the workspace executables in release mode".to_string(),
            requires_root: false,
        });
    }

    // Phase 2: Terminate running instances
    steps.push(TaskStep {
        id: "terminate".to_string(),
        title: "Terminate Old Running Instances (killall)".to_string(),
        description: "Stops active processes whose binaries are about to be replaced to avoid ETXTBSY errors.".to_string(),
        requires_root: false,
    });

    // Phase 3: Packages
    if !manifest.pacman_packages.is_empty() {
        steps.push(TaskStep {
            id: "pacman".to_string(),
            title: "Install Arch Linux Pacman Packages".to_string(),
            description: format!("pacman -Syu --needed {}", manifest.pacman_packages.join(" ")),
            requires_root: true,
        });
    }

    if !manifest.aur_packages.is_empty() {
        steps.push(TaskStep {
            id: "aur_helper".to_string(),
            title: "Install yay AUR Helper (if missing)".to_string(),
            description: "Clones and builds yay-bin from AUR if yay is not found.".to_string(),
            requires_root: false,
        });
        steps.push(TaskStep {
            id: "aur_packages".to_string(),
            title: "Install AUR Packages via yay".to_string(),
            description: format!("yay -S --needed {}", manifest.aur_packages.join(" ")),
            requires_root: false,
        });
    }

    // Phase 4: Build dependencies
    for dep in &manifest.build_deps {
        steps.push(TaskStep {
            id: format!("build_dep:{}", dep.name),
            title: format!("Build optional helper '{}' (if missing)", dep.name),
            description: format!("Builds {} from {}", dep.name, dep.git),
            requires_root: false,
        });
    }

    // Phase 5: Hardware & system permissions
    if manifest.permissions.is_some() {
        steps.push(TaskStep {
            id: "permissions".to_string(),
            title: "Configure hardware and input permissions".to_string(),
            description: "Loads kernel modules and configures CPU performance & input group permissions.".to_string(),
            requires_root: true,
        });
    }

    // Phase 6: Prebuilt Binaries
    for bin in selected_binaries {
        let is_system = matches!(bin.default_dest, BinaryLocation::SystemBin);
        steps.push(TaskStep {
            id: format!("binary:{}", bin.name),
            title: format!("Install binary: {}", bin.name),
            description: format!(
                "Copies {} into {}",
                bin.name,
                if is_system { "/usr/bin" } else { "~/.local/bin" }
            ),
            requires_root: is_system,
        });
    }

    // Phase 7: /var/lib Staging
    steps.push(TaskStep {
        id: "staging_binaries".to_string(),
        title: format!("Stage All Built Binaries to {}/bin/", manifest.staging.path),
        description: format!(
            "Copies compiled executables into {}/bin/ for shared access.",
            manifest.staging.path
        ),
        requires_root: true,
    });
    steps.push(TaskStep {
        id: "staging_permissions".to_string(),
        title: format!(
            "Configure {} Permissions (chmod {})",
            manifest.staging.path, manifest.staging.permissions
        ),
        description: format!(
            "Sets access permissions on {} for system services.",
            manifest.staging.path
        ),
        requires_root: true,
    });

    // Phase 8: Theme packages
    steps.push(TaskStep {
        id: "themes_deploy".to_string(),
        title: "Deploy Theme Packages".to_string(),
        description: "Deploys theme packages to user and system theme directories.".to_string(),
        requires_root: true,
    });

    // Phase 9: Configs
    if !manifest.configs.is_empty() {
        for rule in &manifest.configs {
            steps.push(TaskStep {
                id: format!("config:{}", rule.source),
                title: format!("Sync configuration: {}", rule.source),
                description: format!("Syncs {} to {}", rule.source, rule.target),
                requires_root: false,
            });
        }
    } else {
        // Convention fallback for branches that don't declare [[installer.configs]]
        steps.push(TaskStep {
            id: "config:labwc".to_string(),
            title: "Sync Labwc Compositor Configuration".to_string(),
            description: "Syncs autostart, rc.xml, scripts to ~/.config/labwc".to_string(),
            requires_root: false,
        });
        steps.push(TaskStep {
            id: "config:dotfiles".to_string(),
            title: "Sync application and desktop configuration".to_string(),
            description: "Copies configuration directories found under configs/ to ~/.config".to_string(),
            requires_root: false,
        });
    }

    // Phase 10: Themes, Icons & Cursors
    steps.push(TaskStep {
        id: "themes_icons".to_string(),
        title: "Extract & Install Themes, Icons & Cursors".to_string(),
        description: "Unpacks archives and copies icons/themes into user data directories.".to_string(),
        requires_root: false,
    });

    // Phase 11: Desktop entries & MIME associations
    steps.push(TaskStep {
        id: "desktop".to_string(),
        title: "Register .desktop Entries & MIME Associations".to_string(),
        description: "Installs .desktop entries, custom MIME packages, and MIME type associations.".to_string(),
        requires_root: false,
    });

    // Phase 12: GSettings & Font cache
    if !manifest.gsettings.is_empty() {
        steps.push(TaskStep {
            id: "gsettings".to_string(),
            title: "Apply GNOME GSettings & Rebuild Font Cache".to_string(),
            description: "Applies declared GSettings and rebuilds font cache.".to_string(),
            requires_root: false,
        });
    }

    // Phase 13: Services & compositor reload
    steps.push(TaskStep {
        id: "services".to_string(),
        title: "Reload compositor and activate source-defined services".to_string(),
        description: "Reloads compositor configuration and activates user services.".to_string(),
        requires_root: false,
    });

    // Phase 14: Display manager
    if let Some(greetd) = &manifest.greetd {
        steps.push(TaskStep {
            id: "greetd_config".to_string(),
            title: "Configure the greetd login session".to_string(),
            description: "Configures /etc/greetd/config.toml from source metadata.".to_string(),
            requires_root: true,
        });
        if !greetd.mask_gettys.is_empty() {
            steps.push(TaskStep {
                id: "mask_gettys".to_string(),
                title: "Mask Secondary VTs (tty2-6 gettys) to Eliminate Screen Flash".to_string(),
                description: "Stops and masks getty services on secondary virtual terminals.".to_string(),
                requires_root: true,
            });
        }
        steps.push(TaskStep {
            id: "enable_greetd".to_string(),
            title: format!("Enable {} on Boot", greetd.enable_service),
            description: format!("Runs systemctl enable {}", greetd.enable_service),
            requires_root: true,
        });
    }

    steps
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn pipeline_includes_git_when_from_branch() {
        let manifest = InstallManifest::default();
        let steps = build_pipeline(&manifest, &[], true, "develop");
        assert_eq!(steps[0].id, "git_checkout");
        assert_eq!(steps[1].id, "cargo_build");
    }

    #[test]
    fn pipeline_omits_git_when_not_from_branch() {
        let manifest = InstallManifest::default();
        let steps = build_pipeline(&manifest, &[], false, "");
        assert_eq!(steps[0].id, "terminate");
    }
}
