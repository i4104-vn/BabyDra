//! Syntax detection based on file paths, extensions, names, and shebangs.

use std::path::Path;
use std::sync::OnceLock;
use syntect::parsing::{SyntaxReference, SyntaxSet};

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();

/// Returns the globally shared syntax definition set.
pub fn syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

/// Detects the syntax definition from an optional file path or first-line content.
pub fn detect_syntax(path: Option<&Path>, first_line: Option<&str>) -> &'static SyntaxReference {
    let ps = syntax_set();
    if let Some(p) = path {
        if let Ok(Some(syntax)) = ps.find_syntax_for_file(p) {
            return syntax;
        }
    }
    if let Some(line) = first_line {
        if let Some(syntax) = ps.find_syntax_by_first_line(line) {
            return syntax;
        }
    }
    ps.find_syntax_plain_text()
}

/// Finds syntax by file extension (e.g. "rs", "json", "py", "md").
pub fn detect_syntax_by_extension(ext: &str) -> Option<&'static SyntaxReference> {
    syntax_set().find_syntax_by_extension(ext)
}

/// Finds syntax by language name (e.g. "Rust", "JSON", "Python", "Markdown").
pub fn detect_syntax_by_name(name: &str) -> Option<&'static SyntaxReference> {
    syntax_set().find_syntax_by_name(name)
}

/// Returns a human-friendly language name for the given syntax.
pub fn language_name(syntax: &SyntaxReference) -> String {
    if syntax.name.is_empty() || syntax.name == "Plain Text" {
        "Plain Text".to_string()
    } else {
        syntax.name.clone()
    }
}

/// Returns the list of all supported syntax language names.
pub fn list_supported_languages() -> Vec<String> {
    let mut langs: Vec<String> = syntax_set()
        .syntaxes()
        .iter()
        .map(|s| s.name.clone())
        .filter(|n| !n.is_empty())
        .collect();
    langs.sort();
    langs.dedup();
    langs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_detect_syntax_by_path() {
        let rust_syntax = detect_syntax(Some(Path::new("main.rs")), None);
        assert_eq!(language_name(rust_syntax), "Rust");

        let json_syntax = detect_syntax(Some(Path::new("package.json")), None);
        assert_eq!(language_name(json_syntax), "JSON");

        let unknown_syntax = detect_syntax(Some(Path::new("unknown.xyz123")), None);
        assert_eq!(language_name(unknown_syntax), "Plain Text");
    }

    #[test]
    fn test_detect_syntax_by_first_line() {
        let python_syntax = detect_syntax(None, Some("#!/usr/bin/env python3\n"));
        assert_eq!(language_name(python_syntax), "Python");

        let sh_syntax = detect_syntax(None, Some("#!/bin/bash\n"));
        assert!(
            language_name(sh_syntax) == "Bourne Again Shell (bash)"
                || language_name(sh_syntax) == "Shell-Unix-Generic"
                || language_name(sh_syntax).contains("Shell")
        );
    }

    #[test]
    fn test_detect_syntax_by_extension() {
        assert!(detect_syntax_by_extension("rs").is_some());
        assert!(detect_syntax_by_extension("c").is_some());
        assert!(detect_syntax_by_extension("json").is_some());
        assert!(detect_syntax_by_extension("nonexistentextension123").is_none());
    }

    #[test]
    fn test_list_supported_languages() {
        let langs = list_supported_languages();
        assert!(!langs.is_empty());
        assert!(langs.iter().any(|l| l == "Rust"));
        assert!(langs.iter().any(|l| l == "JSON"));
    }
}
