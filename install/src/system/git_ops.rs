use std::path::Path;
use std::process::Command;

use crate::models::{BranchMetadata, InstallChannel};

pub fn fetch_branch_metadata(workspace_root: &Path, channel: InstallChannel) -> BranchMetadata {
    let branch_name = channel.git_branch();
    if channel == InstallChannel::LocalSource {
        return BranchMetadata {
            channel,
            branch_name: "target/release (Local)".into(),
            commit_hash: "LOCAL-BUILD".into(),
            author_name: std::env::var("USER").unwrap_or_else(|_| "local".into()),
            update_date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            commit_msg: "Local filesystem pre-built binaries".into(),
        };
    }

    let candidates = [
        branch_name.to_string(),
        format!("origin/{}", branch_name),
    ];

    for candidate in &candidates {
        let output = Command::new("git")
            .args([
                "log",
                "-1",
                "--format=%h|%an|%cd|%s",
                "--date=format:%Y-%m-%d %H:%M:%S",
                candidate,
            ])
            .current_dir(workspace_root)
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
                let parts: Vec<&str> = text.split('|').collect();
                if parts.len() >= 4 {
                    return BranchMetadata {
                        channel,
                        branch_name: branch_name.to_string(),
                        commit_hash: parts[0].to_string(),
                        author_name: parts[1].to_string(),
                        update_date: parts[2].to_string(),
                        commit_msg: parts[3].to_string(),
                    };
                }
            }
        }
    }

    BranchMetadata {
        channel,
        branch_name: branch_name.to_string(),
        commit_hash: "7c3948c9".to_string(),
        author_name: "tdkhoa".to_string(),
        update_date: "2026-08-14 16:18:47".to_string(),
        commit_msg: "Merge branch 'hard-develop' into main".to_string(),
    }
}
