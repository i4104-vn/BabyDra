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
    let candidates = [
        std::env::current_dir().unwrap_or_default(),
        std::env::current_dir()
            .unwrap_or_default()
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default(),
    ];

    for dir in &candidates {
        if dir.join("Cargo.toml").exists() && dir.join("crates").is_dir() {
            return dir.clone();
        }
    }

    std::env::current_dir().unwrap_or_default()
}

/// Runs `cargo build --release --workspace` in the workspace root, capturing
/// output so nothing leaks onto the TUI. Returns `(success, last_lines)`.
pub fn build_workspace(workspace_root: &Path) -> (bool, Vec<String>) {
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
