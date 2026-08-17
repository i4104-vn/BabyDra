//! Integration tests: babydra-theme engine.
//!
//! Verifies theme package loading and base-theme inheritance (token merge
//! + CSS layer concatenation) through the public API.

use babydra_theme::{load_package, resolve_theme};
use std::path::PathBuf;

fn temp_root() -> PathBuf {
    std::env::temp_dir().join("babydra_theme_tests")
}

/// Writes a theme package in the canonical layout:
/// `tokens.json` + `fonts.json` at the root, CSS layers under `css/`.
fn write_package(
    id: &str,
    tokens_json: &str,
    dark_css: &str,
    light_css: &str,
    css: &str,
    fonts: &str,
) {
    let dir = temp_root().join(id);
    let css_dir = dir.join("css");
    std::fs::create_dir_all(&css_dir).expect("create theme dir");
    std::fs::write(dir.join("tokens.json"), tokens_json).expect("write tokens");
    std::fs::write(css_dir.join("dark.css"), dark_css).expect("write dark css");
    std::fs::write(css_dir.join("light.css"), light_css).expect("write light css");
    std::fs::write(css_dir.join("theme.css"), css).expect("write theme css");
    std::fs::write(dir.join("fonts.json"), fonts).expect("write fonts");
}

fn point_themes_dir_at_temp() {
    std::env::set_var("BABYDRA_THEMES_DIR", temp_root());
}

#[test]
fn load_package_reads_all_five_files_from_css_subfolder() {
    write_package(
        "test-default",
        r##"{
            "name": "test-default",
            "base": null,
            "dark": { "accent": "#111111", "radius": { "md": 16 }, "font": "Test Font" },
            "light": { "accent": "#eeeeee", "radius": { "md": 16 }, "font": "Test Font" }
        }"##,
        ".dark-rule { color: #111111; }",
        ".light-rule { color: #eeeeee; }",
        ".extra-rule { color: red; }",
        r#"{"Test Font": ["Arial", "sans-serif"]}"#,
    );
    point_themes_dir_at_temp();

    let pkg = load_package("test-default").expect("load package");
    assert_eq!(pkg.id, "test-default");
    assert_eq!(pkg.dark.accent, "#111111");
    assert!(pkg.dark_css.contains(".dark-rule"));
    assert!(pkg.light_css.contains(".light-rule"));
    assert!(pkg.css.contains(".extra-rule"));
    assert_eq!(pkg.fonts["Test Font"], vec!["Arial", "sans-serif"]);
}

#[test]
fn load_package_missing_css_files_defaults_empty() {
    let dir = temp_root().join("test-minimal");
    std::fs::create_dir_all(&dir).expect("create dir");
    std::fs::write(
        dir.join("tokens.json"),
        r##"{"name": "test-minimal", "dark": {"accent": "#111111"}, "light": {"accent": "#eeeeee"}}"##,
    )
    .expect("write tokens");
    // No css/ folder, no fonts.json on purpose — all layers default empty.
    point_themes_dir_at_temp();

    let pkg = load_package("test-minimal").expect("load package");
    assert!(pkg.dark_css.is_empty());
    assert!(pkg.light_css.is_empty());
    assert!(pkg.css.is_empty());
    assert!(pkg.fonts.is_empty());
}

#[test]
fn load_package_falls_back_to_legacy_flat_css_layout() {
    // Packages deployed before the `css/` subfolder put layers at the root.
    let dir = temp_root().join("test-legacy");
    std::fs::create_dir_all(&dir).expect("create dir");
    std::fs::write(
        dir.join("tokens.json"),
        r##"{"name": "test-legacy", "dark": {"accent": "#111111"}, "light": {"accent": "#eeeeee"}}"##,
    )
    .expect("write tokens");
    std::fs::write(dir.join("dark.css"), ".legacy-dark {}").expect("write dark css");
    std::fs::write(dir.join("light.css"), ".legacy-light {}").expect("write light css");
    std::fs::write(dir.join("theme.css"), ".legacy-override {}").expect("write theme css");
    point_themes_dir_at_temp();

    let pkg = load_package("test-legacy").expect("load legacy package");
    assert_eq!(pkg.dark_css, ".legacy-dark {}");
    assert_eq!(pkg.light_css, ".legacy-light {}");
    assert_eq!(pkg.css, ".legacy-override {}");
}

#[test]
fn css_subfolder_wins_over_legacy_flat_layout() {
    let dir = temp_root().join("test-both");
    let css_dir = dir.join("css");
    std::fs::create_dir_all(&css_dir).expect("create dir");
    std::fs::write(
        dir.join("tokens.json"),
        r##"{"name": "test-both", "dark": {"accent": "#111111"}, "light": {"accent": "#eeeeee"}}"##,
    )
    .expect("write tokens");
    // Both layouts present — the css/ subfolder must win.
    std::fs::write(css_dir.join("dark.css"), ".nested-dark {}").expect("write nested");
    std::fs::write(dir.join("dark.css"), ".flat-dark {}").expect("write flat");
    point_themes_dir_at_temp();

    let pkg = load_package("test-both").expect("load package");
    assert_eq!(pkg.dark_css, ".nested-dark {}", "css/ subfolder takes precedence");
}

#[test]
fn resolve_theme_merges_base_tokens_and_css() {
    write_package(
        "test-base",
        r##"{
            "name": "test-base",
            "dark": { "accent": "#000000", "radius": { "md": 8 }, "font": "Base Font" },
            "light": { "accent": "#ffffff", "radius": { "md": 8 }, "font": "Base Font" }
        }"##,
        ".base-dark { color: #000000; }",
        ".base-light { color: #ffffff; }",
        ".base-css {}",
        "{}",
    );
    write_package(
        "test-child",
        r##"{
            "name": "test-child",
            "base": "test-base",
            "dark": { "accent": "#123456" },
            "light": { "accent": "#654321" }
        }"##,
        ".child-dark { color: #123456; }",
        ".child-light { color: #654321; }",
        ".child-css {}",
        "{}",
    );
    point_themes_dir_at_temp();

    let resolved = resolve_theme("test-child").expect("resolve");
    // Child accent wins.
    assert_eq!(resolved.dark.accent, "#123456");
    assert_eq!(resolved.light.accent, "#654321");
    // Radius falls back to base.
    assert_eq!(resolved.dark.radius.md, 8);
    // Dark layer: base first, child second.
    assert!(resolved.dark_css.contains(".base-dark"));
    assert!(resolved.dark_css.contains(".child-dark"));
    assert!(
        resolved.dark_css.find(".child-dark").unwrap()
            > resolved.dark_css.find(".base-dark").unwrap()
    );
    // Same for the light layer.
    assert!(resolved.light_css.contains(".base-light"));
    assert!(resolved.light_css.contains(".child-light"));
    // Both extra CSS layers concatenated, child last.
    assert!(resolved.css_layer.contains(".base-css"));
    assert!(resolved.css_layer.contains(".child-css"));
    assert!(
        resolved.css_layer.find(".child-css").unwrap()
            > resolved.css_layer.find(".base-css").unwrap()
    );
}

#[test]
fn resolve_theme_keeps_base_fields_not_overridden_by_child() {
    // Child only overrides accent — every other token field must stay from base.
    write_package(
        "fields-base",
        r##"{
            "name": "fields-base",
            "dark": { "surface": "#111111", "accent": "#000000", "radius": { "md": 8 } },
            "light": { "surface": "#eeeeee", "accent": "#ffffff", "radius": { "md": 8 } }
        }"##,
        "",
        "",
        "",
        "{}",
    );
    write_package(
        "fields-child",
        r##"{"name": "fields-child", "base": "fields-base", "dark": {"accent": "#123456"}, "light": {"accent": "#654321"}}"##,
        "",
        "",
        "",
        "{}",
    );
    point_themes_dir_at_temp();

    let resolved = resolve_theme("fields-child").expect("resolve");
    assert_eq!(resolved.dark.accent, "#123456", "child accent wins");
    assert_eq!(
        resolved.dark.surface, "#111111",
        "untouched field stays from base"
    );
    assert_eq!(resolved.dark.radius.md, 8, "radius inherited from base");
    assert_eq!(
        resolved.light.surface, "#eeeeee",
        "light untouched field stays"
    );
}

#[test]
fn resolve_theme_child_css_layers_are_optional() {
    // Child inherits base CSS when it ships no CSS of its own.
    write_package(
        "css-base",
        r##"{"name": "css-base", "dark": {"accent": "#111111"}, "light": {"accent": "#eeeeee"}}"##,
        ".base-dark {}",
        ".base-light {}",
        "",
        "{}",
    );
    write_package(
        "css-child",
        r##"{"name": "css-child", "base": "css-base", "dark": {"accent": "#222222"}, "light": {"accent": "#dddddd"}}"##,
        "",
        "",
        "",
        "{}",
    );
    point_themes_dir_at_temp();

    let resolved = resolve_theme("css-child").expect("resolve");
    assert_eq!(resolved.dark_css, ".base-dark {}", "inherits base dark css");
    assert_eq!(
        resolved.light_css, ".base-light {}",
        "inherits base light css"
    );
    assert_eq!(resolved.dark.accent, "#222222");
}

#[test]
fn resolve_theme_errors_on_missing_package() {
    point_themes_dir_at_temp();
    assert!(resolve_theme("does-not-exist").is_err());
}

#[test]
fn resolve_theme_detects_inheritance_cycle() {
    write_package(
        "cycle-a",
        r##"{"name": "cycle-a", "base": "cycle-b", "dark": {"accent": "#111111"}, "light": {"accent": "#eeeeee"}}"##,
        "",
        "",
        "",
        "{}",
    );
    write_package(
        "cycle-b",
        r##"{"name": "cycle-b", "base": "cycle-a", "dark": {"accent": "#222222"}, "light": {"accent": "#dddddd"}}"##,
        "",
        "",
        "",
        "{}",
    );
    point_themes_dir_at_temp();

    let err = resolve_theme("cycle-a").expect_err("cycle must fail");
    assert!(err.contains("cycle"), "error mentions cycle: {err}");
}
