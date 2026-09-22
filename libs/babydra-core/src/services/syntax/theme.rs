//! Syntax highlighting themes and color scheme selection.

use std::sync::OnceLock;
use syntect::highlighting::{Theme, ThemeSet};

static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

pub const DEFAULT_DARK_THEME: &str = "base16-ocean.dark";
pub const DEFAULT_LIGHT_THEME: &str = "InspiredGitHub";

/// Returns the globally shared theme set loaded from syntect defaults.
pub fn theme_set() -> &'static ThemeSet {
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

/// Retrieves a specific theme by name.
pub fn get_theme_by_name(name: &str) -> Option<&'static Theme> {
    theme_set().themes.get(name)
}

/// Selects an appropriate theme based on system dark/light mode preference.
pub fn get_theme(is_dark: bool) -> &'static Theme {
    let ts = theme_set();
    if is_dark {
        ts.themes
            .get(DEFAULT_DARK_THEME)
            .or_else(|| ts.themes.get("base16-eighties.dark"))
            .or_else(|| ts.themes.values().next())
            .expect("default dark theme must exist")
    } else {
        ts.themes
            .get(DEFAULT_LIGHT_THEME)
            .or_else(|| ts.themes.get("base16-ocean.light"))
            .or_else(|| ts.themes.values().next())
            .expect("default light theme must exist")
    }
}

/// Lists all available theme names.
pub fn list_available_themes() -> Vec<String> {
    let mut names: Vec<String> = theme_set().themes.keys().cloned().collect();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_set_loading() {
        let ts = theme_set();
        assert!(!ts.themes.is_empty());
    }

    #[test]
    fn test_get_theme_dark_and_light() {
        let dark = get_theme(true);
        assert!(!dark.scopes.is_empty());

        let light = get_theme(false);
        assert!(!light.scopes.is_empty());
    }

    #[test]
    fn test_get_theme_by_name() {
        assert!(get_theme_by_name(DEFAULT_DARK_THEME).is_some());
        assert!(get_theme_by_name(DEFAULT_LIGHT_THEME).is_some());
        assert!(get_theme_by_name("nonexistent_theme_name").is_none());
    }

    #[test]
    fn test_list_available_themes() {
        let themes = list_available_themes();
        assert!(themes.iter().any(|t| t == DEFAULT_DARK_THEME));
        assert!(themes.iter().any(|t| t == DEFAULT_LIGHT_THEME));
    }
}
