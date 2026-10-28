//! System Update collapsible card UI builder.

use babydra_core::models::system_update::{PackageUpdate, UpdateStatus};
use babydra_ui_kit::components::cards::create_collapsible_card;
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Label, ListBox, ListBoxRow, Orientation, ProgressBar, ScrolledWindow, Spinner,
};

pub struct SystemUpdateCardWidgets {
    pub container: Box,
    pub count_badge: Label,
    pub spinner: Spinner,
    pub refresh_btn: Button,
    pub update_all_btn: Button,
    pub progress_box: Box,
    pub progress_bar: ProgressBar,
    pub status_label: Label,
    pub list_box: ListBox,
}

/// Creates a row for an individual package update.
pub fn create_update_row(pkg: &PackageUpdate) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_selectable(false);
    row.set_activatable(false);

    let row_box = Box::new(Orientation::Horizontal, 14);
    row_box.add_css_class("settings-card-row");
    row_box.set_margin_top(4);
    row_box.set_margin_bottom(4);
    row_box.set_margin_start(8);
    row_box.set_margin_end(8);

    // Left status icon badge
    let icon_box = Box::new(Orientation::Vertical, 0);
    icon_box.set_valign(gtk4::Align::Center);
    icon_box.set_halign(gtk4::Align::Start);

    match pkg.status {
        UpdateStatus::Pending => {
            icon_box.add_css_class("blue-icon-badge-sm");
            let icon_img = babydra_ui_kit::ui::icon::get_icon("download", 18);
            icon_img.set_pixel_size(18);
            icon_img.set_valign(gtk4::Align::Center);
            icon_img.set_halign(gtk4::Align::Center);
            icon_img.set_vexpand(true);
            icon_box.append(&icon_img);
        }
        UpdateStatus::Updating => {
            let row_spinner = babydra_ui_kit::components::create_loading_icon(20);
            row_spinner.set_valign(gtk4::Align::Center);
            row_spinner.set_halign(gtk4::Align::Center);
            icon_box.append(&row_spinner);
        }
        UpdateStatus::Done => {
            icon_box.add_css_class("green-icon-badge-sm");
            let icon_img = babydra_ui_kit::ui::icon::get_icon("check", 18);
            icon_img.set_pixel_size(18);
            icon_img.set_valign(gtk4::Align::Center);
            icon_img.set_halign(gtk4::Align::Center);
            icon_img.set_vexpand(true);
            icon_box.append(&icon_img);
        }
        UpdateStatus::Failed => {
            icon_box.add_css_class("red-icon-badge-sm");
            let icon_img = babydra_ui_kit::ui::icon::get_icon("close", 18);
            icon_img.set_pixel_size(18);
            icon_img.set_valign(gtk4::Align::Center);
            icon_img.set_halign(gtk4::Align::Center);
            icon_img.set_vexpand(true);
            icon_box.append(&icon_img);
        }
    }
    row_box.append(&icon_box);

    let text_box = Box::new(Orientation::Vertical, 2);
    text_box.set_hexpand(true);
    text_box.set_valign(gtk4::Align::Center);

    let name_lbl = Label::new(Some(&pkg.name));
    name_lbl.add_css_class("settings-row-title");
    name_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&name_lbl);

    let ver_lbl = Label::new(Some(&format!("{} → {}", pkg.old_version, pkg.new_version)));
    ver_lbl.add_css_class("settings-row-desc");
    ver_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&ver_lbl);

    row_box.append(&text_box);

    // Right status indicator badge
    let status_badge_lbl = match pkg.status {
        UpdateStatus::Pending => {
            let lbl = Label::new(Some(&babydra_core::i18n::trans("settings.status_waiting")));
            lbl.add_css_class("settings-row-desc");
            lbl.set_valign(gtk4::Align::Center);
            Some(lbl)
        }
        UpdateStatus::Updating => {
            let lbl = Label::new(Some(&babydra_core::i18n::trans("settings.status_pending")));
            lbl.add_css_class("settings-row-desc");
            lbl.set_valign(gtk4::Align::Center);
            Some(lbl)
        }
        UpdateStatus::Done => {
            let lbl = Label::new(Some(&babydra_core::i18n::trans("settings.status_done")));
            lbl.add_css_class("status-success-badge");
            lbl.set_valign(gtk4::Align::Center);
            Some(lbl)
        }
        UpdateStatus::Failed => {
            let lbl = Label::new(Some(&babydra_core::i18n::trans("settings.status_failed")));
            lbl.add_css_class("status-error-badge");
            lbl.set_valign(gtk4::Align::Center);
            Some(lbl)
        }
    };

    if let Some(badge) = status_badge_lbl {
        row_box.append(&badge);
    }

    row.set_child(Some(&row_box));
    row
}

/// Creates a row when the system has no updates pending.
pub fn create_uptodate_row() -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_selectable(false);
    row.set_activatable(false);
    row.set_vexpand(true);
    row.set_valign(gtk4::Align::Fill);

    let row_box = Box::new(Orientation::Vertical, 12);
    row_box.set_valign(gtk4::Align::Center);
    row_box.set_halign(gtk4::Align::Center);
    row_box.set_vexpand(true);
    row_box.set_hexpand(true);
    row_box.set_margin_top(40);
    row_box.set_margin_bottom(40);
    row_box.set_margin_start(16);
    row_box.set_margin_end(16);

    let icon_badge = Box::new(Orientation::Vertical, 0);
    icon_badge.add_css_class("green-icon-badge-sm");
    icon_badge.set_valign(gtk4::Align::Center);
    icon_badge.set_halign(gtk4::Align::Center);
    icon_badge.set_size_request(48, 48);

    let icon_img = babydra_ui_kit::ui::icon::get_icon("check", 24);
    icon_img.set_pixel_size(24);
    icon_img.set_valign(gtk4::Align::Center);
    icon_img.set_halign(gtk4::Align::Center);
    icon_img.set_vexpand(true);
    icon_badge.append(&icon_img);
    row_box.append(&icon_badge);

    let text_lbl = Label::new(Some(&babydra_core::i18n::trans("settings.up_to_date")));
    text_lbl.add_css_class("settings-row-title");
    text_lbl.set_halign(gtk4::Align::Center);
    row_box.append(&text_lbl);

    let desc_lbl = Label::new(Some(&babydra_core::i18n::trans(
        "settings.update_uptodate_desc",
    )));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(gtk4::Align::Center);
    row_box.append(&desc_lbl);

    row.set_child(Some(&row_box));
    row
}

/// Renders the System Update collapsible card.
pub fn render_system_update_card() -> SystemUpdateCardWidgets {
    let card = create_collapsible_card(
        &babydra_core::i18n::trans("settings.update_title"),
        Some(&babydra_core::i18n::trans("settings.update_subtitle")),
        Some("history"),
        true,
    );

    // Tag badge placed right next to the card title in the header
    let count_badge = Label::new(Some(&babydra_core::i18n::trans("settings.up_to_date")));
    count_badge.add_css_class("update-count-badge");
    count_badge.set_valign(gtk4::Align::Center);
    card.title_box.append(&count_badge);

    // Action buttons & spinner placed on the far right of the header row
    let spinner = Spinner::new();
    spinner.set_visible(false);
    spinner.set_valign(gtk4::Align::Center);
    card.action_box.append(&spinner);

    let refresh_btn = Button::with_label(&babydra_core::i18n::trans("settings.update_check"));
    refresh_btn.add_css_class("connect-pill-btn");
    refresh_btn.set_cursor_from_name(Some("pointer"));
    refresh_btn.set_valign(gtk4::Align::Center);
    card.action_box.append(&refresh_btn);

    let update_all_btn = Button::with_label(&babydra_core::i18n::trans("settings.update_all"));
    update_all_btn.add_css_class("suggested-action");
    update_all_btn.set_cursor_from_name(Some("pointer"));
    update_all_btn.set_valign(gtk4::Align::Center);
    update_all_btn.set_visible(false);
    card.action_box.append(&update_all_btn);

    let content = card.content;

    // Progress bar panel
    let progress_box = Box::new(Orientation::Vertical, 6);
    progress_box.set_margin_start(8);
    progress_box.set_margin_end(8);
    progress_box.set_margin_top(4);
    progress_box.set_margin_bottom(4);
    progress_box.set_visible(false);

    let status_label = Label::new(None);
    status_label.add_css_class("settings-row-desc");
    status_label.set_halign(gtk4::Align::Start);

    let progress_bar = ProgressBar::new();
    progress_bar.set_fraction(0.0);
    progress_bar.add_css_class("update-progress-bar");

    progress_box.append(&status_label);
    progress_box.append(&progress_bar);
    content.append(&progress_box);

    // Package List
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    list_box.append(&create_uptodate_row());

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_min_content_height(400);
    scroll.set_max_content_height(620);
    scroll.set_propagate_natural_height(true);
    scroll.set_child(Some(&list_box));

    content.append(&scroll);

    SystemUpdateCardWidgets {
        container: card.container,
        count_badge,
        spinner,
        refresh_btn,
        update_all_btn,
        progress_box,
        progress_bar,
        status_label,
        list_box,
    }
}
