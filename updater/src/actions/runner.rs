use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

#[derive(Debug, Clone)]
pub enum LogMessage {
    Line(String),
    Step(String),
    Success(String),
    Error(String),
    Done(i32),
}

pub struct CommandRunner {
    tx: Sender<LogMessage>,
}

impl CommandRunner {
    pub fn new(tx: Sender<LogMessage>) -> Self {
        Self { tx }
    }

    pub fn log(&self, msg: impl Into<String>) {
        self.send(LogMessage::Line(msg.into()));
    }

    pub fn step(&self, msg: impl Into<String>) {
        let text = msg.into();
        self.send(LogMessage::Step(format!("==> {}", text)));
    }

    pub fn success(&self, msg: impl Into<String>) {
        self.send(LogMessage::Success(format!("[OK] {}", msg.into())));
    }

    pub fn error(&self, msg: impl Into<String>) {
        self.send(LogMessage::Error(format!("[ERROR] {}", msg.into())));
    }

    pub fn done(&self, code: i32) {
        self.send(LogMessage::Done(code));
    }

    /// Execute an external command and stream output line by line
    pub fn run_cmd(&self, prog: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
        self.run_cmd_with_input(prog, args, cwd, None)
    }

    /// Execute a command with optional stdin while streaming stdout/stderr.
    pub fn run_cmd_with_input(
        &self,
        prog: &str,
        args: &[&str],
        cwd: Option<&Path>,
        input: Option<&[u8]>,
    ) -> Result<(), String> {
        self.log(format!("$ {}", format_command(prog, args)));

        let mut command = Command::new(prog);
        command.args(args);
        if let Some(dir) = cwd {
            command.current_dir(dir);
        }

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command.stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });

        let mut child = command.spawn().map_err(|e| {
            let err = format!("Failed to spawn '{}': {}", prog, e);
            self.error(&err);
            err
        })?;

        if let Some(input) = input {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input).map_err(|e| {
                    let err = format!("Failed to write to '{}': {}", prog, e);
                    self.error(&err);
                    err
                })?;
            }
        }

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let tx_out = self.tx.clone();
        let tx_err = self.tx.clone();

        let t_out = thread::spawn(move || {
            if let Some(out) = stdout {
                let reader = BufReader::new(out);
                for line in reader.lines().map_while(Result::ok) {
                    let _ = tx_out.send(LogMessage::Line(line));
                }
            }
        });

        let t_err = thread::spawn(move || {
            if let Some(err) = stderr {
                let reader = BufReader::new(err);
                for line in reader.lines().map_while(Result::ok) {
                    let _ = tx_err.send(LogMessage::Line(line));
                }
            }
        });

        let _ = t_out.join();
        let _ = t_err.join();

        let status = child.wait().map_err(|e| {
            let err = format!("Failed to wait for '{}': {}", prog, e);
            self.error(&err);
            err
        })?;

        if status.success() {
            Ok(())
        } else {
            let code = status.code().unwrap_or(-1);
            let err = format!("Command '{}' failed with exit code {}", prog, code);
            self.error(&err);
            Err(err)
        }
    }

    /// Run command with sudo
    pub fn run_sudo(&self, prog: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
        let mut sudo_args = vec![prog];
        sudo_args.extend_from_slice(args);
        self.run_cmd("sudo", &sudo_args, cwd)
    }

    /// Run a command with sudo and provide data through its stdin.
    #[allow(dead_code)]
    pub fn run_sudo_with_input(
        &self,
        prog: &str,
        args: &[&str],
        cwd: Option<&Path>,
        input: &[u8],
    ) -> Result<(), String> {
        let mut sudo_args = vec![prog];
        sudo_args.extend_from_slice(args);
        self.run_cmd_with_input("sudo", &sudo_args, cwd, Some(input))
    }

    fn send(&self, message: LogMessage) {
        let _ = self.tx.send(message);
    }
}

fn format_command(prog: &str, args: &[&str]) -> String {
    std::iter::once(prog)
        .chain(args.iter().copied())
        .map(shell_quote)
        .collect::<Vec<_>>()
        .join(" ")
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || "_./-".contains(character))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}
