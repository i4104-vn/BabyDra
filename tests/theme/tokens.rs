//! Integration tests: theme token deserialization.
//!
//! Verifies `tokens.json` parsing with serde defaults. Token merge logic
//! itself is exercised indirectly through `resolve_theme` in `engine.rs`.

use babydra_theme::ThemeTokens;

#[test]
fn tokens_deserialize_with_defaults() {
    let raw = r##"{
        "name": "x",
        "dark": { "accent": "#3b82f6" }
    }"##;
    let tokens: ThemeTokens = serde_json::from_str(raw).unwrap();
    assert_eq!(tokens.dark.accent, "#3b82f6");
    assert!(
        tokens.dark.surface.is_empty(),
        "missing field defaults to empty"
    );
    assert!(tokens.base.is_none());
}
