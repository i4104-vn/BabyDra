//! Integration tests: installer theme selection.
//!
//! Verifies `write_theme_selection` writes `theme.selection.id` into a
//! `babydra.conf` file while preserving unrelated keys and replacing any
//! previous selection value.

use babydra_installer::tasks::configs::write_theme_selection;
use std::fs;

fn temp_conf(suffix: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "babydra_inst_theme_{}_{}",
        std::process::id(),
        suffix
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir.join("babydra.conf")
}

#[test]
fn write_theme_selection_creates_file() {
    let conf = temp_conf("create");
    write_theme_selection(&conf, "babydra-blue").expect("write");

    let raw = fs::read_to_string(&conf).unwrap();
    assert!(raw.contains("id = \"babydra-blue\""), "raw: {raw}");

    let _ = fs::remove_dir_all(conf.parent().unwrap());
}

#[test]
fn write_theme_selection_preserves_existing_keys() {
    let conf = temp_conf("preserve");
    fs::write(
        &conf,
        "[power]\nprofile = \"balanced\"\n[theme]\nblur_radius = 30\n",
    )
    .unwrap();

    write_theme_selection(&conf, "babydra-default").expect("write");

    let raw = fs::read_to_string(&conf).unwrap();
    assert!(
        raw.contains("profile = \"balanced\""),
        "power preserved: {raw}"
    );
    assert!(raw.contains("blur_radius = 30"), "theme preserved: {raw}");
    assert!(raw.contains("id = \"babydra-default\""), "id set: {raw}");

    let _ = fs::remove_dir_all(conf.parent().unwrap());
}

#[test]
fn write_theme_selection_overwrites_previous_id() {
    let conf = temp_conf("overwrite");
    fs::write(
        &conf,
        "[theme]\nselection = { id = \"old-theme\", dark = true }\n",
    )
    .unwrap();

    write_theme_selection(&conf, "babydra-blue").expect("write");

    let raw = fs::read_to_string(&conf).unwrap();
    assert!(!raw.contains("old-theme"), "old id replaced: {raw}");
    assert!(raw.contains("id = \"babydra-blue\""), "new id set: {raw}");

    let _ = fs::remove_dir_all(conf.parent().unwrap());
}
