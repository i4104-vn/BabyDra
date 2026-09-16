use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::manifest::{load_install_manifest, BinaryManifestItem};
use crate::models::{BinaryItem, BinaryLocation};
use crate::system::get_user_local_bin;

/// Resolves the default destination from a general-purpose convention. A
/// source branch may override it with `[package.metadata.babydra]` in its
/// Cargo manifest; no binary name needs to be added to this installer.
pub fn default_loc(name: &str) -> BinaryLocation {
    if name.ends_with("-greeter") {
        BinaryLocation::SystemBin
    } else {
        BinaryLocation::UserLocalBin
    }
}

/// Resolves the install target path of a binary.
pub fn binary_target_path(name: &str, loc: &BinaryLocation) -> PathBuf {
    match loc {
        BinaryLocation::UserLocalBin => get_user_local_bin().join(name),
        BinaryLocation::SystemBin => PathBuf::from("/usr/bin").join(name),
    }
}

#[derive(Debug, Clone)]
struct BinarySpec {
    name: String,
    source_name: String,
    description: String,
    crate_path: String,
    default_dest: BinaryLocation,
}

/// Discovers every binary target declared by the source workspace.
///
/// The old implementation maintained a list of known application names and
/// guessed crate directories. This implementation reads Cargo manifests and
/// also accepts executable files from a supplied release directory. Adding a
/// new crate, renaming one, or adding a `src/bin/*` target therefore requires
/// no installer change.
pub fn initial_binaries_list(workspace_root: &Path, source_dir: &Path) -> Vec<BinaryItem> {
    let repository_root = repository_root_for(workspace_root);
    let manifest = load_install_manifest(workspace_root, &repository_root);
    let mut discovered = manifest
        .binaries
        .iter()
        .map(manifest_binary_spec)
        .collect::<Vec<_>>();

    // The manifest controls policy (scope/source name), while Cargo remains a
    // safety net for a newly added binary that has not been described yet.
    let cargo_specs = discover_local_workspace(workspace_root);
    let known_names: HashSet<String> = discovered.iter().map(|item| item.name.clone()).collect();
    discovered.extend(
        cargo_specs
            .into_iter()
            .filter(|item| !known_names.contains(&item.name)),
    );

    // On the distribution branch the source workspace is not checked out yet.
    // Inspect all available git refs so the UI can still show the branch's
    // components before the user confirms the worktree checkout.
    if discovered.is_empty() {
        discovered = discover_git_workspaces(&repository_root);
    }

    let mut seen_names: HashSet<String> = discovered.iter().map(|item| item.name.clone()).collect();
    for path in executable_files(source_dir) {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if seen_names.insert(name.to_owned()) {
            discovered.push(BinarySpec {
                name: name.to_owned(),
                source_name: name.to_owned(),
                description: default_crate_description(name),
                crate_path: path
                    .strip_prefix(workspace_root)
                    .unwrap_or(&path)
                    .display()
                    .to_string(),
                default_dest: default_loc(name),
            });
        }
    }

    discovered.sort_by(|a, b| {
        a.default_dest
            .cmp(&b.default_dest)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            .then_with(|| a.name.cmp(&b.name))
    });

    discovered
        .into_iter()
        .map(|spec| {
            let src_file = source_dir.join(&spec.source_name);
            let exists_in_src = src_file.is_file();
            let size = if exists_in_src {
                fs::metadata(&src_file).map(|metadata| metadata.len()).ok()
            } else {
                None
            };
            let exists_in_target = binary_target_path(&spec.name, &spec.default_dest).exists();

            BinaryItem {
                name: spec.name,
                source_name: spec.source_name,
                description: spec.description,
                crate_path: spec.crate_path,
                default_dest: spec.default_dest,
                selected: true,
                exists_in_source: exists_in_src,
                source_size_bytes: size,
                exists_in_target,
            }
        })
        .collect()
}

fn manifest_binary_spec(item: &BinaryManifestItem) -> BinarySpec {
    BinarySpec {
        name: item.name.clone(),
        source_name: item.source_name.clone(),
        description: if item.description.is_empty() {
            default_crate_description(&item.name)
        } else {
            item.description.clone()
        },
        crate_path: "workspace.toml [[binaries]]".to_owned(),
        default_dest: item.location,
    }
}

fn discover_local_workspace(root: &Path) -> Vec<BinarySpec> {
    let manifests = local_workspace_manifests(root);

    let mut specs = Vec::new();
    for manifest in manifests {
        let Ok(content) = fs::read_to_string(&manifest) else {
            continue;
        };
        let crate_dir = manifest.parent().unwrap_or(root);
        let default_main = crate_dir.join("src/main.rs").is_file();
        let bin_files = executable_source_bins(&crate_dir.join("src/bin"));
        specs.extend(parse_manifest_targets(
            &content,
            &relative_path(root, crate_dir),
            default_main,
            &bin_files,
        ));
    }
    deduplicate_specs(specs)
}

fn local_workspace_manifests(root: &Path) -> Vec<PathBuf> {
    let root_manifest = root.join("Cargo.toml");
    let Ok(content) = fs::read_to_string(&root_manifest) else {
        return Vec::new();
    };
    let Ok(table) = content.parse::<toml::Table>() else {
        return Vec::new();
    };

    let mut manifests = Vec::new();
    if let Some(workspace) = table.get("workspace").and_then(toml::Value::as_table) {
        for member in workspace
            .get("members")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(toml::Value::as_str)
        {
            let member_root = root.join(member.trim_end_matches("/*"));
            if member.contains('*') {
                collect_cargo_manifests(&member_root, &mut manifests);
            } else {
                let manifest = member_root.join("Cargo.toml");
                if manifest.is_file() {
                    manifests.push(manifest);
                }
            }
        }
    }
    if table.get("package").is_some() {
        manifests.push(root_manifest);
    }
    manifests
}

fn collect_cargo_manifests(root: &Path, manifests: &mut Vec<PathBuf>) {
    if !root.is_dir() {
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_cargo_manifests(&path, manifests);
        } else if path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml") {
            manifests.push(path);
        }
    }
}

fn discover_git_workspaces(repo: &Path) -> Vec<BinarySpec> {
    for git_ref in git_source_refs(repo) {
        if let Some(manifest) = git_manifest(repo, &git_ref) {
            let specs = manifest
                .binaries
                .iter()
                .map(manifest_binary_spec)
                .collect::<Vec<_>>();
            if !specs.is_empty() {
                return specs;
            }
        }

        let Some(tree) = git_output(repo, &["ls-tree", "-r", "--name-only", &git_ref]) else {
            continue;
        };
        let tree_paths: HashSet<String> = tree
            .lines()
            .map(str::trim)
            .filter(|path| !path.is_empty())
            .map(str::to_owned)
            .collect();

        let root_manifest = git_output(repo, &["show", &format!("{git_ref}:Cargo.toml")]);
        let manifest_paths = git_workspace_manifest_paths(root_manifest.as_deref(), &tree_paths);
        let mut specs = Vec::new();
        for manifest_path in manifest_paths {
            let Some(content) = git_output(repo, &["show", &format!("{git_ref}:{manifest_path}")])
            else {
                continue;
            };
            let crate_dir = Path::new(&manifest_path).parent().unwrap_or(Path::new("."));
            let crate_dir_string = crate_dir.to_string_lossy().to_string();
            let main_path = crate_dir.join("src/main.rs").to_string_lossy().to_string();
            let default_main = tree_paths.contains(&main_path);
            let bin_prefix = crate_dir.join("src/bin");
            let bin_files = tree_paths
                .iter()
                .filter_map(|path| {
                    let path = Path::new(path);
                    if path.parent() == Some(bin_prefix.as_path()) {
                        path.file_stem()
                            .and_then(|stem| stem.to_str())
                            .map(str::to_owned)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            specs.extend(parse_manifest_targets(
                &content,
                &crate_dir_string,
                default_main,
                &bin_files,
            ));
        }

        let specs = deduplicate_specs(specs);
        if !specs.is_empty() {
            return specs;
        }
    }
    Vec::new()
}

fn git_workspace_manifest_paths(
    root_content: Option<&str>,
    tree_paths: &HashSet<String>,
) -> Vec<String> {
    let Some(root_content) = root_content else {
        return Vec::new();
    };
    let Ok(table) = root_content.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(workspace) = table.get("workspace").and_then(toml::Value::as_table) else {
        return table
            .get("package")
            .is_some_and(|_| tree_paths.contains("Cargo.toml"))
            .then(|| "Cargo.toml".to_owned())
            .into_iter()
            .collect();
    };

    let members = workspace
        .get("members")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str)
        .map(|member| member.trim_end_matches("/*").trim_end_matches('/'))
        .collect::<Vec<_>>();

    tree_paths
        .iter()
        .filter(|path| path.ends_with("Cargo.toml"))
        .filter(|path| {
            members.iter().any(|member| {
                *path == &format!("{member}/Cargo.toml") || path.starts_with(&format!("{member}/"))
            })
        })
        .cloned()
        .collect()
}

fn git_manifest(repo: &Path, git_ref: &str) -> Option<super::manifest::InstallManifest> {
    let content = git_output(repo, &["show", &format!("{git_ref}:workspace.toml")])?;
    Some(super::manifest::parse_manifest(&content))
}

fn parse_manifest_targets(
    content: &str,
    crate_path: &str,
    default_main: bool,
    bin_files: &[String],
) -> Vec<BinarySpec> {
    let Ok(root) = content.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(package) = root.get("package").and_then(toml::Value::as_table) else {
        return Vec::new();
    };
    let Some(package_name) = package.get("name").and_then(toml::Value::as_str) else {
        return Vec::new();
    };
    let description = package
        .get("description")
        .and_then(toml::Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| default_crate_description(package_name));
    let location = manifest_location(package, package_name);

    let mut names = Vec::new();
    if let Some(targets) = root.get("bin").and_then(toml::Value::as_array) {
        for target in targets.iter().filter_map(toml::Value::as_table) {
            if let Some(name) = target.get("name").and_then(toml::Value::as_str) {
                names.push(name.to_owned());
            }
        }
    }
    if names.is_empty() && default_main {
        names.push(package_name.to_owned());
    }
    if names.is_empty() {
        names.extend(bin_files.iter().cloned());
    }

    names
        .into_iter()
        .filter(|name| !name.is_empty())
        .map(|name| BinarySpec {
            default_dest: location.unwrap_or_else(|| default_loc(&name)),
            source_name: name.clone(),
            description: if name == package_name {
                description.clone()
            } else {
                default_crate_description(&name)
            },
            name,
            crate_path: crate_path.to_owned(),
        })
        .collect()
}

fn manifest_location(package: &toml::value::Table, package_name: &str) -> Option<BinaryLocation> {
    let value = package
        .get("metadata")
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("babydra"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("install_location"))
        .and_then(toml::Value::as_str);

    match value {
        Some("system") | Some("system-bin") => Some(BinaryLocation::SystemBin),
        Some("user") | Some("user-bin") => Some(BinaryLocation::UserLocalBin),
        _ if package_name.ends_with("-greeter") => Some(BinaryLocation::SystemBin),
        _ => None,
    }
}

fn deduplicate_specs(specs: Vec<BinarySpec>) -> Vec<BinarySpec> {
    let mut seen = HashSet::new();
    specs
        .into_iter()
        .filter(|spec| seen.insert(spec.name.clone()))
        .collect()
}

fn executable_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !root.is_dir() {
        return files;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && is_executable(&path) && path.extension().is_none() {
            files.push(path);
        }
    }
    files
}

fn executable_source_bins(root: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                    names.push(stem.to_owned());
                }
            }
        }
    }
    names
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(path)
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        path.is_file()
    }
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn repository_root_for(path: &Path) -> PathBuf {
    if path.join(".git").exists() {
        return path.to_path_buf();
    }
    path.parent()
        .and_then(Path::parent)
        .filter(|candidate| candidate.join(".git").exists())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| path.to_path_buf())
}

fn git_source_refs(repo: &Path) -> Vec<String> {
    let mut refs = Vec::new();
    for output in [
        git_output(
            repo,
            &["for-each-ref", "--format=%(refname:short)", "refs/remotes"],
        ),
        git_output(
            repo,
            &["for-each-ref", "--format=%(refname:short)", "refs/heads"],
        ),
    ]
    .into_iter()
    .flatten()
    {
        for reference in output
            .lines()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            if reference.ends_with("/HEAD") || refs.iter().any(|item| item == reference) {
                continue;
            }
            refs.push(reference.to_owned());
        }
    }
    refs
}

fn git_output(repo: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .current_dir(repo)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn parse_crate_cargo_toml(path: &Path, fallback_dir_name: &str) -> (String, String) {
    let Ok(content) = fs::read_to_string(path) else {
        return (
            fallback_dir_name.to_owned(),
            default_crate_description(fallback_dir_name),
        );
    };
    let Ok(root) = content.parse::<toml::Table>() else {
        return (
            fallback_dir_name.to_owned(),
            default_crate_description(fallback_dir_name),
        );
    };
    let package = root.get("package").and_then(toml::Value::as_table);
    let name = package
        .and_then(|table| table.get("name"))
        .and_then(toml::Value::as_str)
        .unwrap_or(fallback_dir_name)
        .to_owned();
    let description = package
        .and_then(|table| table.get("description"))
        .and_then(toml::Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| default_crate_description(&name));
    (name, description)
}

pub fn default_crate_description(name: &str) -> String {
    let clean = name.strip_prefix("babydra-").unwrap_or(name);
    let display = clean
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    if display.is_empty() {
        "Application component".to_owned()
    } else {
        format!("{display} application component")
    }
}

pub fn update_binaries_status(items: &mut [BinaryItem], source_dir: &Path) {
    for item in items.iter_mut() {
        let src_file = source_dir.join(&item.source_name);
        item.exists_in_source = src_file.is_file();
        item.source_size_bytes = if item.exists_in_source {
            fs::metadata(&src_file).map(|metadata| metadata.len()).ok()
        } else {
            None
        };
        item.exists_in_target = binary_target_path(&item.name, &item.default_dest).exists();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_default_and_explicit_binary_targets() {
        let root = std::env::temp_dir().join(format!(
            "babydra_discovery_test_{}_{}",
            std::process::id(),
            "workspace"
        ));
        let crate_dir = root.join("packages/new-shell");
        fs::create_dir_all(crate_dir.join("src/bin")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"packages/new-shell\"]\n",
        )
        .unwrap();
        fs::write(
            crate_dir.join("Cargo.toml"),
            "[package]\nname = \"new-shell\"\ndescription = \"A new shell\"\n\n[[bin]]\nname = \"shell-ui\"\npath = \"src/main.rs\"\n\n[package.metadata.babydra]\ninstall_location = \"system\"\n",
        )
        .unwrap();
        fs::write(crate_dir.join("src/main.rs"), "fn main() {}\n").unwrap();

        let found = discover_local_workspace(&root);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "shell-ui");
        assert_eq!(found[0].default_dest, BinaryLocation::SystemBin);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ignores_nested_cargo_projects_when_root_is_not_a_workspace() {
        let root = std::env::temp_dir().join(format!(
            "babydra_discovery_test_{}_nested",
            std::process::id()
        ));
        let nested = root.join("install");
        fs::create_dir_all(nested.join("src")).unwrap();
        fs::write(
            nested.join("Cargo.toml"),
            "[package]\nname = \"installer\"\nversion = \"1.0.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(nested.join("src/main.rs"), "fn main() {}\n").unwrap();

        assert!(discover_local_workspace(&root).is_empty());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn defaults_are_name_agnostic() {
        assert_eq!(default_loc("my-greeter"), BinaryLocation::SystemBin);
        assert_eq!(default_loc("my-panel"), BinaryLocation::UserLocalBin);
        assert_eq!(
            default_crate_description("babydra-custom"),
            "Custom application component"
        );
    }
}
