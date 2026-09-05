use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::git;

/// Returns the dedicated directory for checking out and building a branch
/// (e.g. `repo/branches/<branch>`).
pub fn branch_worktree_dir(repo: &Path, branch: &str) -> PathBuf {
    if branch.is_empty() {
        repo.to_path_buf()
    } else {
        repo.join("branches").join(branch)
    }
}

/// Pulls and checks out `branch` into a dedicated folder `branches/<branch>`
/// using git worktree. This leaves the main repository's current branch completely
/// untouched, preventing branch disruption if installation is canceled.
pub fn checkout_and_pull(repo: &Path, branch: &str) -> Result<PathBuf> {
    if branch.is_empty() {
        return Ok(repo.to_path_buf());
    }

    let branches_dir = repo.join("branches");
    std::fs::create_dir_all(&branches_dir)
        .with_context(|| format!("failed to create directory: {:?}", branches_dir))?;

    let target_dir = branches_dir.join(branch);

    // 1. Fetch the remote branch in the root repo
    let _ = git(repo, &["fetch", "origin", branch]);

    // 2. If target directory already exists and has a git reference, update it
    if target_dir.exists()
        && (target_dir.join(".git").exists() || target_dir.join("Cargo.toml").exists())
    {
        let _ = git(&target_dir, &["fetch", "origin", branch]);
        let _ = git(&target_dir, &["reset", "--hard", &format!("origin/{branch}")])
            .or_else(|_| git(&target_dir, &["checkout", "-B", branch, &format!("origin/{branch}")]))
            .or_else(|_| git(&target_dir, &["pull", "origin", branch]));
        return Ok(target_dir);
    }

    // If a stale or empty directory exists, clean it up first
    if target_dir.exists() {
        let _ = std::fs::remove_dir_all(&target_dir);
    }
    let _ = git(repo, &["worktree", "prune"]);

    let target_str = target_dir
        .to_str()
        .context("Invalid target directory path")?;

    // Create fresh worktree for this branch
    let add_res = git(
        repo,
        &[
            "worktree",
            "add",
            "-f",
            "-B",
            branch,
            target_str,
            &format!("origin/{branch}"),
        ],
    )
    .or_else(|_| git(repo, &["worktree", "add", "-f", target_str, branch]));

    add_res.with_context(|| {
        format!(
            "failed to create worktree for branch '{}' in {:?}",
            branch, target_dir
        )
    })?;

    // Pull or fast-forward inside the worktree
    let _ = git(&target_dir, &["pull", "origin", branch]);

    Ok(target_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_worktree_dir() {
        let repo = Path::new("/test/repo");
        assert_eq!(
            branch_worktree_dir(repo, "release"),
            PathBuf::from("/test/repo/branches/release")
        );
        assert_eq!(
            branch_worktree_dir(repo, ""),
            PathBuf::from("/test/repo")
        );
    }
}
