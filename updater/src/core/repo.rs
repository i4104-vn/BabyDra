use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_repo_root() -> PathBuf {
    // 1. Env variable override
    if let Ok(dir) = env::var("BABYDRA_ROOT") {
        let p = PathBuf::from(dir);
        if p.exists() {
            return p;
        }
    }

    // 2. Search upward from current working directory
    if let Ok(cwd) = env::current_dir() {
        if let Some(root) = check_ancestors_for_root(&cwd) {
            return root;
        }
    }

    // 3. Search upward from current executable path
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            if let Some(root) = check_ancestors_for_root(parent) {
                return root;
            }
        }
    }

    // Fallback to current directory
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn check_ancestors_for_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if is_repo_root(&current) {
            return Some(current);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

fn is_repo_root(path: &Path) -> bool {
    let cargo_toml = path.join("Cargo.toml");
    let has_workspace_toml = path.join("workspace.toml").exists();
    let has_crates_dir = path.join("crates").is_dir();

    if has_workspace_toml || has_crates_dir {
        return true;
    }

    if cargo_toml.exists() {
        if let Ok(content) = fs::read_to_string(&cargo_toml) {
            if content.contains("[workspace]") {
                return true;
            }
        }
    }

    false
}
