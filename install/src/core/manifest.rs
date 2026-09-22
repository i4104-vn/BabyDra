use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use crate::models::BinaryLocation;

/// Declarative configuration synchronization rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigRule {
    /// Relative source path in the source branch (file or directory).
    pub source: String,
    /// Destination path (supports variables: `$HOME`, `$CONFIG`, `$LOCAL_BIN`, `$DATA`).
    pub target: String,
    /// List of globs/patterns within `target` to make executable (e.g. `["autostart", "scripts/*"]`).
    pub executable: Vec<String>,
}

/// Declarative desktop integration options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopConfig {
    /// Directory containing .desktop entry files (default: `"desktops"`).
    pub entries_dir: Option<String>,
    /// Directories to search for custom MIME XML definitions.
    pub mime_packages: Vec<String>,
    /// Whether to auto-discover and install D-Bus service unit files.
    pub dbus_services: bool,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            entries_dir: Some("desktops".to_string()),
            mime_packages: vec![
                "desktops/mime".to_string(),
                "mime/packages".to_string(),
                "mime".to_string(),
            ],
            dbus_services: true,
        }
    }
}

/// Declarative theme deployment configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeConfig {
    /// Source directory for theme packages (default: `"themes"`).
    pub packages: Option<String>,
    /// Source directory containing theme archives (.tar) (default: `"configs/themes"`).
    pub archives: Option<String>,
    /// Target path for storing selected theme configuration (default: `"$HOME/.babydra/babydra.conf"`).
    pub conf_path: Option<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            packages: Some("themes".to_string()),
            archives: Some("configs/themes".to_string()),
            conf_path: Some("$HOME/.babydra/babydra.conf".to_string()),
        }
    }
}

/// Declarative binary staging options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagingConfig {
    /// System directory to stage binaries into (default: `"/var/lib/babydra"`).
    pub path: String,
    /// File mode/permissions for staging directory (default: `"777"`).
    pub permissions: String,
}

impl Default for StagingConfig {
    fn default() -> Self {
        Self {
            path: "/var/lib/babydra".to_string(),
            permissions: "777".to_string(),
        }
    }
}

/// Declarative kernel permissions and system group setup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionsConfig {
    /// Kernel modules to load via modprobe & /etc/modules-load.d/ (e.g. `["i2c-dev"]`).
    pub modules: Vec<String>,
    /// User groups to add current user to via `usermod -aG` (e.g. `["input"]`).
    pub groups: Vec<String>,
    /// Key-value pairs of tmpfiles.d filenames and their file contents.
    pub tmpfiles: BTreeMap<String, String>,
}

impl Default for PermissionsConfig {
    fn default() -> Self {
        let mut tmpfiles = BTreeMap::new();
        tmpfiles.insert(
            "babydra-perf.conf".to_string(),
            "z /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 0666 root root -\nz /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 0666 root root -\n".to_string(),
        );
        Self {
            modules: vec!["i2c-dev".to_string()],
            groups: vec!["input".to_string()],
            tmpfiles,
        }
    }
}

/// Declarative display manager (greetd) options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreetdConfig {
    /// Explicit path to greetd configuration file (or auto-generated if None).
    pub config: Option<String>,
    /// Virtual terminals to mask getty for to prevent screen flash (e.g. `[2, 3, 4, 5, 6]`).
    pub mask_gettys: Vec<u32>,
    /// Systemd service name to enable on boot (default: `"greetd.service"`).
    pub enable_service: String,
}

impl Default for GreetdConfig {
    fn default() -> Self {
        Self {
            config: None,
            mask_gettys: vec![2, 3, 4, 5, 6],
            enable_service: "greetd.service".to_string(),
        }
    }
}

/// Generic build-from-source dependency rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildDepConfig {
    pub name: String,
    /// Executable name to check with `which <check>`. If found, build is skipped.
    pub check: String,
    pub git: String,
    pub build: Vec<String>,
    pub artifact: String,
    /// Target path (supports variables like `$LOCAL_BIN`).
    pub install_to: String,
}

/// Complete install manifest owned by a source branch.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InstallManifest {
    pub binaries: Vec<BinaryManifestItem>,
    pub pacman_packages: Vec<String>,
    pub aur_packages: Vec<String>,
    pub gsettings: BTreeMap<String, String>,
    pub features: HashSet<String>,
    pub mime: BTreeMap<String, String>,
    pub mime_associations: BTreeMap<String, Vec<String>>,

    // Declarative sections
    pub configs: Vec<ConfigRule>,
    pub desktop: DesktopConfig,
    pub themes: ThemeConfig,
    pub staging: StagingConfig,
    pub permissions: Option<PermissionsConfig>,
    pub greetd: Option<GreetdConfig>,
    pub build_deps: Vec<BuildDepConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryManifestItem {
    pub name: String,
    pub source_name: String,
    pub description: String,
    pub location: BinaryLocation,
}

pub fn load_install_manifest(source_root: &Path, fallback_root: &Path) -> InstallManifest {
    for path in candidate_paths(source_root, fallback_root) {
        if let Ok(content) = std::fs::read_to_string(path) {
            return parse_manifest(&content);
        }
    }
    InstallManifest::default()
}

fn candidate_paths(source_root: &Path, fallback_root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in [source_root, fallback_root] {
        paths.push(root.join("workspace.toml"));
    }
    paths
}

pub fn parse_manifest(content: &str) -> InstallManifest {
    let Ok(root) = content.parse::<toml::Table>() else {
        return InstallManifest::default();
    };

    let packages = root.get("packages").and_then(toml::Value::as_table);
    let binaries = root
        .get("binaries")
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let table = entry.as_table()?;
                    let name = table.get("name")?.as_str()?.trim();
                    if name.is_empty() {
                        return None;
                    }
                    let source_name = table
                        .get("source")
                        .and_then(toml::Value::as_str)
                        .filter(|value| !value.is_empty())
                        .unwrap_or(name);
                    let location = match table
                        .get("scope")
                        .and_then(toml::Value::as_str)
                        .unwrap_or("user")
                    {
                        "system" => BinaryLocation::SystemBin,
                        _ => BinaryLocation::UserLocalBin,
                    };
                    Some(BinaryManifestItem {
                        name: name.to_owned(),
                        source_name: source_name.to_owned(),
                        description: table
                            .get("description")
                            .and_then(toml::Value::as_str)
                            .unwrap_or_default()
                            .to_owned(),
                        location,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let strings = |key: &str| {
        let mut values: Vec<String> = packages
            .and_then(|table| table.get(key))
            .and_then(toml::Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str().map(str::trim))
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();
        let mut seen = HashSet::new();
        values.retain(|value| seen.insert(value.clone()));
        values
    };

    let gsettings = root
        .get("gsettings")
        .and_then(toml::Value::as_table)
        .map(|table| {
            table
                .iter()
                .filter_map(|(key, value)| value.as_str().map(|value| (key.clone(), value.into())))
                .collect()
        })
        .unwrap_or_default();

    let installer_table = root.get("installer").and_then(toml::Value::as_table);

    let features: HashSet<String> = installer_table
        .and_then(|table| table.get("features"))
        .and_then(toml::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(str::trim))
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let mut mime = BTreeMap::new();
    let mut mime_associations: BTreeMap<String, Vec<String>> = BTreeMap::new();

    if let Some(mime_table) = root.get("mime").and_then(toml::Value::as_table) {
        for (key, value) in mime_table {
            if let Some(s) = value.as_str() {
                if key.ends_with(".desktop") {
                    mime_associations
                        .entry(key.clone())
                        .or_default()
                        .push(s.to_string());
                } else {
                    mime.insert(key.clone(), s.to_string());
                }
            } else if let Some(arr) = value.as_array() {
                let list: Vec<String> = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(str::trim))
                    .filter(|v| !v.is_empty())
                    .map(str::to_owned)
                    .collect();
                if !list.is_empty() {
                    mime_associations.insert(key.clone(), list);
                }
            } else if let Some(sub_table) = value.as_table() {
                for (sub_k, sub_v) in sub_table {
                    if let Some(s) = sub_v.as_str() {
                        mime.insert(sub_k.clone(), s.to_string());
                    } else if let Some(arr) = sub_v.as_array() {
                        let list: Vec<String> = arr
                            .iter()
                            .filter_map(|v| v.as_str().map(str::trim))
                            .filter(|v| !v.is_empty())
                            .map(str::to_owned)
                            .collect();
                        if !list.is_empty() {
                            mime_associations.insert(sub_k.clone(), list);
                        }
                    }
                }
            }
        }
    }

    // ─── Declarative Section Parsing ────────────────────────

    // 1. [[installer.configs]]
    let mut configs = Vec::new();
    if let Some(configs_arr) = installer_table
        .and_then(|t| t.get("configs"))
        .and_then(toml::Value::as_array)
    {
        for entry in configs_arr {
            if let Some(t) = entry.as_table() {
                let source = t.get("source").and_then(toml::Value::as_str).unwrap_or("").trim();
                let target = t.get("target").and_then(toml::Value::as_str).unwrap_or("").trim();
                if !source.is_empty() && !target.is_empty() {
                    let executable = t
                        .get("executable")
                        .and_then(toml::Value::as_array)
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(str::to_string))
                                .collect()
                        })
                        .unwrap_or_default();
                    configs.push(ConfigRule {
                        source: source.to_string(),
                        target: target.to_string(),
                        executable,
                    });
                }
            }
        }
    }

    // 2. [installer.desktop]
    let desktop = if let Some(t) = installer_table
        .and_then(|t| t.get("desktop"))
        .and_then(toml::Value::as_table)
    {
        DesktopConfig {
            entries_dir: t.get("entries_dir").and_then(toml::Value::as_str).map(str::to_string),
            mime_packages: t
                .get("mime_packages")
                .and_then(toml::Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_else(|| DesktopConfig::default().mime_packages),
            dbus_services: t
                .get("dbus_services")
                .and_then(toml::Value::as_bool)
                .unwrap_or(true),
        }
    } else {
        DesktopConfig::default()
    };

    // 3. [installer.themes]
    let themes = if let Some(t) = installer_table
        .and_then(|t| t.get("themes"))
        .and_then(toml::Value::as_table)
    {
        ThemeConfig {
            packages: t.get("packages").and_then(toml::Value::as_str).map(str::to_string),
            archives: t.get("archives").and_then(toml::Value::as_str).map(str::to_string),
            conf_path: t.get("conf_path").and_then(toml::Value::as_str).map(str::to_string),
        }
    } else {
        ThemeConfig::default()
    };

    // 4. [installer.staging]
    let staging = if let Some(t) = installer_table
        .and_then(|t| t.get("staging"))
        .and_then(toml::Value::as_table)
    {
        StagingConfig {
            path: t
                .get("path")
                .and_then(toml::Value::as_str)
                .unwrap_or("/var/lib/babydra")
                .to_string(),
            permissions: t
                .get("permissions")
                .and_then(toml::Value::as_str)
                .unwrap_or("777")
                .to_string(),
        }
    } else {
        StagingConfig::default()
    };

    // 5. [installer.permissions]
    let permissions = if let Some(t) = installer_table
        .and_then(|t| t.get("permissions"))
        .and_then(toml::Value::as_table)
    {
        let modules = t
            .get("modules")
            .and_then(toml::Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let groups = t
            .get("groups")
            .and_then(toml::Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let tmpfiles = t
            .get("tmpfiles")
            .and_then(toml::Value::as_table)
            .map(|tbl| {
                tbl.iter()
                    .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        Some(PermissionsConfig {
            modules,
            groups,
            tmpfiles,
        })
    } else if features.contains("kernel_permissions") {
        Some(PermissionsConfig::default())
    } else {
        None
    };

    // 6. [installer.greetd]
    let greetd = if let Some(t) = installer_table
        .and_then(|t| t.get("greetd"))
        .and_then(toml::Value::as_table)
    {
        let config = t.get("config").and_then(toml::Value::as_str).map(str::to_string);
        let mask_gettys = t
            .get("mask_gettys")
            .and_then(toml::Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_integer().map(|n| n as u32))
                    .collect()
            })
            .unwrap_or_else(|| vec![2, 3, 4, 5, 6]);
        let enable_service = t
            .get("enable_service")
            .and_then(toml::Value::as_str)
            .unwrap_or("greetd.service")
            .to_string();
        Some(GreetdConfig {
            config,
            mask_gettys,
            enable_service,
        })
    } else if features.contains("greetd") {
        Some(GreetdConfig::default())
    } else {
        None
    };

    // 7. [[installer.build_deps]]
    let mut build_deps = Vec::new();
    if let Some(deps_arr) = installer_table
        .and_then(|t| t.get("build_deps"))
        .and_then(toml::Value::as_array)
    {
        for entry in deps_arr {
            if let Some(t) = entry.as_table() {
                let name = t.get("name").and_then(toml::Value::as_str).unwrap_or("");
                let check = t.get("check").and_then(toml::Value::as_str).unwrap_or(name);
                let git = t.get("git").and_then(toml::Value::as_str).unwrap_or("");
                let build: Vec<String> = t
                    .get("build")
                    .and_then(toml::Value::as_array)
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                let artifact = t.get("artifact").and_then(toml::Value::as_str).unwrap_or("");
                let install_to = t
                    .get("install_to")
                    .and_then(toml::Value::as_str)
                    .unwrap_or("$LOCAL_BIN");

                if !name.is_empty() && !git.is_empty() {
                    build_deps.push(BuildDepConfig {
                        name: name.to_string(),
                        check: check.to_string(),
                        git: git.to_string(),
                        build,
                        artifact: artifact.to_string(),
                        install_to: install_to.to_string(),
                    });
                }
            }
        }
    } else if features.contains("wtype") {
        build_deps.push(BuildDepConfig {
            name: "wtype".to_string(),
            check: "wtype".to_string(),
            git: "https://github.com/atx/wtype.git".to_string(),
            build: vec!["meson setup build".to_string(), "ninja -C build".to_string()],
            artifact: "build/wtype".to_string(),
            install_to: "$LOCAL_BIN/wtype".to_string(),
        });
    }

    InstallManifest {
        binaries,
        pacman_packages: strings("pacman"),
        aur_packages: strings("aur"),
        gsettings,
        features,
        mime,
        mime_associations,
        configs,
        desktop,
        themes,
        staging,
        permissions,
        greetd,
        build_deps,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parses_branch_owned_dependencies_and_scopes() {
        let manifest = parse_manifest(
            "[[binaries]]\nname = \"new-greeter\"\nscope = \"system\"\n\n[packages]\npacman = [\"gtk4\"]\naur = [\"kitty\"]\n",
        );
        assert_eq!(manifest.binaries[0].location, BinaryLocation::SystemBin);
        assert_eq!(manifest.pacman_packages, vec!["gtk4"]);
        assert!(manifest.aur_packages.contains(&"kitty".to_string()));
    }

    #[test]
    fn parses_mime_types_and_associations() {
        let manifest = parse_manifest(
            "[mime]\n\"inode/directory\" = \"babydra-explore.desktop\"\n\"babydra-notepad.desktop\" = [\"text/plain\", \"text/markdown\"]\n",
        );
        assert_eq!(
            manifest.mime.get("inode/directory").map(String::as_str),
            Some("babydra-explore.desktop")
        );
        assert_eq!(
            manifest.mime_associations.get("babydra-notepad.desktop"),
            Some(&vec!["text/plain".to_string(), "text/markdown".to_string()])
        );
    }

    #[test]
    fn normalizes_package_names_before_install() {
        let manifest = parse_manifest(
            "[packages]\npacman = [\" gtk4 \", \"gtk4\", \"\"]\naur = [\"kitty\", \" kitty \"]\n",
        );
        assert_eq!(manifest.pacman_packages, vec!["gtk4"]);
        assert_eq!(manifest.aur_packages, vec!["kitty"]);
    }

    #[test]
    fn source_workspace_manifest_is_preferred() {
        let source = std::env::temp_dir().join(format!(
            "babydra_manifest_test_{}_{}",
            std::process::id(),
            "source"
        ));
        let fallback = source.join("fallback");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(
            source.join("workspace.toml"),
            "[packages]\npacman = [\"custom-pkg\"]\n",
        )
        .unwrap();

        let manifest = load_install_manifest(&source, &fallback);
        assert_eq!(manifest.pacman_packages, vec!["custom-pkg"]);
        assert!(manifest.aur_packages.is_empty());

        let _ = std::fs::remove_dir_all(source);
    }

    #[test]
    fn parses_declarative_sections_and_fallback_compatibility() {
        let toml_data = r#"
[installer]
features = ["wtype", "kernel_permissions", "greetd"]

[[installer.configs]]
source = "configs/custom"
target = "$CONFIG/custom"
executable = ["run.sh"]

[installer.staging]
path = "/opt/babydra"
permissions = "755"
"#;
        let manifest = parse_manifest(toml_data);
        assert_eq!(manifest.configs.len(), 1);
        assert_eq!(manifest.configs[0].source, "configs/custom");
        assert_eq!(manifest.configs[0].target, "$CONFIG/custom");
        assert_eq!(manifest.configs[0].executable, vec!["run.sh"]);
        assert_eq!(manifest.staging.path, "/opt/babydra");
        assert_eq!(manifest.staging.permissions, "755");

        // Feature fallback checks
        assert!(manifest.permissions.is_some());
        assert!(manifest.greetd.is_some());
        assert_eq!(manifest.build_deps.len(), 1);
        assert_eq!(manifest.build_deps[0].name, "wtype");
    }
}
