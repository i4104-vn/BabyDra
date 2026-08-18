use gtk4::prelude::*;

/// Creates a standard box container styled as a settings card.
pub fn create_card(orientation: gtk4::Orientation, spacing: i32) -> gtk4::Box {
    let card = gtk4::Box::new(orientation, spacing);
    card.add_css_class("settings-card");
    card
}

/// Creates a box container styled as a card with a custom CSS class.
pub fn create_css_card(
    orientation: gtk4::Orientation,
    spacing: i32,
    css_class: &str,
) -> gtk4::Box {
    let card = gtk4::Box::new(orientation, spacing);
    if !css_class.is_empty() {
        card.add_css_class(css_class);
    }
    card
}

/// Creates a title label.
pub fn create_title(text: &str) -> gtk4::Label {
    let label = gtk4::Label::new(Some(text));
    label.add_css_class("settings-title");
    label.set_halign(gtk4::Align::Start);
    label
}

/// Creates a subtitle label.
pub fn create_subtitle(text: &str) -> gtk4::Label {
    let label = gtk4::Label::new(Some(text));
    label.add_css_class("settings-subtitle");
    label.set_halign(gtk4::Align::Start);
    label
}
