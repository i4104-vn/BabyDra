/// Load CSS stylesheet embedded directly from build.
pub fn load_css() {
    tracing::info!(target: "babydra-greeter", "Loading CSS stylesheet");
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("assets/greeter.css"));

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );
    tracing::info!(target: "babydra-greeter", "CSS theme loaded into default GDK Display provider successfully");
}