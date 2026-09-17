//! Lightweight notification popup anchored to the panel clock/bell button.

use babydra_core::models::ActiveNotification;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Image, Label, Orientation, Overlay, Popover, PositionType};
use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

const POPUP_LIFETIME: Duration = Duration::from_secs(5);
const POPUP_WIDTH: i32 = 350;
const CONTENT_WIDTH: i32 = 326;
const MAX_BODY_LINES: usize = 3;
const MAX_CHARS_PER_LINE: usize = 50;
const MAX_TITLE_CHARS: usize = 72;
const MAX_APP_CHARS: usize = 32;
const SHOW_ANIMATION_MS: u64 = 240;
const HIDE_ANIMATION_MS: u64 = 180;

#[derive(Clone)]
pub struct NotificationPopup {
    popover: Popover,
    icon: Image,
    app_label: Label,
    title_label: Label,
    body_label: Label,
    more_label: Label,
    overlay: Overlay,
    app_target: Rc<std::cell::RefCell<(String, String, Option<String>)>>,
    generation: Rc<Cell<u64>>,
    hovered: Rc<Cell<bool>>,
}

impl NotificationPopup {
    pub fn new(anchor: &gtk4::Button) -> Self {
        let popover = Popover::new();
        popover.add_css_class("panel-notification-popover");
        popover.set_has_arrow(false);
        popover.set_autohide(false);
        popover.set_position(PositionType::Bottom);
        popover.set_halign(Align::End);
        popover.set_parent(anchor);
        let generation = Rc::new(Cell::new(0_u64));
        let hovered = Rc::new(Cell::new(false));
        let app_target = Rc::new(std::cell::RefCell::new((
            String::new(),
            String::new(),
            None::<String>,
        )));

        let overlay = Overlay::new();
        let card = GtkBox::new(Orientation::Vertical, 6);
        card.add_css_class("panel-notification-card");
        card.set_cursor_from_name(Some("pointer"));
        card.set_width_request(POPUP_WIDTH);
        card.set_size_request(POPUP_WIDTH, -1);
        card.set_hexpand(false);

        let header = GtkBox::new(Orientation::Horizontal, 10);
        header.set_hexpand(true);
        header.set_halign(Align::Fill);
        header.set_size_request(CONTENT_WIDTH, -1);

        let icon = Image::new();
        icon.set_pixel_size(36);
        icon.set_valign(Align::Center);
        icon.add_css_class("panel-notification-icon");
        header.append(&icon);

        let text_box = GtkBox::new(Orientation::Vertical, 4);
        text_box.set_hexpand(true);
        text_box.set_halign(Align::Fill);
        text_box.set_size_request(0, -1);

        let app_label = Label::new(None);
        app_label.add_css_class("panel-notification-app");
        app_label.set_halign(Align::Fill);
        app_label.set_xalign(0.0);
        app_label.set_single_line_mode(true);
        app_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        text_box.append(&app_label);

        let title_label = Label::new(None);
        title_label.add_css_class("panel-notification-title");
        title_label.set_halign(Align::Fill);
        title_label.set_xalign(0.0);
        title_label.set_single_line_mode(true);
        title_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        text_box.append(&title_label);

        let body_label = Label::new(None);
        body_label.add_css_class("panel-notification-body");
        body_label.set_halign(Align::Fill);
        body_label.set_xalign(0.0);
        body_label.set_hexpand(true);
        body_label.set_wrap(true);
        body_label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
        body_label.set_lines(MAX_BODY_LINES as i32);
        body_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        text_box.append(&body_label);

        let more_label = Label::new(None);
        more_label.add_css_class("panel-notification-more");
        more_label.set_halign(Align::Fill);
        more_label.set_xalign(0.0);
        more_label.set_hexpand(true);
        more_label.set_single_line_mode(true);
        more_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        more_label.set_visible(false);
        text_box.append(&more_label);
        header.append(&text_box);
        card.append(&header);

        let click = gtk4::GestureClick::new();
        let app_target_c = app_target.clone();
        let overlay_click = overlay.clone();
        let popover_click = popover.clone();
        let generation_click = generation.clone();
        click.connect_pressed(move |_, _, _, _| {
            let (app, title, cmd) = app_target_c.borrow().clone();
            if let Some(c) = cmd {
                let trimmed = c.trim();
                if !trimmed.is_empty() {
                    let _ = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(trimmed)
                        .spawn();
                    dismiss_notification(&overlay_click, &popover_click, &generation_click);
                    return;
                }
            }
            babydra_core::jump_to_app(&app, Some(&title));
            dismiss_notification(&overlay_click, &popover_click, &generation_click);
        });
        card.add_controller(click);

        let motion = gtk4::EventControllerMotion::new();
        let hovered_enter = hovered.clone();
        motion.connect_enter(move |_, _, _| {
            hovered_enter.set(true);
        });
        let hovered_leave = hovered.clone();
        let generation_leave = generation.clone();
        let overlay_leave = overlay.clone();
        let popover_leave = popover.clone();
        motion.connect_leave(move |_| {
            if hovered_leave.replace(false) {
                dismiss_notification(&overlay_leave, &popover_leave, &generation_leave);
            }
        });
        card.add_controller(motion);

        overlay.set_child(Some(&card));

        let tail = GtkBox::new(Orientation::Horizontal, 0);
        tail.add_css_class("panel-notification-tail");
        tail.set_size_request(14, 14);
        tail.set_halign(Align::End);
        tail.set_valign(Align::Start);
        tail.set_margin_end(4);
        overlay.add_overlay(&tail);

        popover.set_child(Some(&overlay));

        Self {
            popover,
            icon,
            app_label,
            title_label,
            body_label,
            more_label,
            overlay,
            app_target,
            generation,
            hovered,
        }
    }

    pub fn show(&self, notification: &ActiveNotification) {
        let app_name = if notification.app_name.is_empty() {
            babydra_core::i18n::trans("panel.notification")
        } else {
            notification.app_name.clone()
        };
        *self.app_target.borrow_mut() = (
            notification.app_name.clone(),
            notification.title.clone(),
            notification.command.clone(),
        );

        self.app_label.set_text(&truncate(&app_name, MAX_APP_CHARS));
        self.title_label
            .set_text(&truncate(&notification.title, MAX_TITLE_CHARS));

        let (body, hidden_lines) = format_body(&notification.body);
        self.body_label.set_text(&body);
        self.body_label.set_visible(!body.is_empty());
        if hidden_lines == 0 {
            self.more_label.set_visible(false);
        } else {
            let key = if hidden_lines == 1 {
                "panel.notification_more_lines_one"
            } else {
                "panel.notification_more_lines_many"
            };
            self.more_label.set_text(
                &babydra_core::i18n::trans(key).replace("{count}", &hidden_lines.to_string()),
            );
            self.more_label.set_visible(true);
        }

        let icon = notification_icon(notification);
        babydra_ui_kit::ui::icon::set_fallback_icon(
            &self.icon,
            &icon,
            "preferences-system-notifications-symbolic",
        );

        let generation = self.generation.get().wrapping_add(1);
        self.generation.set(generation);
        let generation_c = self.generation.clone();
        let popup_c = self.clone();
        glib::timeout_add_local_once(POPUP_LIFETIME, move || {
            if generation_c.get() == generation {
                popup_c.hovered.set(false);
                popup_c.close();
            }
        });

        if self.popover.parent().is_none() || self.popover.root().is_none() {
            return;
        }

        // Always reset overlay margins to 0 baseline before starting animation
        self.overlay.set_margin_top(0);
        self.overlay.set_margin_bottom(0);
        self.overlay.set_margin_start(0);
        self.overlay.set_margin_end(0);

        self.popover.popup();
        babydra_ui_kit::ui::animation::slide_in_cancelable(
            self.overlay.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Down,
            14,
            SHOW_ANIMATION_MS,
            self.generation.clone(),
            generation,
        );
    }

    pub fn is_visible(&self) -> bool {
        self.popover.is_visible()
    }

    pub fn close(&self) {
        self.hovered.set(false);
        dismiss_notification(&self.overlay, &self.popover, &self.generation);
    }
}

fn dismiss_notification(overlay: &Overlay, popover: &Popover, generation: &Rc<Cell<u64>>) {
    let next_generation = generation.get().wrapping_add(1);
    generation.set(next_generation);

    let popover_c = popover.clone();
    let generation_c = generation.clone();
    let overlay_widget = overlay.clone();
    let overlay_reset = overlay.clone();
    let finish = move || {
        if generation_c.get() == next_generation {
            if popover_c.parent().is_some() && popover_c.root().is_some() {
                popover_c.popdown();
            }
            overlay_reset.set_margin_top(0);
            overlay_reset.set_margin_bottom(0);
            overlay_reset.set_margin_start(0);
            overlay_reset.set_margin_end(0);
            babydra_core::services::notification::service::close_notif_popup();
        }
    };

    if popover.parent().is_none() || popover.root().is_none() || !popover.is_visible() {
        finish();
        return;
    }

    babydra_ui_kit::ui::animation::slide_out_cb_cancelable(
        overlay_widget.upcast_ref(),
        babydra_ui_kit::ui::animation::SlideDirection::Up,
        10,
        HIDE_ANIMATION_MS,
        false,
        generation.clone(),
        next_generation,
        finish,
    );
}

fn truncate(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let value: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{value}…")
    } else {
        value
    }
}

fn format_body(text: &str) -> (String, usize) {
    let lines = text
        .lines()
        .flat_map(|line| wrap_line(line, MAX_CHARS_PER_LINE))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let hidden_lines = lines.len().saturating_sub(MAX_BODY_LINES);
    let mut visible = lines.into_iter().take(MAX_BODY_LINES).collect::<Vec<_>>();

    if hidden_lines > 0 {
        if let Some(last) = visible.last_mut() {
            last.push('…');
        }
    }

    (visible.join("\n"), hidden_lines)
}

fn wrap_line(line: &str, max_chars: usize) -> Vec<String> {
    let mut wrapped = Vec::new();
    let mut current = String::new();

    for word in line.split_whitespace() {
        if word.chars().count() > max_chars {
            if !current.is_empty() {
                wrapped.push(std::mem::take(&mut current));
            }
            let mut chunk = String::new();
            for ch in word.chars() {
                chunk.push(ch);
                if chunk.chars().count() == max_chars {
                    wrapped.push(std::mem::take(&mut chunk));
                }
            }
            current = chunk;
        } else if current.is_empty() {
            current.push_str(word);
        } else if current.chars().count() + 1 + word.chars().count() <= max_chars {
            current.push(' ');
            current.push_str(word);
        } else {
            wrapped.push(std::mem::take(&mut current));
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        wrapped.push(current);
    }
    wrapped
}

fn notification_icon(notification: &ActiveNotification) -> String {
    let icon = notification.icon.trim();
    if !icon.is_empty() {
        let path = icon.strip_prefix("file://").unwrap_or(icon);
        if !path.starts_with('/') || Path::new(path).is_file() {
            return path.to_string();
        }
    }

    if !notification.app_name.trim().is_empty() {
        return notification.app_name.trim().to_string();
    }

    "preferences-system-notifications-symbolic".to_string()
}
