/// Windows 11 Mica dark theme CSS for babydra-explore (embedded at compile time).
pub const EXPLORE_CSS: &str = include_str!("explore.css");

/// Load and apply the explore theme to the default GDK display.
pub fn apply_explore_theme() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(EXPLORE_CSS);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
