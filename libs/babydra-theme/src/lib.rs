//! Theme package engine for BabyDra.
//!
//! A *theme package* is a folder under `themes/<theme-id>/` containing:
//! - `tokens.json` — design tokens (colors, radius, spacing, font, motion)
//! - `dark.css`    — dark-mode color layer (loaded on top of the core CSS)
//! - `light.css`   — light-mode color layer
//! - `theme.css`   — optional extra color layer (loaded last, e.g. overrides)
//! - `fonts.json`  — font families used by the theme
//!
//! This crate is **pure logic** — no GTK, no CSS parsing at runtime. It
//! reads packages from disk and resolves them into typed `ThemeValue`
//! structs that UI layers can consume.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod tokens;

pub use tokens::{DarkLightTokens, RadiusTokens, ThemeTokens};

/// Fully resolved theme values for one theme id.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeValue {
    /// Resolved tokens with dark + light modes ready to consume.
    pub dark: DarkLightTokens,
    pub light: DarkLightTokens,
    /// Dark-mode color layer CSS (theme-driven).
    pub dark_css: String,
    /// Light-mode color layer CSS (theme-driven).
    pub light_css: String,
    /// Extra theme color layer CSS, verbatim (loaded last).
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
    pub dark_css: String,
    pub light_css: String,
    pub css: String,
    pub fonts: HashMap<String, Vec<String>>,
    pub path: PathBuf,
}

/// Root of the themes tree.
///
/// Resolution order (first hit wins):
/// 1. `BABYDRA_THEMES_DIR` env override (tests / flexible deployments)
/// 2. `~/.babydra/themes` — user-installed theme packages
/// 3. `/usr/share/babydra/themes` — system-installed theme packages
/// 4. Workspace-relative `themes/` folder (dev / repo checkout)
pub fn themes_root() -> PathBuf {
    if let Ok(dir) = std::env::var("BABYDRA_THEMES_DIR") {
        return PathBuf::from(dir);
    }

    if let Ok(home) = std::env::var("HOME") {
        let user_dir = Path::new(&home).join(".babydra").join("themes");
        if user_dir.is_dir() {
            return user_dir;
        }
    }

    let system_dir = PathBuf::from("/usr/share/babydra/themes");
    if system_dir.is_dir() {
        return system_dir;
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

    let dark_css = read_optional(&dir.join("dark.css"));
    let light_css = read_optional(&dir.join("light.css"));
    let css = read_optional(&dir.join("theme.css"));

    let fonts = std::fs::read_to_string(dir.join("fonts.json"))
        .ok()
        .and_then(|raw| serde_json::from_str::<HashMap<String, Vec<String>>>(&raw).ok())
        .unwrap_or_default();

    Ok(ThemePackage {
        id: id.to_string(),
        base: tokens.base.clone(),
        dark: tokens.dark,
        light: tokens.light,
        dark_css,
        light_css,
        css,
        fonts,
        path: dir,
    })
}

fn read_optional(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Resolves a theme by id, applying inheritance from `base` themes.
///
/// Tokens are merged base-first (base values are defaults, the child theme
/// overrides them). CSS layers concatenate base-first, so child rules win.
/// Returns a fully-resolved `ThemeValue`.
pub fn resolve_theme(id: &str) -> Result<ThemeValue, String> {
    let mut visited: Vec<String> = Vec::new();
    let merged = resolve_recursive(id, &mut visited)?;
    Ok(ThemeValue {
        dark: merged.dark,
        light: merged.light,
        dark_css: merged.dark_css,
        light_css: merged.light_css,
        css_layer: merged.css,
        fonts: merged.fonts,
        package_path: merged.path,
    })
}

fn resolve_recursive(id: &str, visited: &mut Vec<String>) -> Result<ThemePackage, String> {
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
        merged.dark_css = concat_layers(&base.dark_css, &package.dark_css);
        merged.light_css = concat_layers(&base.light_css, &package.light_css);
        merged.css = concat_layers(&base.css, &package.css);

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

/// Concatenates two CSS layers, base first, separated by a newline.
fn concat_layers(base: &str, child: &str) -> String {
    if base.is_empty() {
        return child.to_string();
    }
    if child.is_empty() {
        return base.to_string();
    }
    format!("{base}\n{child}")
}
