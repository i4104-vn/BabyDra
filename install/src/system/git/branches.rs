use std::path::Path;

use super::git;
use crate::models::BranchItem;

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

fn has_workspace_manifest(repo: &Path, branch: &str) -> bool {
    git(repo, &["cat-file", "-e", &format!("{branch}:Cargo.toml")]).is_ok()
}
