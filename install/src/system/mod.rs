pub mod fs_ops;
pub mod initializers;
pub mod process;

use std::path::{Path, PathBuf};

pub use fs_ops::{copy_recursive, format_size, safe_copy_binary};
pub use initializers::{
    initial_binaries_list, initial_configs_themes_options, initial_display_manager_options,
    initial_package_options, initial_varlib_options, update_binaries_status,
};
pub use process::{is_root, stop_process};

pub fn find_workspace_root() -> PathBuf {
    let candidates = [
        std::env::current_dir().unwrap_or_default(),
        std::env::current_dir().unwrap_or_default().parent().map(|p| p.to_path_buf()).unwrap_or_default(),
        PathBuf::from("/home/i4104/BabyDra"),
    ];

    for dir in &candidates {
        if dir.join("Cargo.toml").exists() && dir.join("crates").is_dir() {
            return dir.clone();
        }
    }

    std::env::current_dir().unwrap_or_default()
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
