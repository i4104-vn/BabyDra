//! Integration tests: installer variant options.
//!
//! Verifies `variants/*/variant.toml` discovery and default pre-selection
//! through the installer's public `initial_variant_options` API.

use babydra_installer::system::initial_variant_options;
use std::fs;
use std::path::PathBuf;

/// Writes a temp variants tree and checks parsing + default selection.
#[test]
fn variant_options_parses_tree_and_preselects_default() {
    let tmp = std::env::temp_dir().join(format!("babydra_inst_var_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(tmp.join("variants/default")).unwrap();
    fs::create_dir_all(tmp.join("variants/community-dark")).unwrap();

    fs::write(
        tmp.join("variants/default/variant.toml"),
        "name = \"default\"\ntheme = \"babydra-default\"\napps = [\"panel\", \"explore\"]\n",
    )
    .unwrap();
    fs::write(
        tmp.join("variants/community-dark/variant.toml"),
        "name = \"community-dark\"\ntheme = \"babydra-blue\"\napps = [\"panel\"]\n",
    )
    .unwrap();
    // Non-variant folder without variant.toml must be ignored.
    fs::create_dir_all(tmp.join("variants/noop")).unwrap();

    let items = initial_variant_options(&tmp);
    assert_eq!(
        items.len(),
        2,
        "variants without variant.toml must be skipped"
    );

    let default = items.iter().find(|v| v.name == "default").unwrap();
    assert!(default.selected, "default variant must be pre-selected");
    assert_eq!(default.theme, "babydra-default");
    assert_eq!(default.apps, vec!["panel", "explore"]);

    let community = items.iter().find(|v| v.name == "community-dark").unwrap();
    assert!(!community.selected);
    assert_eq!(community.theme, "babydra-blue");

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn variant_options_empty_when_dir_missing() {
    let items = initial_variant_options(&PathBuf::from("/nonexistent-babydra-path"));
    assert!(items.is_empty());
}
