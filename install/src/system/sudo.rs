//! Safe command execution for the TUI installer.
//!
//! # Why this exists
//!
//! The TUI runs in raw mode + alternate screen. Child processes that inherit
//! the terminal (via `Command::status()`) dump their output straight onto the
//! form and steal stdin — this breaks the TUI and makes the sudo password
//! prompt unreadable, which after repeated failures can lock the account.
//!
//! This module fixes that by:
//!
//! 1. **Pre-authenticating sudo once** with the password the user typed into
//!    a TUI modal (`sudo -S -v`), before any task runs.
//! 2. Running every sudo command with `sudo -S -p ''` and the password fed
//!    through a piped stdin — never a TTY prompt.
//! 3. **Redirecting stdout/stderr** of every child process (captured and
//!    returned, or dropped) so nothing leaks onto the alternate screen.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

/// Number of failed password attempts allowed before aborting the install.
/// Prevents account lockouts from endless retries.
pub const MAX_PASSWORD_ATTEMPTS: u32 = 3;

/// Holds the user-provided sudo password in memory for the duration of the
/// install. Password is never written to disk and never passed as an argv.
pub struct SudoSession {
    /// `None` when running as root (no password needed).
    password: Option<String>,
}

/// Result of a captured command run.
pub struct CmdOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl SudoSession {
    /// Creates a session. `password: None` means "already root".
    pub fn new(password: Option<String>) -> Self {
        Self { password }
    }

    /// Whether we are running as root (no sudo needed).
    pub fn is_root() -> bool {
        crate::system::is_root()
    }

    /// Validates the stored password via `sudo -S -v`.
    ///
    /// Runs before any installation task so the password is verified exactly
    /// once — a wrong password shows a clear error in the TUI instead of
    /// failing halfway through the install.
    pub fn preauth(&self) -> Result<()> {
        let Some(pwd) = &self.password else {
            return Ok(()); // root
        };
        let mut cmd = self.sudo_base();
        cmd.arg("-v");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        let mut child = cmd
            .spawn()
            .context("failed to spawn sudo -v (is sudo installed?)")?;
        // Best-effort write: sudo may already have a cached credential and
        // close stdin without reading — a BrokenPipe here is harmless.
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(format!("{pwd}\n").as_bytes());
        }
        let status = child.wait().context("failed to wait for sudo -v")?;

        if status.success() {
            Ok(())
        } else {
            bail!("incorrect password or sudo unavailable (sudo -v failed)")
        }
    }

    /// Runs a command, feeding the sudo password through piped stdin when
    /// elevated, and captures stdout/stderr.
    ///
    /// This is the safe replacement for `Command::status()` everywhere in the
    /// installer: no output reaches the TUI, no TTY prompt is shown.
    pub fn run(&self, program: &str, args: &[&str]) -> Result<CmdOutput> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .with_context(|| format!("failed to spawn {program}"))?;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(b"\n");
        }

        let output = child
            .wait_with_output()
            .with_context(|| format!("failed to wait for {program}"))?;

        Ok(CmdOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    /// Runs a command as root (via `sudo -S` when not root), capturing output.
    pub fn run_root(&self, args: &[&str]) -> Result<CmdOutput> {
        if args.is_empty() {
            bail!("run_root called with no arguments");
        }
        if Self::is_root() {
            return self.run(args[0], &args[1..]);
        }
        let Some(pwd) = &self.password else {
            bail!("sudo password is required but was not provided");
        };

        let mut cmd = self.sudo_base();
        cmd.args(args);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().context("failed to spawn sudo command")?;
        // Best-effort write (see `preauth`): with cached credentials sudo
        // skips reading stdin entirely.
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(format!("{pwd}\n").as_bytes());
        }

        let output = child
            .wait_with_output()
            .context("failed to wait for sudo command")?;

        Ok(CmdOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    /// Runs a root command, discarding all output (safe for silent ops).
    pub fn run_root_quiet(&self, args: &[&str]) -> Result<()> {
        self.run_root(args)?;
        Ok(())
    }

    /// Writes `content` to a root-owned file: writes a temp file as the user,
    /// then `sudo cp` it into place (avoids fragile `sudo sh -c echo` quoting).
    pub fn write_root_file(&self, path: &Path, content: &str) -> Result<()> {
        let parent = path.parent().context("file has no parent dir")?;
        let tmp = std::env::temp_dir().join(format!(
            "babydra_install_{}_{}",
            std::process::id(),
            path.file_name().and_then(|n| n.to_str()).unwrap_or("tmp")
        ));
        std::fs::write(&tmp, content)
            .with_context(|| format!("failed to write temp file {:?}", tmp))?;

        self.run_root_quiet(&["mkdir", "-p", parent.to_str().unwrap_or("/")])?;
        let out = self.run_root(&[
            "cp",
            tmp.to_str().unwrap_or(""),
            path.to_str().unwrap_or(""),
        ])?;
        let _ = std::fs::remove_file(&tmp);
        if out.success {
            Ok(())
        } else {
            bail!("failed to write {:?} via sudo", path)
        }
    }

    /// Builds `sudo -S -p ''` — reads the password from stdin, no prompt.
    fn sudo_base(&self) -> Command {
        let mut cmd = Command::new("sudo");
        cmd.args(["-S", "-p", ""]);
        cmd
    }
}

impl Default for SudoSession {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Trims captured output to the last `max_lines` non-empty lines — used when
/// surfacing command output into the TUI log without flooding it.
pub fn tail_lines(s: &str, max_lines: usize) -> Vec<String> {
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .rev()
        .take(max_lines)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|l| l.to_string())
        .collect()
}
