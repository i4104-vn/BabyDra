use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::core::manifest::{load_install_manifest, BinaryManifestItem};
use crate::models::{BinaryItem, BinaryLocation};
use crate::runtime::get_user_local_bin;

/// Resolves the default destination from a general-purpose convention.
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
pub fn initial_binaries_list(workspace_root: &Path, source_dir: &Path) -> Vec<BinaryItem> {
    let repository_root = repository_root_for(workspace_root);
    let manifest = load_install_manifest(workspace_root, &repository_root);
    let mut discovered = manifest
        .binaries
        .iter()
        .map(manifest_binary_spec)
        .collect::<Vec<_>>();

    let cargo_specs = discover_local_workspace(workspace_root);
    let known_names: HashSet<String> = discovered.iter().map(|item| item.name.clone()).collect();
    discovered.extend(
        cargo_specs
            .into_iter()
            .filter(|item| !known_names.contains(&item.name)),
    );

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
            let exists_in_source = src_file.is_file();
            let source_size_bytes = fs::metadata(&src_file).ok().map(|m| m.len());
            let exists_in_target = binary_target_path(&spec.name, &spec.default_dest).exists();
            BinaryItem {
                name: spec.name,
                source_name: spec.source_name,
                description: spec.description,
                crate_path: spec.crate_path,
                default_dest: spec.default_dest,
                selected: true,
                exists_in_source,
                source_size_bytes,
                exists_in_target,
            }
        })
        .collect()
}

pub fn update_binaries_status(items: &mut [BinaryItem], source_dir: &Path) {
    for item in items {
        let src_file = source_dir.join(&item.source_name);
        item.exists_in_source = src_file.is_file();
        item.source_size_bytes = fs::metadata(&src_file).ok().map(|m| m.len());
        item.exists_in_target = binary_target_path(&item.name, &item.default_dest).exists();
    }
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
        crate_path: format!("workspace.toml:{}", item.name),
        default_dest: item.location,
    }
}

fn discover_local_workspace(root: &Path) -> Vec<BinarySpec> {
    let manifest_path = root.join("Cargo.toml");
    if !manifest_path.is_file() {
        return Vec::new();
    }

    let Ok(content) = fs::read_to_string(&manifest_path) else {
        return Vec::new();
    };
    let Ok(table) = content.parse::<toml::Table>() else {
        return Vec::new();
    };

    let member_globs = workspace_member_globs(&table);
    if member_globs.is_empty() {
        return parse_cargo_manifest(&content, manifest_path.parent().unwrap_or(root), root);
    }

    let mut specs = Vec::new();
    let mut visited_paths = HashSet::new();

    for member_dir in resolve_member_paths(root, &member_globs) {
        let manifest = member_dir.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }
        let Ok(member_content) = fs::read_to_string(&manifest) else {
            continue;
        };
        for spec in parse_cargo_manifest(&member_content, &member_dir, root) {
            if visited_paths.insert(spec.crate_path.clone()) {
                specs.push(spec);
            }
        }
    }

    specs
}

fn discover_git_workspaces(repo_root: &Path) -> Vec<BinarySpec> {
    let mut specs = Vec::new();
    let mut visited = HashSet::new();

    for branch in available_branches(repo_root) {
        for spec in discover_git_workspace_ref(repo_root, &branch) {
            if visited.insert(spec.name.clone()) {
                specs.push(spec);
            }
        }
        if !specs.is_empty() {
            break;
        }
    }

    specs
}

fn discover_git_workspace_ref(repo_root: &Path, git_ref: &str) -> Vec<BinarySpec> {
    let Ok(root_cargo) = git_show(repo_root, git_ref, "Cargo.toml") else {
        return Vec::new();
    };
    let Ok(table) = root_cargo.parse::<toml::Table>() else {
        return Vec::new();
    };

    let member_globs = workspace_member_globs(&table);
    if member_globs.is_empty() {
        return parse_cargo_manifest(&root_cargo, repo_root, repo_root);
    }

    let mut specs = Vec::new();
    let mut visited = HashSet::new();

    for file_path in git_manifest_paths(repo_root, git_ref) {
        let member_dir = Path::new(&file_path)
            .parent()
            .unwrap_or_else(|| Path::new(""));
        let member_dir_str = member_dir.to_string_lossy();
        if !member_globs
            .iter()
            .any(|glob| member_matches_glob(&member_dir_str, glob))
        {
            continue;
        }

        let Ok(content) = git_show(repo_root, git_ref, &file_path) else {
            continue;
        };
        for spec in parse_cargo_manifest(&content, member_dir, Path::new("")) {
            if visited.insert(spec.name.clone()) {
                specs.push(spec);
            }
        }
    }

    specs
}

fn workspace_member_globs(table: &toml::Table) -> Vec<String> {
    table
        .get("workspace")
        .and_then(toml::Value::as_table)
        .and_then(|ws| ws.get("members"))
        .and_then(toml::Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn resolve_member_paths(root: &Path, member_globs: &[String]) -> Vec<PathBuf> {
    let mut member_paths = Vec::new();
    let mut seen = HashSet::new();

    for glob in member_globs {
        if glob.contains('*') {
            let prefix = glob.trim_end_matches("/*").trim_end_matches('*');
            let base_dir = if prefix.is_empty() {
                root.to_path_buf()
            } else {
                root.join(prefix)
            };
            if let Ok(entries) = fs::read_dir(base_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() && path.join("Cargo.toml").is_file() && seen.insert(path.clone()) {
                        member_paths.push(path);
                    }
                }
            }
        } else {
            let path = root.join(glob);
            if path.is_dir() && path.join("Cargo.toml").is_file() && seen.insert(path.clone()) {
                member_paths.push(path);
            }
        }
    }

    member_paths
}

fn member_matches_glob(member_dir: &str, glob: &str) -> bool {
    if glob.ends_with("/*") {
        let prefix = glob.trim_end_matches("/*");
        member_dir
            .strip_prefix(prefix)
            .map(|rest| rest.trim_start_matches('/').chars().all(|c| c != '/'))
            .unwrap_or(false)
    } else {
        member_dir == glob
    }
}

fn parse_cargo_manifest(content: &str, member_dir: &Path, workspace_root: &Path) -> Vec<BinarySpec> {
    let Ok(table) = content.parse::<toml::Table>() else {
        return Vec::new();
    };

    let crate_path = member_dir
        .strip_prefix(workspace_root)
        .unwrap_or(member_dir)
        .display()
        .to_string();

    let mut specs = Vec::new();
    let package_desc = table
        .get("package")
        .and_then(toml::Value::as_table)
        .and_then(|pkg| pkg.get("description"))
        .and_then(toml::Value::as_str)
        .unwrap_or("");

    let metadata_dest = table
        .get("package")
        .and_then(toml::Value::as_table)
        .and_then(|pkg| pkg.get("metadata"))
        .and_then(toml::Value::as_table)
        .and_then(|meta| meta.get("babydra"))
        .and_then(toml::Value::as_table)
        .and_then(|babydra| babydra.get("dest"))
        .and_then(toml::Value::as_str)
        .and_then(parse_destination_override);

    if let Some(bins) = table.get("bin").and_then(toml::Value::as_array) {
        for bin in bins.iter().filter_map(toml::Value::as_table) {
            let Some(name) = bin.get("name").and_then(toml::Value::as_str) else {
                continue;
            };
            let desc = bin
                .get("description")
                .and_then(toml::Value::as_str)
                .unwrap_or(package_desc);
            let dest = bin
                .get("metadata")
                .and_then(toml::Value::as_table)
                .and_then(|meta| meta.get("babydra"))
                .and_then(toml::Value::as_table)
                .and_then(|babydra| babydra.get("dest"))
                .and_then(toml::Value::as_str)
                .and_then(parse_destination_override)
                .or(metadata_dest)
                .unwrap_or_else(|| default_loc(name));

            specs.push(BinarySpec {
                name: name.to_owned(),
                source_name: name.to_owned(),
                description: if desc.is_empty() {
                    default_crate_description(name)
                } else {
                    desc.to_owned()
                },
                crate_path: crate_path.clone(),
                default_dest: dest,
            });
        }
    }

    if specs.is_empty() {
        let has_bin_source = member_dir.join("src/main.rs").is_file()
            || member_dir.join("src/bin").is_dir()
            || table
                .get("package")
                .and_then(toml::Value::as_table)
                .and_then(|pkg| pkg.get("default-run"))
                .is_some();

        if has_bin_source {
            if let Some(name) = table
                .get("package")
                .and_then(toml::Value::as_table)
                .and_then(|pkg| pkg.get("name"))
                .and_then(toml::Value::as_str)
            {
                specs.push(BinarySpec {
                    name: name.to_owned(),
                    source_name: name.to_owned(),
                    description: if package_desc.is_empty() {
                        default_crate_description(name)
                    } else {
                        package_desc.to_owned()
                    },
                    crate_path,
                    default_dest: metadata_dest.unwrap_or_else(|| default_loc(name)),
                });
            }
        }
    }

    specs
}

fn parse_destination_override(dest: &str) -> Option<BinaryLocation> {
    match dest.trim().to_lowercase().as_str() {
        "user" | "local" | "userlocalbin" | "~/.local/bin" => Some(BinaryLocation::UserLocalBin),
        "system" | "systembin" | "/usr/bin" => Some(BinaryLocation::SystemBin),
        _ => None,
    }
}

pub fn default_crate_description(name: &str) -> String {
    let trimmed = name.strip_prefix("babydra-").unwrap_or(name);
    let mut words = trimmed
        .split(['-', '_'])
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>();

    if words.is_empty() {
        "Application binary".to_string()
    } else {
        words.push("component".to_string());
        words.join(" ")
    }
}

fn executable_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_executable(path))
        .collect()
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(_path: &Path) -> bool {
    false
}

fn repository_root_for(root: &Path) -> PathBuf {
    if root.join(".git").exists() {
        return root.to_path_buf();
    }
    for ancestor in root.ancestors().skip(1) {
        if ancestor.join(".git").exists() {
            return ancestor.to_path_buf();
        }
    }
    root.to_path_buf()
}

fn git_show(repo_root: &Path, git_ref: &str, file_path: &str) -> Result<String, ()> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["show", &format!("{git_ref}:{file_path}")])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| ())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|_| ())
    } else {
        Err(())
    }
}

fn git_manifest_paths(repo_root: &Path, git_ref: &str) -> Vec<String> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["ls-tree", "-r", "--name-only", git_ref])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    let Ok(output) = output else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.ends_with("Cargo.toml"))
        .map(str::to_owned)
        .collect()
}

fn available_branches(repo_root: &Path) -> Vec<String> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["branch", "-a", "--format=%(refname:short)"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    let Ok(output) = output else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn discovers_default_and_explicit_binary_targets() {
        let temp_dir = std::env::temp_dir().join(format!(
            "babydra_test_discovery_{}_{}",
            std::process::id(),
            "targets"
        ));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        fs::write(
            temp_dir.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/*\"]\n",
        )
        .unwrap();

        let app1_dir = temp_dir.join("crates/app1");
        fs::create_dir_all(app1_dir.join("src")).unwrap();
        fs::write(
            app1_dir.join("Cargo.toml"),
            "[package]\nname = \"app1\"\nversion = \"0.1.0\"\ndescription = \"Primary app\"\n",
        )
        .unwrap();
        fs::write(app1_dir.join("src/main.rs"), "fn main() {}\n").unwrap();

        let greeter_dir = temp_dir.join("crates/custom-greeter");
        fs::create_dir_all(greeter_dir.join("src")).unwrap();
        fs::write(
            greeter_dir.join("Cargo.toml"),
            "[package]\nname = \"custom-greeter\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(greeter_dir.join("src/main.rs"), "fn main() {}\n").unwrap();

        let binaries = initial_binaries_list(&temp_dir, &temp_dir.join("target/release"));
        assert_eq!(binaries.len(), 2);

        let app1 = binaries.iter().find(|b| b.name == "app1").unwrap();
        assert_eq!(app1.default_dest, BinaryLocation::UserLocalBin);
        assert_eq!(app1.description, "Primary app");

        let greeter = binaries.iter().find(|b| b.name == "custom-greeter").unwrap();
        assert_eq!(greeter.default_dest, BinaryLocation::SystemBin);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn defaults_are_name_agnostic() {
        assert_eq!(default_loc("my-daemon"), BinaryLocation::UserLocalBin);
        assert_eq!(default_loc("custom-greeter"), BinaryLocation::SystemBin);
    }

    #[test]
    fn ignores_nested_cargo_projects_when_root_is_not_a_workspace() {
        let temp_dir = std::env::temp_dir().join(format!(
            "babydra_test_discovery_{}_{}",
            std::process::id(),
            "nested_ignored"
        ));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let old_project = temp_dir.join(".old_project/subapp");
        fs::create_dir_all(old_project.join("src")).unwrap();
        fs::write(
            old_project.join("Cargo.toml"),
            "[package]\nname = \"legacy-app\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(old_project.join("src/main.rs"), "fn main() {}\n").unwrap();

        let binaries = initial_binaries_list(&temp_dir, &temp_dir.join("target/release"));
        assert!(binaries.is_empty());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
