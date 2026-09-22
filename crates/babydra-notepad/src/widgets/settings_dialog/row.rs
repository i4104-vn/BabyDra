//! Row builders for settings list cards.

use babydra_ui_kit::components::CustomSwitch;
use babydra_ui_kit::ui::icon::get_icon;
use gtk4::prelude::*;
use gtk4::{Align, Box, DropDown, Label, ListBox, ListBoxRow, Orientation, SpinButton, StringList};

/// Adds a row with a toggle switch to the given settings listbox.
pub fn add_switch_row(
    listbox: &ListBox,
    icon_name: &str,
    label_title: &str,
    label_desc: &str,
    active: bool,
    on_toggle: impl Fn(bool) + 'static,
) {
    let row = ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = Box::new(Orientation::Horizontal, 12);
    hbox.set_margin_top(10);
    hbox.set_margin_bottom(10);
    hbox.set_margin_start(16);
    hbox.set_margin_end(16);

    let icon = get_icon(icon_name, 18);
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

    let switch = CustomSwitch::new(active);
    switch.connect_state_set(move |state| {
        on_toggle(state);
    });

    hbox.append(&switch.container);
    row.set_child(Some(&hbox));
    listbox.append(&row);
}

/// Adds a row with a numeric SpinButton to the given settings listbox.
#[allow(clippy::too_many_arguments)]
pub fn add_spin_row(
    listbox: &ListBox,
    icon_name: &str,
    label_title: &str,
    label_desc: &str,
    value: u32,
    min: f64,
    max: f64,
    step: f64,
    on_change: impl Fn(u32) + 'static,
) {
    let row = ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = Box::new(Orientation::Horizontal, 12);
    hbox.set_margin_top(10);
    hbox.set_margin_bottom(10);
    hbox.set_margin_start(16);
    hbox.set_margin_end(16);

    let icon = get_icon(icon_name, 18);
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

    let spin = SpinButton::with_range(min, max, step);
    spin.set_value(value as f64);
    spin.set_valign(Align::Center);
    spin.add_css_class("settings-spin");
    spin.connect_value_changed(move |btn| {
        on_change(btn.value() as u32);
    });

    hbox.append(&spin);
    row.set_child(Some(&hbox));
    listbox.append(&row);
}

/// Adds a row with a DropDown selection to the given settings listbox.
pub fn add_dropdown_row(
    listbox: &ListBox,
    icon_name: &str,
    label_title: &str,
    label_desc: &str,
    items: &[&str],
    selected_idx: u32,
    on_select: impl Fn(u32) + 'static,
) {
    let row = ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = Box::new(Orientation::Horizontal, 12);
    hbox.set_margin_top(10);
    hbox.set_margin_bottom(10);
    hbox.set_margin_start(16);
    hbox.set_margin_end(16);

    let icon = get_icon(icon_name, 18);
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

    let str_list = StringList::new(items);
    let dropdown = DropDown::new(Some(str_list), gtk4::Expression::NONE);
    dropdown.set_selected(selected_idx);
    dropdown.set_valign(Align::Center);
    dropdown.add_css_class("settings-dropdown");
    dropdown.connect_selected_notify(move |dd| {
        on_select(dd.selected());
    });

    hbox.append(&dropdown);
    row.set_child(Some(&hbox));
    listbox.append(&row);
}
