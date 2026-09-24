use std::path::{Path, PathBuf};
use std::process::Command;

use super::sudo::tail_lines;

pub fn find_workspace_root() -> PathBuf {
    // 1. Try git rev-parse --show-toplevel
    if let Ok(out) = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
    {
        if out.status.success() {
            let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path_str.is_empty() {
                let p = PathBuf::from(path_str);
                if p.is_dir() {
                    return p;
                }
            }
        }
    }

    // 2. Search upwards from current directory for .git directory
    let mut current = std::env::current_dir().unwrap_or_default();
    loop {
        if current.join(".git").exists() {
            return current;
        }
        if !current.pop() {
            break;
        }
    }

    // 3. Check parent if we are inside install/
    let cur = std::env::current_dir().unwrap_or_default();
    if cur.file_name().and_then(|s| s.to_str()) == Some("install") {
        if let Some(parent) = cur.parent() {
            return parent.to_path_buf();
        }
    }

    cur
}

/// Runs `cargo clean` and `cargo build --release --workspace` in the workspace root.
pub fn build_workspace(workspace_root: &Path) -> (bool, Vec<String>) {
    let _ = Command::new("cargo")
        .current_dir(workspace_root)
        .args(["clean"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output();

    let output = Command::new("cargo")
        .current_dir(workspace_root)
        .args(["build", "--release", "--workspace"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();

    match output {
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            let tail = tail_lines(&format!("{stdout}\n{stderr}"), 15);
            (out.status.success(), tail)
        }
        Err(e) => (false, vec![format!("cargo failed to start: {e}")]),
    }
}

/// Runs `cargo clean` and `cargo build --release --workspace`, streaming each output line
/// live in real time to the provided logger callback.
pub fn build_workspace_streaming<F>(workspace_root: &Path, mut log: F) -> bool
where
    F: FnMut(crate::models::LogLevel, String),
{
    log(crate::models::LogLevel::Info, "Executing cargo clean...".into());
    let _ = Command::new("cargo")
        .current_dir(workspace_root)
        .args(["clean"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    log(
        crate::models::LogLevel::Info,
        format!(
            "Running cargo build --release --workspace in {}...",
            workspace_root.display()
        ),
    );

    let mut cmd = Command::new("cargo");
    cmd.current_dir(workspace_root);
    cmd.args(["build", "--release", "--workspace"]);

    let res = super::sudo::spawn_and_stream(cmd, None, move |is_err, line| {
        if line.contains("error:") || (is_err && line.starts_with("error[")) {
            log(crate::models::LogLevel::Error, line);
        } else if line.contains("warning:") {
            log(crate::models::LogLevel::Warn, line);
        } else {
            log(crate::models::LogLevel::Info, line);
        }
    });

    res.unwrap_or(false)
}

pub fn default_binary_source_dir(workspace_root: &Path) -> PathBuf {
    let release_dir = workspace_root.join("target").join("release");
    if release_dir.exists() {
        return release_dir;
    }
    let local_release = PathBuf::from("target/release");
    if local_release.exists() {
        return local_release;
    }
    workspace_root.join("target").join("release")
}

pub fn get_user_local_bin() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".local").join("bin"))
        .unwrap_or_else(|| PathBuf::from("/usr/local/bin"))
}

pub fn get_user_home() -> PathBuf {
    dirs::home_dir()
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("/root"))
}

/// Expands variables such as `$HOME`, `$CONFIG`, `$LOCAL_BIN`, `$DATA` in path strings.
pub fn expand_path(raw: &str) -> PathBuf {
    let home = get_user_home();
    let config = home.join(".config");
    let local_bin = get_user_local_bin();
    let data = home.join(".local/share");

    let expanded = raw
        .replace("$CONFIG", &config.to_string_lossy())
        .replace("$LOCAL_BIN", &local_bin.to_string_lossy())
        .replace("$DATA", &data.to_string_lossy())
        .replace("$HOME", &home.to_string_lossy())
        .replace("~", &home.to_string_lossy());

    PathBuf::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_workspace_root() {
        let root = find_workspace_root();
        assert!(
            root.join(".git").exists(),
            "Workspace root {:?} must contain .git",
            root
        );
    }

    #[test]
    fn test_expand_path() {
        let p = expand_path("$CONFIG/labwc");
        assert!(p.ends_with(".config/labwc"));
    }
}
