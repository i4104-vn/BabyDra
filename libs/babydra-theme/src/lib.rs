//! Theme package engine for BabyDra.
//!
//! A *theme package* is a folder under `themes/<theme-id>/` containing:
//! - `tokens.json` — design tokens (colors, radius, spacing, font, motion)
//! - `theme.css`   — the theme color layer, loaded on top of the core CSS
//! - `fonts.json`  — font families used by the theme
//!
//! This crate is **pure logic** — no GTK, no CSS parsing at runtime. It
//! reads packages from disk and resolves them into typed `ThemeValue`
//! structs that UI layers can consume.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod tokens;

pub use tokens::{ThemeTokens, DarkLightTokens, RadiusTokens};

/// Fully resolved theme values for one theme id.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeValue {
    /// Resolved tokens with dark + light modes ready to consume.
    pub dark: DarkLightTokens,
    pub light: DarkLightTokens,
    /// The theme color layer CSS, verbatim.
    pub css_layer: String,
    /// Font declarations from fonts.json (family → list of fallbacks).
    pub fonts: HashMap<String, Vec<String>>,
    /// Absolute path to the theme package folder.
    pub package_path: PathBuf,
}

/// A theme package as found on disk, before mode resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemePackage {
    pub id: String,
    /// Theme this one inherits from (optional).
    pub base: Option<String>,
    pub dark: DarkLightTokens,
    pub light: DarkLightTokens,
    pub css: String,
    pub fonts: HashMap<String, Vec<String>>,
    pub path: PathBuf,
}

/// Root of the themes tree (resolved from repo layout or env override).
pub fn themes_root() -> PathBuf {
    if let Ok(dir) = std::env::var("BABYDRA_THEMES_DIR") {
        return PathBuf::from(dir);
    }
    // Default: workspace-relative `themes/` folder.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // During tests we live under libs/babydra-theme — themes/ is two levels up.
    manifest_dir
        .ancestors()
        .nth(2)
        .map(|p| p.join("themes"))
        .unwrap_or_else(|| PathBuf::from("themes"))
}

/// Loads a theme package folder (`themes/<id>/`) from disk.
pub fn load_package(id: &str) -> Result<ThemePackage, String> {
    let dir = themes_root().join(id);
    if !dir.is_dir() {
        return Err(format!("theme package not found: {}", dir.display()));
    }

    let tokens_path = dir.join("tokens.json");
    let tokens_raw = std::fs::read_to_string(&tokens_path)
        .map_err(|e| format!("cannot read {}: {}", tokens_path.display(), e))?;
    let tokens: ThemeTokens = serde_json::from_str(&tokens_raw)
        .map_err(|e| format!("invalid tokens.json in {}: {}", dir.display(), e))?;

    let css = std::fs::read_to_string(dir.join("theme.css")).unwrap_or_default();

    let fonts = std::fs::read_to_string(dir.join("fonts.json"))
        .ok()
        .and_then(|raw| serde_json::from_str::<HashMap<String, Vec<String>>>(&raw).ok())
        .unwrap_or_default();

    Ok(ThemePackage {
        id: id.to_string(),
        base: tokens.base.clone(),
        dark: tokens.dark,
        light: tokens.light,
        css,
        fonts,
        path: dir,
    })
}

/// Resolves a theme by id, applying inheritance from `base` themes.
///
/// Tokens are merged base-first (base values are defaults, the child theme
/// overrides them). Returns a fully-resolved `ThemeValue`.
pub fn resolve_theme(id: &str) -> Result<ThemeValue, String> {
    let mut visited: Vec<String> = Vec::new();
    let merged = resolve_recursive(id, &mut visited)?;
    Ok(ThemeValue {
        dark: merged.dark,
        light: merged.light,
        css_layer: merged.css,
        fonts: merged.fonts,
        package_path: merged.path,
    })
}

fn resolve_recursive(
    id: &str,
    visited: &mut Vec<String>,
) -> Result<ThemePackage, String> {
    if visited.iter().any(|v| v == id) {
        return Err(format!("theme inheritance cycle detected at '{}'", id));
    }
    visited.push(id.to_string());

    let package = load_package(id)?;
    let mut merged = package.clone();

    if let Some(base_id) = &package.base {
        let base = resolve_recursive(base_id, visited)?;
        // Child overrides base: merge base tokens first, then child on top.
        let mut dark = base.dark;
        dark.merge(&package.dark);
        let mut light = base.light;
        light.merge(&package.light);

        merged.dark = dark;
        merged.light = light;
        // CSS layers concatenate: base layer first, child layer on top.
        let mut css = base.css;
        css.push('\n');
        css.push_str(&package.css);
        merged.css = css;

        // Fonts: child entries win, base fills the rest.
        let mut fonts = base.fonts;
        for (family, fallbacks) in package.fonts {
            fonts.insert(family, fallbacks);
        }
        merged.fonts = fonts;
        merged.base = Some(base_id.clone());
    }

    Ok(merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_package(id: &str, tokens_json: &str, css: &str, fonts: &str) {
        let dir = std::env::temp_dir()
            .join("babydra_theme_tests")
            .join(id);
        std::fs::create_dir_all(&dir).expect("create theme dir");
        std::fs::write(dir.join("tokens.json"), tokens_json).expect("write tokens");
        std::fs::write(dir.join("theme.css"), css).expect("write css");
        std::fs::write(dir.join("fonts.json"), fonts).expect("write fonts");
    }

    #[test]
    fn load_package_reads_all_three_files() {
        write_package(
            "test-default",
            r##"{
                "name": "test-default",
                "base": null,
                "dark": { "accent": "#111111", "radius": { "md": 16 }, "font": "Test Font" },
                "light": { "accent": "#eeeeee", "radius": { "md": 16 }, "font": "Test Font" }
            }"##,
            ".test-theme { color: red; }",
            r#"{"Test Font": ["Arial", "sans-serif"]}"#,
        );

        // Point the engine at the temp tree.
        std::env::set_var("BABYDRA_THEMES_DIR", std::env::temp_dir().join("babydra_theme_tests"));

        let pkg = load_package("test-default").expect("load package");
        assert_eq!(pkg.id, "test-default");
        assert_eq!(pkg.dark.accent, "#111111");
        assert!(pkg.css.contains(".test-theme"));
        assert_eq!(pkg.fonts["Test Font"], vec!["Arial", "sans-serif"]);
    }

    #[test]
    fn resolve_theme_merges_base_tokens() {
        write_package(
            "test-base",
            r##"{
                "name": "test-base",
                "dark": { "accent": "#000000", "radius": { "md": 8 }, "font": "Base Font" },
                "light": { "accent": "#ffffff", "radius": { "md": 8 }, "font": "Base Font" }
            }"##,
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
            ".child-css {}",
            "{}",
        );
        std::env::set_var("BABYDRA_THEMES_DIR", std::env::temp_dir().join("babydra_theme_tests"));

        let resolved = resolve_theme("test-child").expect("resolve");
        // Child accent wins.
        assert_eq!(resolved.dark.accent, "#123456");
        assert_eq!(resolved.light.accent, "#654321");
        // Radius falls back to base.
        assert_eq!(resolved.dark.radius.md, 8);
        // Both CSS layers concatenated, child last.
        assert!(resolved.css_layer.contains(".base-css"));
        assert!(resolved.css_layer.contains(".child-css"));
        assert!(resolved.css_layer.find(".child-css").unwrap() > resolved.css_layer.find(".base-css").unwrap());
    }

    #[test]
    fn resolve_theme_errors_on_missing_package() {
        std::env::set_var("BABYDRA_THEMES_DIR", std::env::temp_dir().join("babydra_theme_tests"));
        assert!(resolve_theme("does-not-exist").is_err());
    }

    #[test]
    fn resolve_theme_detects_inheritance_cycle() {
        write_package(
            "cycle-a",
            r##"{"name": "cycle-a", "base": "cycle-b", "dark": {"accent": "#111111"}, "light": {"accent": "#eeeeee"}}"##,
            "",
            "{}",
        );
        write_package(
            "cycle-b",
            r##"{"name": "cycle-b", "base": "cycle-a", "dark": {"accent": "#222222"}, "light": {"accent": "#dddddd"}}"##,
            "",
            "{}",
        );
        std::env::set_var("BABYDRA_THEMES_DIR", std::env::temp_dir().join("babydra_theme_tests"));

        let err = resolve_theme("cycle-a").expect_err("cycle must fail");
        assert!(err.contains("cycle"), "error mentions cycle: {}", err);
    }
}
