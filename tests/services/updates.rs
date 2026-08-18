//! Integration tests: update service pure logic.
//!
//! Verifies pacman progress-line parsing and update-state persistence through
//! the public API. These helpers were extracted from the large `updates/mod.rs`
//! so they can be tested without running the system updater.

use babydra_core::models::system_update::{PackageUpdate, UpdateStatus};
use babydra_core::services::system::updates::{get_update_log_path, parse_pacman_prog};

#[test]
fn parses_pacman_progress_line() {
    let parsed = parse_pacman_prog("(1/3) installing firefox...");
    assert_eq!(parsed, Some((1, 3, "firefox...".to_string())));
}

#[test]
fn parses_pacman_progress_with_spaces() {
    let parsed = parse_pacman_prog("(12/45) upgrading  linux-firmware");
    assert_eq!(parsed, Some((12, 45, "linux-firmware".to_string())));
}

#[test]
fn ignores_lines_without_progress() {
    assert_eq!(parse_pacman_prog("loading packages..."), None);
    assert_eq!(parse_pacman_prog(""), None);
}

#[test]
fn malformed_progress_falls_back_to_action_parse() {
    // Non-numeric progress falls back to the action-based branch and still
    // extracts the package name.
    assert_eq!(
        parse_pacman_prog("(a/b) installing foo"),
        Some((1, 1, "foo".to_string()))
    );
    // A line with no progress and no known action yields nothing.
    assert_eq!(parse_pacman_prog("random noise"), None);
}

#[test]
fn parses_fallback_action_style() {
    // Handles the "upgrading <name>" fallback branch too.
    let parsed = parse_pacman_prog("(2/3) upgrading firefox");
    assert_eq!(parsed, Some((2, 3, "firefox".to_string())));
}

#[test]
fn update_log_path_is_in_temp_dir() {
    let p = get_update_log_path();
    assert!(
        p.to_string_lossy().contains("babydra-update"),
        "log path: {}",
        p.display()
    );
}

#[test]
fn update_status_serde_roundtrip() {
    let pkg = PackageUpdate {
        name: "firefox".into(),
        old_version: "1.0".into(),
        new_version: "2.0".into(),
        status: UpdateStatus::Pending,
    };
    let json = serde_json::to_string(&pkg).unwrap();
    let back: PackageUpdate = serde_json::from_str(&json).unwrap();
    assert_eq!(back.name, "firefox");
    assert_eq!(back.status, UpdateStatus::Pending);
}
