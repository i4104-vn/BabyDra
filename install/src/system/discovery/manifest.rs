use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::models::BinaryLocation;

/// Data that belongs to a source branch rather than to installer logic.
///
/// A source branch owns this data. The installer only reads
/// `workspace.toml`; it does not contain a project-specific fallback.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstallManifest {
    pub binaries: Vec<BinaryManifestItem>,
    pub pacman_packages: Vec<String>,
    pub aur_packages: Vec<String>,
    pub gsettings: BTreeMap<String, String>,
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

pub(crate) fn parse_manifest(content: &str) -> InstallManifest {
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
        packages
            .and_then(|table| table.get(key))
            .and_then(toml::Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .map(str::to_owned)
                    .filter(|value| !value.is_empty())
                    .collect()
            })
            .unwrap_or_default()
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

    InstallManifest {
        binaries,
        pacman_packages: strings("pacman"),
        aur_packages: strings("aur"),
        gsettings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_branch_owned_dependencies_and_scopes() {
        let manifest = parse_manifest(
            "[[binaries]]\nname = \"new-greeter\"\nscope = \"system\"\n\n[packages]\npacman = [\"gtk4\"]\naur = [\"kitty\"]\n",
        );
        assert_eq!(manifest.binaries[0].location, BinaryLocation::SystemBin);
        assert!(manifest.pacman_packages.contains(&"gtk4".to_string()));
        assert!(manifest.aur_packages.contains(&"kitty".to_string()));
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
}
