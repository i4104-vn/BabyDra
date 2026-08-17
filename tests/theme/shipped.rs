//! Integration tests: shipped themes & variants stay in sync.
//!
//! Verifies every theme package in `themes/` resolves, and every variant in
//! `variants/` points to an existing theme. Uses its own test binary so it
//! can point `BABYDRA_THEMES_DIR` / `BABYDRA_VARIANTS_DIR` at the repo
//! without racing the other theme tests that use a temp dir.

use babydra_core::config::variant::{list_variants, load_variant};
use babydra_theme::resolve_theme;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

const SHIPPED_THEMES: [&str; 5] = [
    "babydra-default",
    "babydra-blue",
    "babydra-purple",
    "babydra-green",
    "babydra-rose",
];

/// Documented accent for each color theme (`tokens.json` dark + `css/theme.css`).
/// Key: theme id → accent hex that must appear in the resolved CSS layer.
const SHIPPED_ACCENTS: [(&str, &str); 4] = [
    ("babydra-blue", "38bdf8"),
    ("babydra-purple", "8b5cf6"),
    ("babydra-green", "10b981"),
    ("babydra-rose", "f43f5e"),
];

// `list_variants()` returns names sorted alphabetically.
const SHIPPED_VARIANTS: [&str; 5] = ["blue", "default", "green", "purple", "rose"];

#[test]
fn all_shipped_themes_resolve() {
    std::env::set_var("BABYDRA_THEMES_DIR", repo_root().join("themes"));
    for id in SHIPPED_THEMES {
        let theme = resolve_theme(id).unwrap_or_else(|e| panic!("theme {id} failed: {e}"));
        assert!(
            !theme.dark.accent.is_empty(),
            "theme {id} has no dark accent"
        );
    }

    // The color themes' accent must actually reach the resolved CSS layer
    // (tokens.json + css/theme.css stay in sync).
    for (id, accent) in SHIPPED_ACCENTS {
        let theme = resolve_theme(id).unwrap_or_else(|e| panic!("theme {id} failed: {e}"));
        assert!(
            theme.dark.accent == format!("#{accent}"),
            "theme {id} tokens.json accent mismatch: expected #{accent}, got {}",
            theme.dark.accent
        );
        assert!(
            theme.css_layer.contains(&format!("#{accent}")),
            "theme {id} css/theme.css missing accent #{accent}"
        );
    }
}

#[test]
fn shipped_variants_match_shipped_themes() {
    std::env::set_var("BABYDRA_THEMES_DIR", repo_root().join("themes"));
    std::env::set_var("BABYDRA_VARIANTS_DIR", repo_root().join("variants"));

    let variants = list_variants();
    assert_eq!(
        variants, SHIPPED_VARIANTS,
        "variants/ must stay in sync with the theme set"
    );

    for name in SHIPPED_VARIANTS {
        let v = load_variant(name).unwrap_or_else(|e| panic!("variant {name} failed: {e}"));
        assert!(
            resolve_theme(&v.theme).is_ok(),
            "variant {name} points to missing theme {}",
            v.theme
        );
    }
}
