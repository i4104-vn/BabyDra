//! Git operations for the installer's "pick a branch" step.
//!
//! The installer shows the full list of branches (local + remote), lets the
//! user pick one, then at install time checks it out, pulls the latest code,
//! builds the workspace and copies the freshly built binaries — mirroring
//! what `scripts/install.sh` does for the branch-based flow.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::models::BranchItem;

/// Runs a git command in `repo`, capturing stdout. No output reaches the TUI.
fn git(repo: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(repo)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("failed to run git {:?} in {:?}", args, repo))?;

    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Discovers local + remote branches in the workspace repository.
pub fn list_branches(repo: &Path) -> Vec<BranchItem> {
    let mut items = Vec::new();

    // Local branches: `git branch --format=%(refname:short)`.
    let local: Vec<String> = git(repo, &["branch", "--format=%(refname:short)"])
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    // Remote branches (`remotes/origin/*`), fetch first so the list is fresh.
    // Bounded by a timeout so a dead network can never hang the TUI startup.
    {
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        let repo_owned = repo.to_path_buf();
        std::thread::spawn(move || {
            let _ = git(&repo_owned, &["fetch", "--prune", "origin"]);
            let _ = tx.send(());
        });
        let _ = rx.recv_timeout(std::time::Duration::from_secs(8));
    }
    let remote: Vec<String> = git(repo, &["branch", "-r", "--format=%(refname:short)"])
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_string())
        .filter_map(|l| l.strip_prefix("origin/").map(|s| s.to_string()))
        .filter(|l| !l.is_empty() && l != "HEAD")
        .collect();

    let current = git(repo, &["branch", "--show-current"])
        .unwrap_or_default()
        .trim()
        .to_string();

    // Merge local + remote into one list. `main` is excluded: it only hosts
    // the installer + docs, so there is no source code to build from.
    let mut names: Vec<String> = local.clone();
    for r in &remote {
        if !names.contains(r) {
            names.push(r.clone());
        }
    }
    names.retain(|n| n != "main");
    names.sort_by(|a, b| {
        let a_current = *a == current;
        let b_current = *b == current;
        b_current
            .cmp(&a_current)
            .then_with(|| a.to_lowercase().cmp(&b.to_lowercase()))
    });

    for name in names {
        let is_current = name == current;
        items.push(BranchItem {
            has_remote: remote.contains(&name),
            is_current,
            selected: is_current,
            name,
        });
    }

    items
}

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

    // Create a symlink `branchs -> branches` at the repo root so both folder names work
    #[cfg(unix)]
    {
        let branchs_symlink = repo.join("branchs");
        if !branchs_symlink.exists() && !branchs_symlink.is_symlink() {
            let _ = std::os::unix::fs::symlink("branches", &branchs_symlink);
        }
    }

    let target_dir = branches_dir.join(branch);

    // 1. Fetch the remote branch in the root repo
    let _ = git(repo, &["fetch", "origin", branch]);

    // 2. If target directory already exists and has a git reference, update it
    if target_dir.exists() && (target_dir.join(".git").exists() || target_dir.join("Cargo.toml").exists()) {
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
