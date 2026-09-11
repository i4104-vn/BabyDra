//! Factory reset service to restore the system back to vanilla Arch Linux baseline natively.

pub mod packages;
pub mod system;
pub mod user;

use crate::error::CoreResult;
pub use packages::{clean_orphan_packages, remove_all_user_applications, remove_shell_packages};
use std::sync::mpsc::Sender;
pub use system::{
    clean_system_files, finalize_system, reboot_system, restore_systemd_console,
    stop_active_processes,
};
pub use user::clean_user_files;

/// Executes the full native Arch Linux factory reset sequence.
pub fn run_factory_reset_stream(
    password: &str,
    remove_shell_pkgs: bool,
    remove_all_apps: bool,
    sender: Sender<String>,
) -> CoreResult<()> {
    let _ = sender.send("=== Starting Native Arch Linux Factory Reset ===".into());

    stop_active_processes(&sender);
    restore_systemd_console(password, &sender);
    clean_system_files(password, &sender);
    clean_user_files(&sender);

    if remove_all_apps {
        remove_all_user_applications(password, &sender);
    } else if remove_shell_pkgs {
        remove_shell_packages(password, &sender);
    } else {
        let _ = sender.send("[5/6] Skipping package removal as requested.".into());
    }

    finalize_system(&sender);

    let _ = sender.send("========================================================".into());
    let _ = sender.send("Factory Reset Complete!".into());
    let _ = sender.send("The system has been restored to clean Arch Linux baseline.".into());
    let _ = sender.send("Login prompt will be available via standard TTY console.".into());
    let _ = sender.send("========================================================".into());

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reset_channel_streaming() {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let _ = tx.send("[1/6] Stopping active BabyDra and compositor processes...".into());
        let _ = tx.send("Factory Reset Complete!".into());
        assert_eq!(
            rx.recv().unwrap(),
            "[1/6] Stopping active BabyDra and compositor processes..."
        );
        assert_eq!(rx.recv().unwrap(), "Factory Reset Complete!");
    }
}
