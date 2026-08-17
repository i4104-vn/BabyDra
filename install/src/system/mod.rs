pub mod fs_ops;
pub mod git;
pub mod initializers;
pub mod process;
pub mod sudo;

use std::path::{Path, PathBuf};
use std::process::Command;

pub use fs_ops::{copy_recursive, format_size, safe_copy_binary};
pub use git::{checkout_and_pull, list_branches};
pub use initializers::{
    initial_binaries_list, initial_configs_themes_options, initial_display_manager_options,
    initial_package_options, initial_variant_options, initial_varlib_options,
    update_binaries_status,
};
pub use process::{is_root, stop_process};
pub use sudo::{tail_lines, CmdOutput, SudoSession};

pub fn find_workspace_root() -> PathBuf {
    // 1. Try git rev-parse --show-toplevel
    if let Ok(out) = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
    {
        if out.status.success() {
            let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path_str.is_empty() {
                let p = PathBuf::from(path_str);
                if p.is_dir() {
                    return p;
                }
            }
        }
    }

    // 2. Search upwards from current directory for .git directory or workspace Cargo.toml
    let mut current = std::env::current_dir().unwrap_or_default();
    loop {
        if current.join(".git").exists() {
            return current;
        }
        if !current.pop() {
            break;
        }
    }

    // 3. Check parent if we are inside install/
    let cur = std::env::current_dir().unwrap_or_default();
    if cur.file_name().and_then(|s| s.to_str()) == Some("install") {
        if let Some(parent) = cur.parent() {
            return parent.to_path_buf();
        }
    }

    cur
}

/// Runs `cargo clean` and `cargo build --release --workspace` in the workspace root, capturing
/// output so nothing leaks onto the TUI. Returns `(success, last_lines)`.
pub fn build_workspace(workspace_root: &Path) -> (bool, Vec<String>) {
    let _ = Command::new("cargo")
        .current_dir(workspace_root)
        .args(["clean"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output();

    let output = Command::new("cargo")
        .current_dir(workspace_root)
        .args(["build", "--release", "--workspace"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();

    match output {
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            let tail = tail_lines(&format!("{stdout}\n{stderr}"), 15);
            (out.status.success(), tail)
        }
        Err(e) => (false, vec![format!("cargo failed to start: {e}")]),
    }
}

pub fn default_binary_source_dir(workspace_root: &Path) -> PathBuf {
    let release_dir = workspace_root.join("target").join("release");
    if release_dir.exists() {
        return release_dir;
    }
    let local_release = PathBuf::from("target/release");
    if local_release.exists() {
        return local_release;
    }
    workspace_root.join("target").join("release")
}

pub fn get_user_local_bin() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".local").join("bin"))
        .unwrap_or_else(|| PathBuf::from("/usr/local/bin"))
}

pub fn get_user_home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/i4104"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_workspace_root() {
        let root = find_workspace_root();
        assert!(root.join(".git").exists(), "Workspace root {:?} must contain .git", root);
    }
}
