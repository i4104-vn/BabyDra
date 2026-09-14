//! Data → widgets for the notification badge.

use super::popover::NotificationPopover;

fn truncate(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let value: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{value}...")
    } else {
        value
    }
}

/// Pushes active notification data into the glassmorphic badge popover.
pub fn render_popover_notification(
    popover: &NotificationPopover,
    notif: &crate::models::ActiveNotification,
) {
    popover.title_lbl.set_text(&truncate(&notif.title, 40));
    popover.body_lbl.set_text(&truncate(&notif.body, 120));

    let icon = if notif.icon.is_empty() {
        "logo"
    } else {
        notif.icon.as_str()
    };
    babydra_ui_kit::ui::icon::set_fallback_icon(
        &popover.icon,
        icon,
        "preferences-system-notifications-symbolic",
    );
}
