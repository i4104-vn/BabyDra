use std::collections::HashMap;
use std::time::Instant;

/// Formats elapsed time since notification trigger into a user-friendly localized text.
pub fn format_elapsed_time(instant: Instant) -> String {
    let secs = instant.elapsed().as_secs();
    if secs < 60 {
        babydra_core::i18n::trans("panel.just_now")
    } else if secs < 3600 {
        babydra_core::i18n::trans("panel.minutes_ago").replace("{}", &(secs / 60).to_string())
    } else if secs < 86400 {
        babydra_core::i18n::trans("panel.hours_ago").replace("{}", &(secs / 3600).to_string())
    } else {
        babydra_core::i18n::trans("panel.days_ago").replace("{}", &(secs / 86400).to_string())
    }
}

/// Groups active notifications by application key and returns the app order.
pub fn group_notifs_by_app(
    notifications: &[babydra_island::models::ActiveNotification],
) -> (
    HashMap<String, Vec<babydra_island::models::ActiveNotification>>,
    Vec<String>,
) {
    let mut grouped = HashMap::<String, Vec<babydra_island::models::ActiveNotification>>::new();
    let mut app_order = Vec::new();

    for notif in notifications.iter() {
        let app_key = if notif.app_name.is_empty() {
            if notif.icon.is_empty() {
                "babydra".to_string()
            } else {
                notif.icon.clone()
            }
        } else {
            notif.app_name.clone()
        };
        if !grouped.contains_key(&app_key) {
            app_order.push(app_key.clone());
        }
        grouped.entry(app_key).or_default().push(notif.clone());
    }

    app_order.reverse();
    (grouped, app_order)
}
