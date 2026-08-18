//! Greeter login widget: user selection, password authentication, and submit.

mod render;

pub use render::build;

use gtk4::{Box as GtkBox, Button, DropDown, Label, PasswordEntry, Spinner};

pub struct LoginWidget {
    pub container: GtkBox,
    pub login_panel: GtkBox,
    pub user_dropdown: DropDown,
    pub users: Vec<String>,
    pub pass_entry: PasswordEntry,
    pub login_btn: Button,
    pub btn_spinner: Spinner,
    pub error_box: GtkBox,
    pub error_label: Label,
}

/// Helper function to retrieve all system users excluding `root`.
/// Returns a list of usernames filtered to normal/login users.
pub fn get_system_users() -> Vec<String> {
    let mut normal_users = Vec::new();
    let mut fallback_users = Vec::new();

    if let Ok(content) = std::fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 {
                let username = parts[0];
                let uid: u32 = parts[2].parse().unwrap_or(0);
                let shell = parts[6];

                // Exclude root and greeter system daemon account
                if username == "root" || username == "greeter" {
                    continue;
                }

                let is_nologin = shell.ends_with("nologin")
                    || shell.ends_with("false")
                    || shell.ends_with("git-shell");

                // Standard non-root login users have UID >= 1000 and UID != 65534 (nobody)
                if uid >= 1000 && uid != 65534 && !is_nologin {
                    normal_users.push(username.to_string());
                } else if !is_nologin {
                    fallback_users.push(username.to_string());
                }
            }
        }
    }

    let mut result = if !normal_users.is_empty() {
        normal_users
    } else if !fallback_users.is_empty() {
        fallback_users
    } else {
        vec![std::env::var("USER").unwrap_or_else(|_| "user".to_string())]
    };

    result.sort();
    result.dedup();
    result
}
