use std::process::{Command, Stdio};

pub fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

/// Kills a process quietly. Output is discarded so nothing ever leaks onto
/// the raw-mode TUI alternate screen.
pub fn stop_process(name: &str) {
    let _ = Command::new("killall")
        .arg("-q")
        .arg(name)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
