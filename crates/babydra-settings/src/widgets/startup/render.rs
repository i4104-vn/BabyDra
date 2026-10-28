//! Startup settings page renderer.
//! Builds the card grid, responsive FlowBox layout, and individual cards.

use crate::widgets::helpers::{create_placeholder_box, PlaceholderState};
use crate::widgets::state::StartupWidget;
use crate::widgets::startup::modal::StartupAddDialog;
use babydra_core::models::startup_command::StartupCommand;
use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Label, Orientation, Overlay, Stack, StackTransitionType};

/// Extracts a friendly display title from a startup command.
pub fn extract_command_title(cmd: &StartupCommand) -> String {
    if let Some(ref name) = cmd.name {
        let name_trim = name.trim();
        if !name_trim.is_empty() {
            return name_trim.to_string();
        }
    }
    let clean = cmd.command.trim();
    if clean.is_empty() {
        return "Command".to_string();
    }
    if let Some(rest) = clean.strip_prefix("export ") {
        if let Some((var, _)) = rest.split_once('=') {
            return format!("export {}", var.trim());
        }
        return "export".to_string();
    }
    let first_token = clean.split_whitespace().next().unwrap_or(clean);
    let bin_name = first_token
        .rsplit('/')
        .next()
        .unwrap_or(first_token)
        .trim_end_matches('&')
        .trim();
    if bin_name.is_empty() {
        clean.to_string()
    } else {
        bin_name.to_string()
    }
}

/// Chooses an appropriate icon for the startup item card.
pub fn get_command_icon(cmd: &StartupCommand) -> &'static str {
    if cmd.name.is_some() {
        return "rocket";
    }
    let lower = cmd.command.to_lowercase();
    if lower.starts_with("export")
        || lower.starts_with("dbus")
        || lower.contains("systemd")
        || lower.starts_with("sh ")
        || lower.starts_with("bash ")
    {
        "terminal"
    } else {
        "rocket"
    }
}

/// Creates a single startup application card.
pub fn create_card<F: Fn() + 'static>(cmd: &StartupCommand, on_delete: F) -> Box {
    let card = Box::new(Orientation::Vertical, 10);
    card.add_css_class("settings-card");
    card.add_css_class("device-card");
    card.add_css_class("startup-card");
    card.set_hexpand(true);
    card.set_halign(Align::Fill);
    card.set_size_request(-1, 130);

    // ── Top Row: Left icon badge, Right delete action button ──
    let top_row = Box::new(Orientation::Horizontal, 8);
    top_row.set_valign(Align::Center);

    let icon_badge = Box::new(Orientation::Vertical, 0);
    icon_badge.add_css_class("blue-icon-badge-sm");
    icon_badge.set_valign(Align::Center);
    icon_badge.set_halign(Align::Start);

    let icon_name = get_command_icon(cmd);
    let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 18);
    icon.set_pixel_size(18);
    icon.set_valign(Align::Center);
    icon.set_halign(Align::Center);
    icon.set_vexpand(true);
    icon_badge.append(&icon);
    top_row.append(&icon_badge);

    let spacer_top = Box::new(Orientation::Horizontal, 0);
    spacer_top.set_hexpand(true);
    top_row.append(&spacer_top);

    // Delete Button
    let del_btn = Button::new();
    del_btn.add_css_class("icon-btn");
    del_btn.add_css_class("circular");
    del_btn.add_css_class("delete-btn");
    del_btn.set_valign(Align::Center);
    del_btn.set_cursor_from_name(Some("pointer"));

    let del_icon = babydra_ui_kit::ui::icon::get_icon("edit-delete", 16);
    del_icon.set_pixel_size(16);
    del_btn.set_child(Some(&del_icon));
    del_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.startup_remove")));

    del_btn.connect_clicked(move |_| {
        on_delete();
    });
    top_row.append(&del_btn);

    card.append(&top_row);

    // Spacer to push bottom info
    let mid_spacer = Box::new(Orientation::Vertical, 0);
    mid_spacer.set_vexpand(true);
    card.append(&mid_spacer);

    // ── Bottom Row: Title + Command Subtitle on Left, Status Badge on Right ──
    let bottom_row = Box::new(Orientation::Horizontal, 8);
    bottom_row.set_valign(Align::End);

    let text_box = Box::new(Orientation::Vertical, 2);
    text_box.set_valign(Align::End);
    text_box.set_halign(Align::Start);
    text_box.set_hexpand(true);

    let title_text = extract_command_title(cmd);
    let name_lbl = Label::new(Some(&title_text));
    name_lbl.add_css_class("settings-row-title");
    name_lbl.add_css_class("device-card-title");
    name_lbl.set_halign(Align::Start);
    name_lbl.set_tooltip_text(Some(&title_text));
    name_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    name_lbl.set_width_chars(1);
    text_box.append(&name_lbl);

    let cmd_text = &cmd.command;
    let desc_lbl = Label::new(Some(cmd_text));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.add_css_class("device-card-subtitle");
    desc_lbl.set_halign(Align::Start);
    desc_lbl.set_tooltip_text(Some(cmd_text));
    desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    desc_lbl.set_width_chars(1);
    text_box.append(&desc_lbl);

    bottom_row.append(&text_box);

    let is_app = cmd.name.is_some();
    let badge_label = if is_app {
        babydra_core::i18n::trans("settings.startup_badge_app")
    } else {
        babydra_core::i18n::trans("settings.startup_badge_cmd")
    };
    let status_lbl = Label::new(Some(&badge_label));
    if is_app {
        status_lbl.add_css_class("device-card-status-on");
    } else {
        status_lbl.add_css_class("device-card-status-off");
    }
    status_lbl.set_valign(Align::End);
    bottom_row.append(&status_lbl);

    card.append(&bottom_row);
    card
}

/// Builds the startup application page UI with card grid and modal overlay.
pub fn build_startup_ui() -> (StartupWidget, StartupAddDialog) {
    let root = Overlay::new();
    root.set_vexpand(true);
    root.set_hexpand(true);

    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(Align::Fill);

    // ── Header Row ──────────────────────────────────────────────
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let title_lbl = Label::new(Some(&babydra_core::i18n::trans("settings.startup_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(Align::Start);
    title_lbl.set_hexpand(true);
    header_box.append(&title_lbl);

    let add_btn = Button::new();
    add_btn.add_css_class("icon-btn");
    add_btn.add_css_class("circular");
    add_btn.set_cursor_from_name(Some("pointer"));
    add_btn.set_valign(Align::Center);
    add_btn.set_size_request(34, 34);
    let add_icon = babydra_ui_kit::ui::icon::get_icon("plus", 16);
    add_icon.set_pixel_size(16);
    add_btn.set_child(Some(&add_icon));
    add_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.startup_add_new")));
    header_box.append(&add_btn);

    container.append(&header_box);

    // ── Stack with ResponsiveCardGrid and Empty State ────────────
    let stack = Stack::new();
    stack.set_vexpand(true);
    stack.set_hexpand(true);
    stack.set_transition_type(StackTransitionType::Crossfade);

    let card_grid = crate::widgets::helpers::ResponsiveCardGrid::new();
    stack.add_named(&card_grid.scrolled_win, Some("cards"));

    let empty_box = create_placeholder_box(PlaceholderState::Empty {
        title_key: "settings.startup_no_items",
        desc_key: Some("settings.startup_no_items_sub"),
        icon_name: "rocket",
    });
    stack.add_named(&empty_box, Some("empty"));

    container.append(&stack);
    root.set_child(Some(&container));

    // Modal dialog overlay
    let add_dialog = StartupAddDialog::new();
    root.add_overlay(&add_dialog.container);

    let widget = StartupWidget {
        root,
        container,
        add_btn,
        card_grid,
        stack,
    };

    (widget, add_dialog)
}
