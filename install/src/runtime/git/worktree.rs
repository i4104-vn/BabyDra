use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Returns the dedicated directory for checking out and building a branch
/// (e.g. `repo/branches/<branch>`).
pub fn branch_worktree_dir(repo: &Path, branch: &str) -> PathBuf {
    if branch.is_empty() {
        repo.to_path_buf()
    } else {
        repo.join("branches").join(branch)
    }
}

/// Prepares the dedicated container directory for building applications from the chosen branch.
/// Sets up the target/release directory and synchronizes distribution metadata.
pub fn checkout_and_pull(repo: &Path, branch: &str) -> Result<PathBuf> {
    if branch.is_empty() {
        return Ok(repo.to_path_buf());
    }

    let target_dir = branch_worktree_dir(repo, branch);
    std::fs::create_dir_all(&target_dir)
        .with_context(|| format!("failed to create directory: {:?}", target_dir))?;

    let target_release = target_dir.join("target").join("release");
    let _ = std::fs::create_dir_all(&target_release);

    // Sync workspace.toml to target_dir so manifest loader can find it
    let src_workspace_toml = repo.join("workspace.toml");
    if src_workspace_toml.exists() {
        let dst_workspace_toml = target_dir.join("workspace.toml");
        let _ = std::fs::copy(&src_workspace_toml, &dst_workspace_toml);
    }

    // Sync assets if present
    let src_assets = repo.join("assets");
    if src_assets.exists() {
        let dst_assets = target_dir.join("assets");
        if !dst_assets.exists() {
            let _ = crate::runtime::copy_recursive(&src_assets, &dst_assets);
        }
    }

    Ok(target_dir)
}

/// Clones or updates an individual application git repository for the specified branch.
pub fn clone_or_sync_app(
    app_name: &str,
    git_url: &str,
    branch: &str,
    target_dir: &Path,
) -> Result<PathBuf> {
    let branch_to_use = if branch.is_empty() { "main" } else { branch };

    if target_dir.join(".git").exists() {
        // Already cloned: fetch and checkout
        let _ = super::git(target_dir, &["fetch", "origin", branch_to_use]);
        let checkout_res = super::git(target_dir, &["checkout", branch_to_use])
            .or_else(|_| super::git(target_dir, &["checkout", "-B", branch_to_use, &format!("origin/{branch_to_use}")]));
        if checkout_res.is_ok() {
            let _ = super::git(target_dir, &["pull", "--ff-only", "origin", branch_to_use]);
        }
        return Ok(target_dir.to_path_buf());
    }

    if target_dir.exists() {
        let _ = std::fs::remove_dir_all(target_dir);
    }

    if let Some(parent) = target_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Try cloning specific branch
    let clone_res = Command::new("git")
        .args(["clone", "--depth", "1", "-b", branch_to_use, git_url, target_dir.to_str().unwrap()])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match clone_res {
        Ok(out) if out.status.success() => Ok(target_dir.to_path_buf()),
        _ => {
            // Fallback: clone default branch (e.g. main)
            let fb_res = Command::new("git")
                .args(["clone", "--depth", "1", git_url, target_dir.to_str().unwrap()])
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;
            if !fb_res.status.success() {
                bail!(
                    "Failed to clone {} from {}: {}",
                    app_name,
                    git_url,
                    String::from_utf8_lossy(&fb_res.stderr).trim()
                );
            }
            Ok(target_dir.to_path_buf())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_branch_worktree_dir() {
        let repo = Path::new("/test/repo");
        assert_eq!(
            branch_worktree_dir(repo, "release"),
            PathBuf::from("/test/repo/branches/release")
        );
        assert_eq!(branch_worktree_dir(repo, ""), PathBuf::from("/test/repo"));
    }
}
