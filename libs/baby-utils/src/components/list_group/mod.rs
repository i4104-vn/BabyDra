use gtk4::prelude::*;

/// Creates a standard row with an icon, title, subtitle, and an optional right-aligned widget.
pub fn create_list_row(
    icon_name: &str,
    title: &str,
    subtitle: &str,
    right_widget: Option<&impl IsA<gtk4::Widget>>,
) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    row.set_margin_top(8);
    row.set_margin_bottom(8);
    row.set_margin_start(8);
    row.set_margin_end(8);

    if !icon_name.is_empty() {
        let icon = babydra_common::icon::get_icon(icon_name, 20);
        icon.set_valign(gtk4::Align::Center);
        row.append(&icon);
    }

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
