//! Reusable row builders for Notepad settings cards.

use babydra_ui_kit::components::CustomSwitch;
use babydra_ui_kit::ui::icon::get_icon;
use gtk4::prelude::*;
use gtk4::{
    Align, Box, DropDown, Label, ListBox, ListBoxRow, Orientation, SpinButton, StringList, Widget,
};

const ROW_SPACING: i32 = 12;
const ROW_MARGIN_VERTICAL: i32 = 12;
const ROW_MARGIN_HORIZONTAL: i32 = 16;

/// Adds a row with a toggle switch to the settings card.
pub fn add_switch_row(
    listbox: &ListBox,
    icon_name: &str,
    label_title: &str,
    label_desc: &str,
    active: bool,
    on_toggle: impl Fn(bool) + 'static,
) {
    let switch = CustomSwitch::new(active);
    add_control_row(
        listbox,
        icon_name,
        label_title,
        label_desc,
        &switch.container,
    );
    switch.connect_state_set(on_toggle);
}

/// Adds a row with a numeric SpinButton to the settings card.
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
    let spin = SpinButton::with_range(min, max, step);
    spin.set_value(value as f64);
    spin.set_valign(Align::Center);
    spin.add_css_class("settings-spin");

    add_control_row(listbox, icon_name, label_title, label_desc, &spin);
    spin.connect_value_changed(move |button| on_change(button.value() as u32));
}

/// Adds a row with a DropDown to the settings card.
pub fn add_dropdown_row(
    listbox: &ListBox,
    icon_name: &str,
    label_title: &str,
    label_desc: &str,
    items: &[&str],
    selected_idx: u32,
    on_select: impl Fn(u32) + 'static,
) {
    let string_list = StringList::new(items);
    let dropdown = DropDown::new(Some(string_list), gtk4::Expression::NONE);
    dropdown.set_selected(selected_idx);
    dropdown.set_valign(Align::Center);
    dropdown.add_css_class("settings-dropdown");

    add_control_row(listbox, icon_name, label_title, label_desc, &dropdown);
    dropdown.connect_selected_notify(move |dropdown| on_select(dropdown.selected()));
}

fn add_control_row<W: IsA<Widget>>(
    listbox: &ListBox,
    icon_name: &str,
    label_title: &str,
    label_desc: &str,
    control: &W,
) {
    let row = ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let content = Box::new(Orientation::Horizontal, ROW_SPACING);
    content.set_margin_top(ROW_MARGIN_VERTICAL);
    content.set_margin_bottom(ROW_MARGIN_VERTICAL);
    content.set_margin_start(ROW_MARGIN_HORIZONTAL);
    content.set_margin_end(ROW_MARGIN_HORIZONTAL);

    let icon = get_icon(icon_name, 18);
    icon.set_valign(Align::Center);
    icon.add_css_class("settings-row-icon");
    content.append(&icon);

    let labels = Box::new(Orientation::Vertical, 2);
    labels.set_hexpand(true);
    labels.set_valign(Align::Center);
    labels.append(&setting_label("settings-row-title", label_title));
    labels.append(&setting_label("settings-row-desc", label_desc));
    content.append(&labels);
    content.append(control);

    row.set_child(Some(&content));
    listbox.append(&row);
}

fn setting_label(css_class: &str, text: &str) -> Label {
    let label = Label::builder().label(text).halign(Align::Start).build();
    label.add_css_class(css_class);
    label
}
