use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn get_home_dir() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/home/user"))
}

pub fn get_current_user() -> String {
    env::var("USER").unwrap_or_else(|_| "user".to_string())
}

pub fn get_local_bin_dir() -> PathBuf {
    get_home_dir().join(".local/bin")
}

pub fn get_applications_dir() -> PathBuf {
    get_home_dir().join(".local/share/applications")
}

pub fn get_babydra_dir() -> PathBuf {
    get_home_dir().join(".babydra")
}

pub fn update_desktop_db(dir: &Path) {
    let _ = Command::new("update-desktop-database")
        .arg(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

pub fn set_xdg_mime(mime: &str, desktop: &str) {
    let _ = Command::new("xdg-mime")
        .args(["default", desktop, mime])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

pub fn refresh_fc_cache() {
    let _ = Command::new("fc-cache")
        .arg("-f")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

pub fn gsettings_set(schema: &str, key: &str, value: &str) {
    let _ = Command::new("gsettings")
        .args(["set", schema, key, value])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

pub fn gsettings_reset(schema: &str, key: &str) {
    let _ = Command::new("gsettings")
        .args(["reset", schema, key])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
