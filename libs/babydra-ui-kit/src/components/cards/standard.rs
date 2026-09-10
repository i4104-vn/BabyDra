use gtk4::prelude::*;

/// Creates a standard box container styled as a settings card.
pub fn create_card(orientation: gtk4::Orientation, spacing: i32) -> gtk4::Box {
    let card = gtk4::Box::new(orientation, spacing);
    card.add_css_class("settings-card");
    card
}

/// Creates a box container styled as a card with a custom CSS class.
pub fn create_css_card(orientation: gtk4::Orientation, spacing: i32, css_class: &str) -> gtk4::Box {
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

/// Creates a standard slider card header with a title and percentage value label.
pub fn create_slider_header(title: &str, initial_percent: f64) -> (gtk4::Box, gtk4::Label) {
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let title_label = gtk4::Label::new(Some(title));
    title_label.add_css_class("control-slider-title");
    title_label.set_xalign(0.0);
    title_label.set_hexpand(true);

    let value_label = gtk4::Label::new(Some(&format!("{:.0}%", initial_percent)));
    value_label.add_css_class("control-slider-value");
    value_label.set_xalign(1.0);

    header_box.append(&title_label);
    header_box.append(&value_label);
    (header_box, value_label)
}

