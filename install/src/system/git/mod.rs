pub mod branches;
pub mod worktree;

use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub use branches::list_branches;
pub use worktree::{branch_worktree_dir, checkout_and_pull};

/// Runs a git command in `repo`, capturing stdout. No output reaches the TUI.
pub(crate) fn git(repo: &Path, args: &[&str]) -> Result<String> {
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
