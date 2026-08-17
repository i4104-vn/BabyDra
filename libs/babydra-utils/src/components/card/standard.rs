use gtk4::prelude::*;

/// Creates a standard box container styled as a settings card.
pub fn create_card(orientation: gtk4::Orientation, spacing: i32) -> gtk4::Box {
    let card = gtk4::Box::new(orientation, spacing);
    card.add_css_class("settings-card");
    card
}

/// Creates a box container styled as a card with a custom CSS class.
pub fn create_card_with_class(
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

/// Creates a general purpose settings row inside a card list.
///
/// This duplicates `list_group::create_list_row` (the single canonical row
/// builder) — deprecated, migrate callers to `create_list_row`.
#[deprecated(note = "use list_group::create_list_row instead")]
pub fn create_item_row(
    title: &str,
    subtitle: &str,
    suffix_widget: Option<&impl IsA<gtk4::Widget>>,
) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    row.add_css_class("settings-item-row");
    row.set_valign(gtk4::Align::Center);

    let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let title_lbl = gtk4::Label::new(Some(title));
    title_lbl.add_css_class("settings-label");
    title_lbl.set_halign(gtk4::Align::Start);
    label_box.append(&title_lbl);

    if !subtitle.is_empty() {
        let desc_lbl = gtk4::Label::new(Some(subtitle));
        desc_lbl.add_css_class("settings-desc");
        desc_lbl.set_halign(gtk4::Align::Start);
        label_box.append(&desc_lbl);
    }

    row.append(&label_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    row.append(&spacer);

    if let Some(widget) = suffix_widget {
        row.append(widget);
    }

    row
}
