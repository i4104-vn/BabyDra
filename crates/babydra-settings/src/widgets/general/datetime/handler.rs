//! Event wiring and system operations for Date & Time settings.

use super::controls;
use super::render::DateTimeWidgets;
use babydra_core::i18n::trans;
use chrono::{Datelike, Local, NaiveDateTime};
use gtk4::prelude::*;
use std::process::Command;

pub fn wire_events(widgets: &DateTimeWidgets) {
    let time_label = widgets.time_label.clone();
    let date_label = widgets.date_label.clone();
    let timezone_label = widgets.timezone_label.clone();
    let time_format_combo = widgets.time_format_combo.clone();

    update_datetime_labels(
        &time_label,
        &date_label,
        &timezone_label,
        time_format_combo.selected() == 1,
    );

    gtk4::glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
        if !time_label.is_mapped() {
            return gtk4::glib::ControlFlow::Continue;
        }
        update_datetime_labels(
            &time_label,
            &date_label,
            &timezone_label,
            time_format_combo.selected() == 1,
        );
        gtk4::glib::ControlFlow::Continue
    });

    let manual_time_box = widgets.manual.time_box.clone();
    widgets.auto_time_switch.connect_state_set(move |active| {
        manual_time_box.set_visible(!active);
        let _ = Command::new("pkexec")
            .args([
                "timedatectl",
                "set-ntp",
                if active { "true" } else { "false" },
            ])
            .spawn();
    });

    let timezone_row = widgets.manual.timezone_row.clone();
    widgets
        .auto_timezone_switch
        .connect_state_set(move |active| timezone_row.set_visible(!active));

    let time_label_fmt = widgets.time_label.clone();
    let date_label_fmt = widgets.date_label.clone();
    let timezone_label_fmt = widgets.timezone_label.clone();
    let format_combo = widgets.time_format_combo.clone();
    widgets.time_format_combo.connect_selected_notify(move |_| {
        update_datetime_labels(
            &time_label_fmt,
            &date_label_fmt,
            &timezone_label_fmt,
            format_combo.selected() == 1,
        );
    });

    let date_entry = widgets.manual.date_entry.clone();
    let time_entry = widgets.manual.time_entry.clone();
    let timezone_dropdown = widgets.manual.timezone_dropdown.clone();
    let apply_button = widgets.manual.apply_button.clone();
    let auto_timezone_switch = widgets.auto_timezone_switch.clone();
    apply_button.connect_clicked(move |button| {
        let date = date_entry.text();
        let time = time_entry.text();
        let date_time = format!("{} {}", date.trim(), time.trim());

        if NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M:%S").is_err() {
            button.set_tooltip_text(Some("Use YYYY-MM-DD and HH:MM:SS"));
            return;
        }

        button.set_tooltip_text(None);
        let _ = Command::new("pkexec")
            .args(["timedatectl", "set-time", date_time.as_str()])
            .spawn();

        if !auto_timezone_switch.is_active() {
            if let Some(timezone) = controls::timezone_at(timezone_dropdown.selected()) {
                let _ = Command::new("pkexec")
                    .args(["timedatectl", "set-timezone", timezone])
                    .spawn();
            }
        }
    });
}

fn update_datetime_labels(
    time_label: &gtk4::Label,
    date_label: &gtk4::Label,
    timezone_label: &gtk4::Label,
    use_12_hour_format: bool,
) {
    let now = Local::now();
    let time_format = if use_12_hour_format {
        "%I:%M:%S %p"
    } else {
        "%H:%M:%S"
    };
    time_label.set_text(&now.format(time_format).to_string());

    let weekday = match now.weekday() {
        chrono::Weekday::Mon => trans("weekday.mon"),
        chrono::Weekday::Tue => trans("weekday.tue"),
        chrono::Weekday::Wed => trans("weekday.wed"),
        chrono::Weekday::Thu => trans("weekday.thu"),
        chrono::Weekday::Fri => trans("weekday.fri"),
        chrono::Weekday::Sat => trans("weekday.sat"),
        chrono::Weekday::Sun => trans("weekday.sun"),
    };
    let month = trans(&format!("month.{:02}", now.month()));
    date_label.set_text(&format!(
        "{}, {} {} {}",
        weekday,
        now.day(),
        month,
        now.year()
    ));

    let offset_seconds = now.offset().local_minus_utc();
    timezone_label.set_text(&format!(
        "UTC{:+03}:{:02}",
        offset_seconds / 3600,
        (offset_seconds.abs() % 3600) / 60
    ));
}
