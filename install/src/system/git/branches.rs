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
        let a_rel = a == "release";
        let b_rel = b == "release";
        b_rel
            .cmp(&a_rel)
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
