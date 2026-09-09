//! Integration tests: Kitty theme synchronization.

use babydra_core::services::system::theme::{is_kitty_available, sync_kitty_theme};
use std::fs;
use std::path::Path;

#[test]
fn test_is_kitty_available() {
    let _ = is_kitty_available();
}

#[test]
fn test_sync_kitty_theme_switch() {
    // Test dark mode switch
    assert!(sync_kitty_theme(true).is_ok());

    let home = std::env::var("HOME").unwrap_or_default();
    let theme_file = Path::new(&home).join(".config/kitty/theme.conf");

    if is_kitty_available() && theme_file.exists() {
        let dark_content = fs::read_to_string(&theme_file).unwrap();
        assert!(dark_content.contains("BabyDra Kitty Theme Override (Dark)"));
        assert!(dark_content.contains("#14141c"));

        // Test light mode switch
        assert!(sync_kitty_theme(false).is_ok());
        let light_content = fs::read_to_string(&theme_file).unwrap();
        assert!(light_content.contains("BabyDra Kitty Theme Override (Light)"));
        assert!(light_content.contains("#f2f2f8"));

        // Restore dark theme
        let _ = sync_kitty_theme(true);
    }
}
