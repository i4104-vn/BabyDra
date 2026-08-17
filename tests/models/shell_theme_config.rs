//! Integration tests: shell theme configuration model.
//!
//! Verifies JSON round-trips and backward-compatible deserialization of
//! `ShellConfig` / `ThemeConfig` (including the `theme.selection` package
//! pointer introduced after the legacy visual fields).

use babydra_core::models::ShellConfig;

#[test]
fn old_config_without_selection_still_deserializes() {
    let old = r##"{
        "theme": {
            "blur_radius": 24,
            "opacity": 0.8,
            "border_color": "#ff0000",
            "border_width": 2
        }
    }"##;
    let cfg: ShellConfig = serde_json::from_str(old).expect("old config must parse");
    assert_eq!(cfg.theme.blur_radius, 24);
    assert_eq!(cfg.theme.opacity, 0.8);
    assert_eq!(cfg.theme.border_color, "#ff0000");
    assert_eq!(cfg.theme.border_width, 2);
    assert!(cfg.theme.selection.id.is_empty());
    assert_eq!(cfg.theme.selection.dark, None);
}

#[test]
fn empty_config_uses_defaults() {
    let cfg: ShellConfig = serde_json::from_str("{}").expect("empty config must parse");
    assert_eq!(cfg.theme.blur_radius, 20);
    assert_eq!(cfg.theme.opacity, 0.75);
    assert_eq!(cfg.theme.border_width, 1);
}

#[test]
fn new_config_with_selection_roundtrips() {
    let json = r##"{
        "theme": {
            "blur_radius": 30,
            "selection": { "id": "babydra-blue", "dark": false }
        }
    }"##;
    let cfg: ShellConfig = serde_json::from_str(json).expect("new config must parse");
    assert_eq!(cfg.theme.selection.id, "babydra-blue");
    assert_eq!(cfg.theme.selection.dark, Some(false));
    assert_eq!(cfg.theme.blur_radius, 30);
    // untouched legacy fields fall back to defaults
    assert_eq!(cfg.theme.border_width, 1);
}
