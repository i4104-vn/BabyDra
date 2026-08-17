//! Data → widgets: pushes a notification into the view and re-measures height.

use gtk4::prelude::*;

use super::NotificationFeature;

const TARGET_WIDTH: i32 = 280;

impl NotificationFeature {
    /// Pushes the notification content into the widgets and re-measures height
    /// so the capsule hugs the content.
    pub(crate) fn render(&mut self, notif: &crate::models::ActiveNotification) {
        let truncated_title = if notif.title.chars().count() > 35 {
            let t: String = notif.title.chars().take(35).collect();
            t + "..."
        } else {
            notif.title.clone()
        };
        self.view.title_lbl.set_text(&truncated_title);

        let truncated_body = if notif.body.chars().count() > 80 {
            let b: String = notif.body.chars().take(80).collect();
            b + "..."
        } else {
            notif.body.clone()
        };
        self.view.body_lbl.set_text(&truncated_body);

        if let Some(child) = self.view.art_container.first_child() {
            self.view.art_container.remove(&child);
        }

        let mut use_logo = notif.icon.is_empty();
        if !use_logo {
            if notif.icon.starts_with('/') {
                if !std::path::Path::new(&notif.icon).exists() {
                    use_logo = true;
                }
            } else {
                let mut clean_name = notif.icon.clone();
                for ext in &[".png", ".svg", ".xpm", ".jpg", ".jpeg", ".gif"] {
                    if clean_name.to_lowercase().ends_with(ext) {
                        clean_name = clean_name[..clean_name.len() - ext.len()].to_string();
                        break;
                    }
                }
                if let Some(disp) = gdk4::Display::default() {
                    let theme = gtk4::IconTheme::for_display(&disp);
                    if !theme.has_icon(&clean_name) {
                        use_logo = true;
                    }
                } else {
                    use_logo = true;
                }
            }
        }

        let notif_icon = if use_logo {
            babydra_ui_kit::ui::icon::get_icon("logo", 24)
        } else {
            babydra_ui_kit::ui::icon::get_system_or_file_icon(
                &notif.icon,
                "preferences-system-notifications-symbolic",
            )
        };
        notif_icon.set_pixel_size(24);
        notif_icon.add_css_class("notch-album-art");
        self.view.art_container.append(&notif_icon);

        // Re-measure desired height so the capsule hugs the content.
        let (_, nat_height, _, _) = self
            .view
            .root
            .measure(gtk4::Orientation::Vertical, TARGET_WIDTH - 32);
        let h = (nat_height + 16).max(48);
        self.desired = (TARGET_WIDTH, h);
    }
}
