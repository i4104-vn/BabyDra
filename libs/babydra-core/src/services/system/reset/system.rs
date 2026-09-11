//! System-wide process termination, console restoration, system path cleanup and reboot.

use crate::error::{CoreError, CoreResult};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

/// Executes a command with elevated privileges (`sudo -S`) and streams its output.
pub fn run_sudo_cmd(
    password: &str,
    cmd: &str,
    args: &[&str],
    sender: &Sender<String>,
) -> Result<(), String> {
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg("-p")
        .arg("")
        .arg(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn sudo {}: {}", cmd, e))?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", password);
        let _ = stdin.flush();
    }

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let sender_out = sender.clone();
    let out_th = std::thread::spawn(move || {
        if let Some(out) = stdout {
            let reader = BufReader::new(out);
            for line in reader.lines().flatten() {
                if !line.trim().is_empty() {
                    let _ = sender_out.send(format!("  {}", line));
                }
            }
        }
    });

    let sender_err = sender.clone();
    let err_th = std::thread::spawn(move || {
        if let Some(err) = stderr {
            let reader = BufReader::new(err);
            for line in reader.lines().flatten() {
                if !line.contains("[sudo] password") && !line.trim().is_empty() {
                    let _ = sender_err.send(format!("  {}", line));
                }
            }
        }
    });

    let _ = out_th.join();
    let _ = err_th.join();

    let status = child
        .wait()
        .map_err(|e| format!("Process error on {}: {}", cmd, e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command {} exited with code {:?}",
            cmd,
            status.code()
        ))
    }
}

/// Terminates active BabyDra and compositor processes.
pub fn stop_active_processes(sender: &Sender<String>) {
    let _ = sender.send("[1/6] Stopping active BabyDra and compositor processes...".into());
    crate::services::utils::pkill("babydra-", false);
    crate::services::utils::pkill("labwc", true);
    crate::services::utils::pkill("cage", true);
    let _ = sender.send("  Processes stopped.".into());
}

/// Restores the default systemd login target and unmasks the virtual terminal console login.
pub fn restore_systemd_console(password: &str, sender: &Sender<String>) {
    let _ = sender
        .send("[2/6] Restoring Arch Linux TTY login console & disabling display manager...".into());
    let _ = run_sudo_cmd(password, "systemctl", &["stop", "greetd.service"], sender);
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &["disable", "greetd.service"],
        sender,
    );

    // Unmask tty2-6 and enable tty1 console
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &[
            "unmask",
            "getty@tty2.service",
            "getty@tty3.service",
            "getty@tty4.service",
            "getty@tty5.service",
            "getty@tty6.service",
        ],
        sender,
    );
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &["enable", "getty@tty1.service"],
        sender,
    );
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &["set-default", "multi-user.target"],
        sender,
    );
    let _ = sender.send("  Arch Linux multi-user TTY login target restored.".into());
}

/// Removes BabyDra system-level files and configurations.
pub fn clean_system_files(password: &str, sender: &Sender<String>) {
    let _ = sender.send("[3/6] Cleaning system-wide BabyDra directories and modules...".into());
    let paths_to_remove = [
        "/usr/bin/babydra-greeter",
        "/usr/share/babydra",
        "/var/lib/babydra",
        "/etc/modules-load.d/i2c.conf",
        "/etc/tmpfiles.d/babydra-perf.conf",
        "/etc/greetd",
    ];
    for path in paths_to_remove {
        let _ = run_sudo_cmd(password, "rm", &["-rf", path], sender);
        let _ = sender.send(format!("  Removed system path: {}", path));
    }
}

/// Rebuilds font cache and finalizes.
pub fn finalize_system(sender: &Sender<String>) {
    let _ = sender.send("[6/6] Finalizing system state...".into());
    let _ = Command::new("fc-cache").arg("-r").output();
    let _ = sender.send("  Font cache refreshed.".into());
}

/// Requests an immediate system reboot.
pub fn reboot_system(password: Option<&str>) -> CoreResult<()> {
    if let Some(pwd) = password {
        let mut child = Command::new("sudo")
            .arg("-S")
            .arg("systemctl")
            .arg("reboot")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| CoreError::msg(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = writeln!(stdin, "{}", pwd);
            let _ = stdin.flush();
        }
        let _ = child.wait();
        Ok(())
    } else {
        Command::new("systemctl")
            .arg("reboot")
            .spawn()
            .map(|_| ())
            .map_err(|e| CoreError::msg(e.to_string()))
    }
}
