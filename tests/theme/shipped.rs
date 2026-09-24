//! Integration tests: shipped themes.
//!
//! Verifies every theme package in `assets/themes/` resolves, and that documented
//! accent colors match tokens and CSS layers.

use babydra_theme::resolve_theme;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn shipped_themes_dir() -> PathBuf {
    let root = repo_root();
    let assets = root.join("assets").join("themes");
    if assets.is_dir() {
        assets
    } else {
        root.join("themes")
    }
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

#[test]
fn all_shipped_themes_resolve() {
    std::env::set_var("BABYDRA_THEMES_DIR", shipped_themes_dir());
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
