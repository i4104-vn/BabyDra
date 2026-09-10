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

/// Collapsible card component with an animated slide revealer.
#[derive(Clone)]
pub struct CollapsibleCard {
    pub container: gtk4::Box,
    pub content: gtk4::Box,
    pub revealer: gtk4::Revealer,
    pub title_label: gtk4::Label,
    pub subtitle_label: Option<gtk4::Label>,
    pub header_button: gtk4::Button,
}

/// Creates a collapsible settings card with a clickable header and smooth slide revealer.
pub fn create_collapsible_card(
    title: &str,
    subtitle: Option<&str>,
    icon_name: Option<&str>,
    initially_expanded: bool,
) -> CollapsibleCard {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.add_css_class("settings-card");
    container.add_css_class("collapsible-card");

    let header_button = gtk4::Button::new();
    header_button.add_css_class("card-collapse-header");
    header_button.set_cursor_from_name(Some("pointer"));

    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_box.set_valign(gtk4::Align::Center);

    if let Some(icon) = icon_name {
        let icon_w = crate::ui::icon::get_icon(icon, 20);
        icon_w.set_pixel_size(20);
        icon_w.set_valign(gtk4::Align::Center);
        header_box.append(&icon_w);
    }

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    text_box.set_hexpand(true);
    text_box.set_valign(gtk4::Align::Center);

    let title_label = gtk4::Label::new(Some(title));
    title_label.add_css_class("settings-row-title");
    title_label.set_halign(gtk4::Align::Start);
    text_box.append(&title_label);

    let subtitle_label = if let Some(sub) = subtitle {
        let sub_lbl = gtk4::Label::new(Some(sub));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&sub_lbl);
        Some(sub_lbl)
    } else {
        None
    };

    header_box.append(&text_box);

    let arrow_icon = crate::ui::icon::get_icon(
        if initially_expanded { "down" } else { "forward" },
        14,
    );
    arrow_icon.add_css_class("card-collapse-arrow");
    arrow_icon.set_pixel_size(14);
    arrow_icon.set_valign(gtk4::Align::Center);
    header_box.append(&arrow_icon);

    header_button.set_child(Some(&header_box));
    container.append(&header_button);

    let revealer = gtk4::Revealer::new();
    revealer.set_transition_type(gtk4::RevealerTransitionType::SlideDown);
    revealer.set_transition_duration(250);
    revealer.set_reveal_child(initially_expanded);

    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    content.add_css_class("card-collapse-content");
    revealer.set_child(Some(&content));
    container.append(&revealer);

    let rev_c = revealer.clone();
    let arrow_c = arrow_icon.clone();
    header_button.connect_clicked(move |_| {
        let next_state = !rev_c.reveals_child();
        rev_c.set_reveal_child(next_state);
        crate::ui::icon::set_image_from_icon(
            &arrow_c,
            if next_state { "down" } else { "forward" },
            14,
        );
    });

    CollapsibleCard {
        container,
        content,
        revealer,
        title_label,
        subtitle_label,
        header_button,
    }
}


