use crate::widgets::state::HostsWidget;
use babydra_ui_kit::components::modal::PasswordDialog;
use gtk4::prelude::*;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

/// Load hosts file.
fn load_hosts_file(buffer: &gtk4::TextBuffer, status_badge: &gtk4::Label) {
    match fs::read_to_string("/etc/hosts") {
        Ok(content) => {
            buffer.set_text(&content);
            status_badge.set_text("/etc/hosts");
        }
        Err(e) => {
            status_badge.set_text(&format!("Error reading /etc/hosts: {}", e));
        }
    }
}

/// Save hosts file.
fn save_hosts_file(content: &str, password: &str) -> Result<(), String> {
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg("sh")
        .arg("-c")
        .arg("cat > /etc/hosts")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute sudo: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", password);
        let _ = stdin.write_all(content.as_bytes());
        let _ = stdin.flush();
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for process: {}", e))?;
    if output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(if err_msg.trim().is_empty() {
            "Incorrect password or permission denied".to_string()
        } else {
            err_msg.trim().to_string()
        })
    }
}

/// Wire events.
pub fn wire_events(widget: &HostsWidget, auth_dialog: PasswordDialog) {
    // Initial load
    load_hosts_file(&widget.text_buffer, &widget.status_badge);

    // Wire Reload button
    let buffer_reload = widget.text_buffer.clone();
    let badge_reload = widget.status_badge.clone();
    widget.reload_btn.connect_clicked(move |_| {
        load_hosts_file(&buffer_reload, &badge_reload);
    });

    // Wire Save button to open PasswordDialog
    let auth_dialog_rc = std::rc::Rc::new(auth_dialog);
    let auth_dialog_show = auth_dialog_rc.clone();
    widget.save_btn.connect_clicked(move |_| {
        auth_dialog_show.show_for(
            "Authentication Required",
            "Enter sudo password to save /etc/hosts:",
        );
    });

    // Wire Confirm inside PasswordDialog
    let buffer_save = widget.text_buffer.clone();
    let badge_save = widget.status_badge.clone();
    auth_dialog_rc.connect_submit(move |password_opt| {
        let password = match password_opt {
            Some(p) => p,
            None => return,
        };

        let start_iter = buffer_save.start_iter();
        let end_iter = buffer_save.end_iter();
        let content = buffer_save.text(&start_iter, &end_iter, true).to_string();

        let badge = badge_save.clone();
        badge.set_text("Saving /etc/hosts...");

        let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();

        std::thread::spawn(move || {
            let res = save_hosts_file(&content, &password);
            let _ = tx.send(res);
        });

        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(res) = rx.try_recv() {
                match res {
                    Ok(_) => badge.set_text("/etc/hosts saved successfully!"),
                    Err(err) => badge.set_text(&format!("Save failed: {}", err)),
                }
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    });
}
