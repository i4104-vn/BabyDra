use gtk4::prelude::*;

/// Creates a container Box with card layout (rounded corners, dark background).
pub fn create_card(orientation: gtk4::Orientation, spacing: i32) -> gtk4::Box {
    let card = gtk4::Box::new(orientation, spacing);
    card.add_css_class("settings-card");
    card
}

/// Creates a card with custom class.
pub fn create_card_with_class(orientation: gtk4::Orientation, spacing: i32, css_class: &str) -> gtk4::Box {
    let card = gtk4::Box::new(orientation, spacing);
    if !css_class.is_empty() {
        card.add_css_class(css_class);
    }
    card
}

/// Creates a header/title label.
pub fn create_title(text: &str) -> gtk4::Label {
    let lbl = gtk4::Label::new(Some(text));
    lbl.add_css_class("settings-title");
    lbl.set_halign(gtk4::Align::Start);
    lbl
}

/// Creates a sub-header/subtitle label.
pub fn create_subtitle(text: &str) -> gtk4::Label {
    let lbl = gtk4::Label::new(Some(text));
    lbl.add_css_class("settings-subtitle");
    lbl.set_halign(gtk4::Align::Start);
    lbl
}

/// Creates an item row with a title, subtitle, and an optional right-aligned widget.
pub fn create_item_row(title: &str, subtitle: &str, right_widget: Option<&impl IsA<gtk4::Widget>>) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    row.add_css_class("settings-row-item");

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let title_lbl = gtk4::Label::new(Some(title));
    title_lbl.add_css_class("settings-label");
    title_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&title_lbl);

    let desc_lbl = gtk4::Label::new(Some(subtitle));
    desc_lbl.add_css_class("settings-desc");
    desc_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&desc_lbl);

    row.append(&text_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    row.append(&spacer);

    if let Some(widget) = right_widget {
        row.append(widget);
    }

    row
}

/// Creates a switch card containing a title, subtitle, and an active switch.
pub fn create_switch_card(title: &str, subtitle: &str) -> (gtk4::Box, gtk4::Switch) {
    let card = create_card(gtk4::Orientation::Horizontal, 12);
    card.set_valign(gtk4::Align::Center);

    let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let status_title = gtk4::Label::new(Some(title));
    status_title.add_css_class("settings-label");
    status_title.set_halign(gtk4::Align::Start);
    
    let status_desc = gtk4::Label::new(Some(subtitle));
    status_desc.add_css_class("settings-desc");
    status_desc.set_halign(gtk4::Align::Start);
    
    label_box.append(&status_title);
    label_box.append(&status_desc);
    card.append(&label_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    card.append(&spacer);

    let sw = super::switch::create_switch(false, |_| {});
    card.append(&sw);

    (card, sw)
}

/// Creates a ScrolledWindow + ListBox combo.
pub fn create_scrollable_list(css_class: &str) -> (gtk4::ScrolledWindow, gtk4::ListBox) {
    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    if !css_class.is_empty() {
        list_box.add_css_class(css_class);
    }
    scroll.set_child(Some(&list_box));

    (scroll, list_box)
}
