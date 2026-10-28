use gtk4::prelude::*;

/// Creates a standard row with an icon badge, title, subtitle, and an optional right-aligned widget.
pub fn create_list_row(
    icon_name: &str,
    title: &str,
    subtitle: &str,
    right_widget: Option<&impl IsA<gtk4::Widget>>,
) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
    row.add_css_class("settings-card-row");
    row.set_margin_top(4);
    row.set_margin_bottom(4);
    row.set_margin_start(8);
    row.set_margin_end(8);

    if !icon_name.is_empty() {
        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Start);

        let icon = crate::ui::icon::get_icon(icon_name, 18);
        icon.set_pixel_size(18);
        icon.set_valign(gtk4::Align::Center);
        icon.set_halign(gtk4::Align::Center);
        icon.set_vexpand(true);
        icon_badge.append(&icon);
        row.append(&icon_badge);
    }

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    text_box.set_valign(gtk4::Align::Center);
    text_box.set_hexpand(true);

    let title_lbl = gtk4::Label::new(Some(title));
    title_lbl.add_css_class("settings-row-title");
    title_lbl.add_css_class("settings-label");
    title_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&title_lbl);

    if !subtitle.is_empty() {
        let desc_lbl = gtk4::Label::new(Some(subtitle));
        desc_lbl.add_css_class("settings-row-desc");
        desc_lbl.add_css_class("settings-desc");
        desc_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&desc_lbl);
    }

    row.append(&text_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    row.append(&spacer);

    if let Some(widget) = right_widget {
        widget.set_valign(gtk4::Align::Center);
        widget.set_halign(gtk4::Align::End);
        row.append(widget);
    }

    row
}

/// Creates a standard row with an icon badge, title, subtitle, and an interactive CustomSwitch.
pub fn create_switch_row(
    icon_name: &str,
    title: &str,
    subtitle: &str,
    active: bool,
) -> (gtk4::Box, crate::components::switch::CustomSwitch) {
    let switch = crate::components::switch::CustomSwitch::new(active);
    switch.container.set_valign(gtk4::Align::Center);
    let row = create_list_row(icon_name, title, subtitle, Some(&switch.container));
    (row, switch)
}

/// Removes all children from a ListBox.
pub fn clear_list_box(list_box: &gtk4::ListBox) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }
}

/// Removes all children from a Box container.
pub fn clear_box(box_container: &gtk4::Box) {
    while let Some(child) = box_container.first_child() {
        box_container.remove(&child);
    }
}

/// Removes all children from a FlowBox container.
pub fn clear_flow_box(flow_box: &gtk4::FlowBox) {
    while let Some(child) = flow_box.first_child() {
        flow_box.remove(&child);
    }
}
