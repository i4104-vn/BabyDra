use std::path::Path;

use super::git;
use crate::models::BranchItem;

/// Discovers local + cached remote branches in the workspace repository.
///
/// This function is intentionally read-only. Fetching remote refs is handled
/// by [`refresh_branches`] so the TUI can start without waiting for a network
/// operation.
pub fn list_branches(repo: &Path) -> Vec<BranchItem> {
    let mut items = Vec::new();

    // Local branches: `git branch --format=%(refname:short)`.
    let local: Vec<String> = git(repo, &["branch", "--format=%(refname:short)"])
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    // Remote branches (`remotes/origin/*`) are read from the local git cache.
    // A refresh is deliberately not performed here; this function is called
    // while constructing the application state.
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

    // Merge local + remote into one list, then keep only refs containing a
    // workspace manifest. The distribution branch is not special-cased: a
    // future branch becomes installable automatically as soon as it contains
    // a Cargo workspace.
    let mut names: Vec<String> = local.clone();
    for r in &remote {
        if !names.contains(r) {
            names.push(r.clone());
        }
    }
    names.retain(|name| has_workspace_manifest(repo, name));
    names.sort_by(|a, b| {
        let a_current = a == &current;
        let b_current = b == &current;
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

/// Refreshes remote refs in the background and returns the resulting branch
/// list. The timeout bounds the lifetime of the fetch worker when the network
/// or remote is unavailable.
pub fn refresh_branches(repo: &Path) -> Vec<BranchItem> {
    let (tx, rx) = std::sync::mpsc::channel();
    let repo_owned = repo.to_path_buf();
    std::thread::spawn(move || {
        let _ = git(&repo_owned, &["fetch", "--prune", "origin"]);
        let _ = tx.send(());
    });
    let _ = rx.recv_timeout(std::time::Duration::from_secs(8));
    list_branches(repo)
}

fn has_workspace_manifest(repo: &Path, branch: &str) -> bool {
    git(repo, &["cat-file", "-e", &format!("{branch}:Cargo.toml")]).is_ok()
}
