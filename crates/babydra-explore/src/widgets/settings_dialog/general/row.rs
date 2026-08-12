use gtk4::prelude::*;
use gtk4::{Box, Orientation, ListBox, ListBoxRow, Label, Align};

/// Helper to render a settings row with a switch toggle.
pub fn add_switch_row(
    listbox: &ListBox,
    icon_name: &str,
    label_title: &str,
    label_desc: &str,
    active: bool,
    on_toggle: std::boxed::Box<dyn Fn(bool)>,
) {
    let row = ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = Box::new(Orientation::Horizontal, 12);
    hbox.set_margin_top(12);
    hbox.set_margin_bottom(12);
    hbox.set_margin_start(16);
    hbox.set_margin_end(16);

    // Left Icon
    let icon = babydra_utils::ui::icon::get_icon(icon_name, 16);
    icon.set_valign(Align::Center);
    icon.add_css_class("settings-row-icon");
    hbox.append(&icon);

    let vbox_lbl = Box::new(Orientation::Vertical, 2);
    vbox_lbl.set_hexpand(true);
    vbox_lbl.set_valign(Align::Center);

    let lbl_title = Label::builder()
        .label(label_title)
        .halign(Align::Start)
        .build();
    lbl_title.add_css_class("settings-row-title");

    let lbl_desc = Label::builder()
        .label(label_desc)
        .halign(Align::Start)
        .build();
    lbl_desc.add_css_class("settings-row-desc");

    vbox_lbl.append(&lbl_title);
    vbox_lbl.append(&lbl_desc);
    hbox.append(&vbox_lbl);

    let sw = babydra_utils::components::CustomSwitch::new(active);
    sw.connect_state_set(move |state| {
        on_toggle(state);
    });

    hbox.append(&sw.container);
    row.set_child(Some(&hbox));
    listbox.append(&row);
}
