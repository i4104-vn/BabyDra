//! Git operations for the installer's "pick a branch" step.
//!
//! The installer shows the full list of branches (local + remote), lets the
//! user pick one, then at install time checks it out, pulls the latest code,
//! builds the workspace and copies the freshly built binaries — mirroring
//! what `scripts/install.sh` does for the branch-based flow.

use anyhow::{bail, Context, Result};
use std::path::Path;
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

/// Checks out `branch` in the repository and pulls the latest code.
/// Handles uncommitted working tree changes gracefully with stash/force fallback.
pub fn checkout_and_pull(repo: &Path, branch: &str) -> Result<()> {
    // 1. Fetch remote branch first to ensure we have the latest commits
    let _ = git(repo, &["fetch", "origin", branch]);

    let current = git(repo, &["branch", "--show-current"])
        .unwrap_or_default()
        .trim()
        .to_string();

    if current != branch {
        // Try regular checkout
        if let Err(orig_err) = git(repo, &["checkout", branch]) {
            // If checkout failed (e.g. dirty working tree with uncommitted files),
            // stash changes to avoid blocking branch switch.
            let _ = git(repo, &["stash", "push", "-u", "-m", "installer-autostash"]);

            // Try checkout again after stash
            if let Err(_) = git(repo, &["checkout", branch]) {
                // If local branch doesn't exist or is in detached state, checkout from origin/<branch>
                git(repo, &["checkout", "-B", branch, &format!("origin/{branch}")])
                    .or_else(|_| git(repo, &["checkout", "-f", branch]))
                    .with_context(|| format!("{orig_err}"))?;
            }
        }
    }

    // Pull or fast-forward to latest remote origin/branch
    if let Err(_) = git(repo, &["pull", "origin", branch]) {
        // If pull fails due to local differences, reset to origin/branch
        let _ = git(repo, &["reset", "--hard", &format!("origin/{branch}")]);
    }

    Ok(())
}
