//! Safe command execution for the TUI installer.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

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

impl CmdOutput {
    fn from_output(output: Output) -> Self {
        Self {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

fn spawn_and_wait(mut cmd: Command, stdin_data: &[u8], what: &str) -> Result<CmdOutput> {
    let mut child = cmd
        .spawn()
        .with_context(|| format!("failed to spawn {what}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(stdin_data);
    }
    let output = child
        .wait_with_output()
        .with_context(|| format!("failed to wait for {what}"))?;
    Ok(CmdOutput::from_output(output))
}

impl SudoSession {
    /// Creates a session. `password: None` means "already root".
    pub fn new(password: Option<String>) -> Self {
        Self { password }
    }

    /// Whether we are running as root (no sudo needed).
    pub fn is_root() -> bool {
        crate::runtime::is_root()
    }

    /// Validates the stored password via `sudo -S -v`.
    pub fn preauth(&self) -> Result<()> {
        let Some(pwd) = &self.password else {
            return Ok(()); // root
        };
        let mut cmd = self.sudo_base();
        cmd.arg("-v");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        let output = spawn_and_wait(cmd, format!("{pwd}\n").as_bytes(), "sudo -v")?;
        if output.success {
            Ok(())
        } else {
            bail!("incorrect password or sudo unavailable (sudo -v failed)")
        }
    }

    /// Runs a command, feeding a newline through piped stdin when elevated,
    /// and captures stdout/stderr.
    pub fn run(&self, program: &str, args: &[&str]) -> Result<CmdOutput> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        spawn_and_wait(cmd, b"\n", program)
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

        spawn_and_wait(cmd, format!("{pwd}\n").as_bytes(), "sudo command")
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

/// Trims captured output to the last `max_lines` non-empty lines.
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
