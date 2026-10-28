//! Manual date, time, and timezone controls.

use babydra_core::i18n::trans;
use babydra_ui_kit::components::create_list_row;
use chrono::Local;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, DropDown, Entry, Orientation, StringList};

const TIMEZONES: [&str; 9] = [
    "UTC",
    "Asia/Ho_Chi_Minh",
    "Asia/Tokyo",
    "Asia/Seoul",
    "Asia/Singapore",
    "Europe/London",
    "Europe/Berlin",
    "America/New_York",
    "America/Los_Angeles",
];

pub struct ManualDateTimeControls {
    pub container: GtkBox,
    pub time_box: GtkBox,
    pub timezone_row: GtkBox,
    pub date_entry: Entry,
    pub time_entry: Entry,
    pub timezone_dropdown: DropDown,
    pub apply_button: Button,
}

pub fn build() -> ManualDateTimeControls {
    let container = GtkBox::new(Orientation::Vertical, 4);
    container.set_margin_top(4);
    container.set_margin_bottom(4);

    let date_entry = Entry::new();
    date_entry.add_css_class("settings-entry");
    date_entry.set_placeholder_text(Some(&trans("settings.datetime_date_placeholder")));
    date_entry.set_width_chars(12);
    date_entry.set_halign(gtk4::Align::End);
    date_entry.set_valign(gtk4::Align::Center);
    date_entry.set_text(&Local::now().format("%Y-%m-%d").to_string());
    let date_row = create_list_row(
        "calendar",
        &trans("settings.datetime_manual_date"),
        &trans("settings.datetime_date_placeholder"),
        Some(&date_entry),
    );

    let time_entry = Entry::new();
    time_entry.add_css_class("settings-entry");
    time_entry.set_placeholder_text(Some(&trans("settings.datetime_time_placeholder")));
    time_entry.set_width_chars(10);
    time_entry.set_halign(gtk4::Align::End);
    time_entry.set_valign(gtk4::Align::Center);
    time_entry.set_text(&Local::now().format("%H:%M:%S").to_string());
    let time_row = create_list_row(
        "clock",
        &trans("settings.datetime_manual_time"),
        &trans("settings.datetime_time_placeholder"),
        Some(&time_entry),
    );

    let apply_button = Button::with_label(&trans("settings.datetime_apply"));
    apply_button.add_css_class("connect-pill-btn");
    apply_button.add_css_class("suggested-action");
    apply_button.set_cursor_from_name(Some("pointer"));
    apply_button.set_halign(gtk4::Align::End);
    apply_button.set_margin_top(6);
    apply_button.set_margin_end(8);
    apply_button.set_margin_bottom(6);

    let time_box = GtkBox::new(Orientation::Vertical, 4);
    time_box.append(&date_row);
    time_box.append(&time_row);
    time_box.append(&apply_button);
    time_box.set_visible(false);
    container.append(&time_box);

    let timezone_names = StringList::new(&TIMEZONES);
    let timezone_dropdown = DropDown::new(
        Some(timezone_names.clone()),
        Option::<gtk4::Expression>::None,
    );
    timezone_dropdown.add_css_class("settings-dropdown");
    timezone_dropdown.set_hexpand(false);
    timezone_dropdown.set_halign(gtk4::Align::End);
    timezone_dropdown.set_valign(gtk4::Align::Center);
    timezone_dropdown.set_cursor_from_name(Some("pointer"));
    timezone_dropdown.set_selected(current_timezone_index());

    let timezone_row = create_list_row(
        "map-pin",
        &trans("settings.datetime_manual_timezone"),
        "",
        Some(&timezone_dropdown),
    );
    timezone_row.set_visible(false);
    container.append(&timezone_row);

    ManualDateTimeControls {
        container,
        time_box,
        timezone_row,
        date_entry,
        time_entry,
        timezone_dropdown,
        apply_button,
    }
}

fn current_timezone_index() -> u32 {
    let timezone = std::fs::read_to_string("/etc/timezone")
        .ok()
        .map(|value| value.trim().to_owned());

    timezone
        .and_then(|value| TIMEZONES.iter().position(|item| *item == value))
        .unwrap_or(0) as u32
}

pub fn timezone_at(index: u32) -> Option<&'static str> {
    TIMEZONES.get(index as usize).copied()
}
