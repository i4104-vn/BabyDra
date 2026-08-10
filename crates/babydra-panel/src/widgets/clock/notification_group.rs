use std::time::Instant;
use std::collections::HashMap;

/// Formats elapsed time since notification trigger into a user-friendly localized text.
pub fn format_elapsed_time(instant: Instant) -> String {
    let secs = instant.elapsed().as_secs();
    if secs < 60 {
        babydra_common::i18n::t("panel.just_now")
    } else if secs < 3600 {
        babydra_common::i18n::t("panel.minutes_ago").replace("{}", &(secs / 60).to_string())
    } else if secs < 86400 {
        babydra_common::i18n::t("panel.hours_ago").replace("{}", &(secs / 3600).to_string())
    } else {
        babydra_common::i18n::t("panel.days_ago").replace("{}", &(secs / 86400).to_string())
    }
}

/// Groups active notifications by application key and returns the app order.
pub fn group_notifications_by_app(
    notifications: &[babydra_island::models::ActiveNotification],
) -> (HashMap<String, Vec<babydra_island::models::ActiveNotification>>, Vec<String>) {
    let mut grouped = HashMap::<String, Vec<babydra_island::models::ActiveNotification>>::new();
    let mut app_order = Vec::new();

    for notif in notifications.iter() {
        let app_key = if notif.icon.is_empty() {
            "system".to_string()
        } else {
            notif.icon.to_lowercase()
        };
        if !grouped.contains_key(&app_key) {
            app_order.push(app_key.clone());
        }
        grouped.entry(app_key).or_default().push(notif.clone());
    }

    app_order.reverse();
    (grouped, app_order)
}
