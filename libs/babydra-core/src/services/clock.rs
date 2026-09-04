//! Shared clock/date formatting used by full-screen components
//! (lock screen, greeter) so they don't duplicate the same i18n logic.
//!
//! This module is GTK-free: it returns plain strings and lets the UI layer
//! assign them to its own labels.

use chrono::{Datelike, Local};

/// Formats the current local time and date.
///
/// Returns `(time, date)` where `time` is `HH:MM` and `date` is built from the
/// i18n template referenced by `date_format_key` (e.g. `"lock.date_format"` or
/// `"greeter.date_format"`), with the `{weekday}`, `{day}`, `{month}` and
/// `{year}` placeholders filled in.
pub fn format_clock_date(date_format_key: &str) -> (String, String) {
    let now = Local::now();
    let time = now.format("%H:%M").to_string();

    let weekday_code = match now.weekday() {
        chrono::Weekday::Mon => "mon",
        chrono::Weekday::Tue => "tue",
        chrono::Weekday::Wed => "wed",
        chrono::Weekday::Thu => "thu",
        chrono::Weekday::Fri => "fri",
        chrono::Weekday::Sat => "sat",
        chrono::Weekday::Sun => "sun",
    };
    let weekday_key = format!("weekday.{weekday_code}");
    let weekday = crate::i18n::trans(&weekday_key);

    let month_key = format!("month.{:02}", now.month());
    let month = crate::i18n::trans(&month_key);

    let day_str = format!("{:02}", now.day());
    let year_str = now.year().to_string();

    let date = crate::i18n::trans(date_format_key)
        .replace("{weekday}", &weekday)
        .replace("{day}", &day_str)
        .replace("{month}", &month)
        .replace("{year}", &year_str);

    (time, date)
}
