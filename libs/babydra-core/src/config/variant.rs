//! Variant system: load and merge variant definitions.
//!
//! A *variant* is a folder under `variants/<name>/` containing a
//! `variant.toml` that selects a theme, an app list, keybinds and config
//! overrides. Merge order (right side wins):
//!
//! ```text
//! system defaults < configs/ seed < theme package < variant < ~/.babydra/
//! ```
//!
//! The `Variant` data model lives in `models::config` and is re-exported here
//! as a legacy bridge so existing `config::variant::Variant` imports keep working.

pub use crate::models::config::Variant;
use crate::error::CoreResult;
use std::path::{Path, PathBuf};

/// Root of the variants tree (env override for tests / flexible deployments).
pub fn variants_root() -> PathBuf {
    if let Ok(dir) = std::env::var("BABYDRA_VARIANTS_DIR") {
        return PathBuf::from(dir);
    }
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)
        .map(|p| p.join("variants"))
        .unwrap_or_else(|| PathBuf::from("variants"))
}

/// Lists all available variant names from `variants/*/variant.toml`.
pub fn list_variants() -> Vec<String> {
    let root = variants_root();
    let mut names = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("variant.toml").is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    names.push(name.to_string());
                }
            }
        }
    }
    names.sort();
    names
}

/// Loads a variant by name from `variants/<name>/variant.toml`.
pub fn load_variant(name: &str) -> CoreResult<Variant> {
    let path = variants_root().join(name).join("variant.toml");
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {}", path.display(), e))?;
    let mut variant: Variant = toml::from_str(&content)
        .map_err(|e| format!("invalid variant.toml in {}: {}", path.display(), e))?;

    // Guard: name field must match the folder (single source of truth).
    if variant.name.is_empty() {
        variant.name = name.to_string();
    }
    Ok(variant)
}

/// Resolves a keybind with a default fallback (like `ExploreSettings::get_keybind`).
pub fn get_keybind(variant: &Variant, action: &str, default: &str) -> String {
    variant
        .keybinds
        .get(action)
        .cloned()
        .unwrap_or_else(|| default.to_string())
}
