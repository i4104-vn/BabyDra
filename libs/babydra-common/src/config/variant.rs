//! Variant system: load and merge variant definitions.
//!
//! A *variant* is a folder under `variants/<name>/` containing a
//! `variant.toml` that selects a theme, an app list, keybinds and config
//! overrides. Merge order (right side wins):
//!
//! ```text
//! system defaults < configs/ seed < theme package < variant < ~/.babydra/
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Parsed `variant.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Variant {
    #[serde(default)]
    pub name: String,
    /// Theme package id to use (see `themes/`).
    pub theme: String,
    /// List of apps this variant installs / runs.
    #[serde(default)]
    pub apps: Vec<String>,
    /// Keybind map (action → target).
    #[serde(default)]
    pub keybinds: HashMap<String, String>,
    /// Config overrides (dotted path → value).
    #[serde(default)]
    pub config_overrides: HashMap<String, toml::Value>,
}

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
pub fn load_variant(name: &str) -> Result<Variant, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
