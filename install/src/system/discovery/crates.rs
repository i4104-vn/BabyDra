use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{BinaryItem, BinaryLocation};
use crate::system::get_user_local_bin;

/// The greeter installs to `/usr/bin` (greetd runs it as the `greeter` user);
/// everything else lands in `~/.local/bin`.
pub fn default_loc(name: &str) -> BinaryLocation {
    if name == "babydra-greeter" {
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

pub fn parse_crate_cargo_toml(path: &Path, fallback_dir_name: &str) -> (String, String) {
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

pub fn default_crate_description(name: &str) -> String {
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
