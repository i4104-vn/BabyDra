use crate::widgets::state::SystemUpdateWidget;
use babydra_core::models::system_update::{PackageUpdate, UpdateStatus};
use babydra_ui_kit::components::modals::PasswordDialog;
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Label, ListBox, ListBoxRow, Orientation, Overlay, ProgressBar, ScrolledWindow,
    Spinner,
};

/// Creates a new `update row`.
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
            let row_spinner = Spinner::new();
            row_spinner.set_size_request(20, 20);
            row_spinner.set_valign(gtk4::Align::Center);
            row_spinner.set_halign(gtk4::Align::Center);
            row_spinner.start();
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

/// Creates a new `empty up to date row`.
pub fn create_uptodate_row() -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_selectable(false);
    row.set_activatable(false);
    row.set_vexpand(true);
    row.set_valign(gtk4::Align::Fill);

    let row_box = Box::new(Orientation::Vertical, 14);
    row_box.add_css_class("settings-card-row");
    row_box.set_valign(gtk4::Align::Center);
    row_box.set_halign(gtk4::Align::Center);
    row_box.set_vexpand(true);
    row_box.set_hexpand(true);
    row_box.set_margin_top(48);
    row_box.set_margin_bottom(48);

    let icon_badge = Box::new(Orientation::Vertical, 0);
    icon_badge.add_css_class("blue-icon-badge");
    icon_badge.set_valign(gtk4::Align::Center);
    icon_badge.set_halign(gtk4::Align::Center);

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
    row.set_child(Some(&row_box));
    row
}

/// Build.
pub fn build(updates: &[PackageUpdate]) -> (SystemUpdateWidget, PasswordDialog) {
    let root = Overlay::new();

    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header Row with Title, Count Badge, Spinner & Actions
    let header_box = Box::new(Orientation::Horizontal, 12);

    let title_label = Label::new(Some(&babydra_core::i18n::trans("settings.update_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_halign(gtk4::Align::Start);

    let count_text = if updates.is_empty() {
        babydra_core::i18n::trans("settings.up_to_date")
    } else {
        format!(
            "{} {}",
            updates.len(),
            babydra_core::i18n::trans("settings.updates_available")
        )
    };
    let count_badge = Label::new(Some(&count_text));
    count_badge.add_css_class("update-count-badge");
    count_badge.set_hexpand(true);
    count_badge.set_halign(gtk4::Align::Start);

    let spinner = Spinner::new();
    spinner.set_visible(false);

    let refresh_btn = Button::with_label(&babydra_core::i18n::trans("settings.update_check"));
    refresh_btn.add_css_class("connect-pill-btn");
    refresh_btn.set_cursor_from_name(Some("pointer"));

    let update_all_btn = Button::with_label(&babydra_core::i18n::trans("settings.update_all"));
    update_all_btn.add_css_class("suggested-action");
    update_all_btn.set_cursor_from_name(Some("pointer"));

    if updates.is_empty() {
        update_all_btn.set_visible(false);
        refresh_btn.set_visible(true);
    } else {
        update_all_btn.set_visible(true);
        refresh_btn.set_visible(false);
    }

    header_box.append(&title_label);
    header_box.append(&count_badge);
    header_box.append(&spinner);
    header_box.append(&refresh_btn);
    header_box.append(&update_all_btn);
    container.append(&header_box);

    // Overall Progress Bar and Status Label Panel
    let progress_box = Box::new(Orientation::Vertical, 6);
    progress_box.set_visible(false);

    let status_label = Label::new(None);
    status_label.add_css_class("settings-row-desc");
    status_label.set_halign(gtk4::Align::Start);

    let progress_bar = ProgressBar::new();
    progress_bar.set_fraction(0.0);
    progress_bar.add_css_class("update-progress-bar");

    progress_box.append(&status_label);
    progress_box.append(&progress_bar);
    container.append(&progress_box);

    // Package List Glass Card
    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    if updates.is_empty() {
        list_box.append(&create_uptodate_row());
    } else {
        for pkg in updates {
            list_box.append(&create_update_row(pkg));
        }
    }

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    glass_card.append(&scroll);
    container.append(&glass_card);

    root.set_child(Some(&container));

    // Reusable Password Dialog Overlay
    let auth_dialog = PasswordDialog::new(
        "Authentication Required",
        "Enter sudo password to apply system updates:",
    );
    root.add_overlay(&auth_dialog.container);

    let widget = SystemUpdateWidget {
        root,
        container,
        count_badge,
        spinner,
        update_all_btn,
        refresh_btn,
        progress_bar,
        status_label,
        glass_card,
        list_box,
    };

    (widget, auth_dialog)
}
