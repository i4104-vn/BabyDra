//! Integration tests: variant definitions (`variants/*/variant.toml`).
//!
//! Verifies variant loading, default name fallback, error handling and
//! keybind lookup through the public variant API.

use babydra_core::config::variant::{get_keybind, list_variants, load_variant};

fn write_variant(name: &str, content: &str) {
    let dir = std::env::temp_dir()
        .join("babydra_variant_tests")
        .join(name);
    std::fs::create_dir_all(&dir).expect("create variant dir");
    std::fs::write(dir.join("variant.toml"), content).expect("write variant.toml");
}

fn set_test_root() {
    std::env::set_var(
        "BABYDRA_VARIANTS_DIR",
        std::env::temp_dir().join("babydra_variant_tests"),
    );
}

#[test]
fn load_variant_parses_full_definition() {
    write_variant(
        "test-variant",
        r#"name = "test-variant"
theme = "babydra-default"
apps = ["panel", "explore"]

[keybinds]
"A-Tab" = "babydra-switcher"

[config_overrides]
"labwc.rc.margin.gap" = 12
"#,
    );
    set_test_root();

    let v = load_variant("test-variant").expect("load variant");
    assert_eq!(v.name, "test-variant");
    assert_eq!(v.theme, "babydra-default");
    assert_eq!(v.apps, vec!["panel", "explore"]);
    assert_eq!(v.keybinds["A-Tab"], "babydra-switcher");
    assert_eq!(
        v.config_overrides["labwc.rc.margin.gap"],
        toml::Value::Integer(12)
    );
}

#[test]
fn load_variant_fills_missing_name_from_folder() {
    write_variant("anon", "theme = \"babydra-default\"\n");
    set_test_root();
    let v = load_variant("anon").expect("load variant");
    assert_eq!(v.name, "anon");
    assert!(v.apps.is_empty(), "apps default to empty");
    assert!(v.keybinds.is_empty());
}

#[test]
fn load_variant_errors_on_missing_folder() {
    set_test_root();
    assert!(load_variant("does-not-exist").is_err());
}

#[test]
fn list_variants_returns_sorted_names() {
    write_variant("zeta", "name = \"zeta\"\ntheme = \"t\"\n");
    write_variant("alpha", "name = \"alpha\"\ntheme = \"t\"\n");
    set_test_root();
    let names = list_variants();
    assert!(names.contains(&"zeta".to_string()));
    assert!(names.contains(&"alpha".to_string()));
    assert_eq!(
        names,
        {
            let mut n = names.clone();
            n.sort();
            n
        },
        "sorted"
    );
}

#[test]
fn get_keybind_falls_back_to_default() {
    write_variant(
        "kb",
        "name = \"kb\"\ntheme = \"t\"\n[keybinds]\n\"A-Tab\" = \"switcher\"\n",
    );
    set_test_root();
    let v = load_variant("kb").unwrap();
    assert_eq!(get_keybind(&v, "A-Tab", "fallback"), "switcher");
    assert_eq!(get_keybind(&v, "unknown", "fallback"), "fallback");
}
