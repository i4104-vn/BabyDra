//! Shared clock/date label updater used by full-screen components
//! (lock screen, greeter) so they don't duplicate the same i18n logic.

/// Updates the given clock/date labels with the current local time.
///
/// `date_format_key` is an i18n key resolving to a template that contains the
/// `{weekday}`, `{day}`, `{month}` and `{year}` placeholders (e.g.
/// `"lock.date_format"` or `"greeter.date_format"`).
pub fn update_clock(clock_label: &gtk4::Label, date_label: &gtk4::Label, date_format_key: &str) {
    let now = chrono::Local::now();
    clock_label.set_text(&now.format("%H:%M").to_string());

    let weekday_key = format!("weekday.{}", now.format("%a").to_string().to_lowercase());
    let weekday = crate::i18n::t(&weekday_key);
    let month_key = format!("month.{}", now.format("%m"));
    let month = crate::i18n::t(&month_key);

    let date_str = crate::i18n::t(date_format_key)
        .replace("{weekday}", &weekday)
        .replace("{day}", &now.format("%d").to_string())
        .replace("{month}", &month)
        .replace("{year}", &now.format("%Y").to_string());
    date_label.set_text(&date_str);
}
