use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ListBox, Orientation, Overlay, PasswordEntry, ScrolledWindow, Spinner, TextView};
use babydra_common::models::system_update::{PackageUpdate, SystemUpdateWidget};

pub fn create_update_row(pkg: &PackageUpdate) -> Box {
    let row_box = Box::new(Orientation::Horizontal, 14);
    row_box.add_css_class("settings-card-row");
    row_box.set_margin_top(8);
    row_box.set_margin_bottom(8);
    row_box.set_margin_start(16);
    row_box.set_margin_end(16);

    let icon_box = Box::new(Orientation::Vertical, 0);
    icon_box.add_css_class("blue-icon-badge-sm");
    icon_box.set_valign(gtk4::Align::Center);
    icon_box.set_halign(gtk4::Align::Start);

    let icon_img = babydra_utils::ui::icon::get_icon("download", 18);
    icon_img.set_pixel_size(18);
    icon_img.set_valign(gtk4::Align::Center);
    icon_img.set_halign(gtk4::Align::Center);
    icon_img.set_vexpand(true);
    icon_box.append(&icon_img);
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
    row_box
}

pub fn create_empty_up_to_date_row() -> Box {
    let row_box = Box::new(Orientation::Horizontal, 14);
    row_box.add_css_class("settings-card-row");
    row_box.set_margin_top(16);
    row_box.set_margin_bottom(16);
    row_box.set_margin_start(16);
    row_box.set_margin_end(16);

    let icon_box = Box::new(Orientation::Vertical, 0);
    icon_box.add_css_class("blue-icon-badge-sm");
    icon_box.set_valign(gtk4::Align::Center);

    let icon_img = babydra_utils::ui::icon::get_icon("check", 18);
    icon_img.set_pixel_size(18);
    icon_img.set_valign(gtk4::Align::Center);
    icon_img.set_halign(gtk4::Align::Center);
    icon_img.set_vexpand(true);
    icon_box.append(&icon_img);
    row_box.append(&icon_box);

    let text_lbl = Label::new(Some(&babydra_common::i18n::t("settings.up_to_date")));
    text_lbl.add_css_class("settings-row-title");
    text_lbl.set_halign(gtk4::Align::Start);
    text_lbl.set_valign(gtk4::Align::Center);
    text_lbl.set_hexpand(true);

    row_box.append(&text_lbl);
    row_box
}

pub fn build(updates: &[PackageUpdate]) -> SystemUpdateWidget {
    let root = Overlay::new();

    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header Row with Title, Count Badge, Spinner & Actions
    let header_box = Box::new(Orientation::Horizontal, 12);

    let title_label = Label::new(Some(&babydra_common::i18n::t("settings.update_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_halign(gtk4::Align::Start);

    let count_text = if updates.is_empty() {
        babydra_common::i18n::t("settings.up_to_date")
    } else {
        format!("{} {}", updates.len(), babydra_common::i18n::t("settings.updates_available"))
    };
    let count_badge = Label::new(Some(&count_text));
    count_badge.add_css_class("update-count-badge");
    count_badge.set_hexpand(true);
    count_badge.set_halign(gtk4::Align::Start);

    let spinner = Spinner::new();
    spinner.set_visible(false);

    let refresh_btn = Button::with_label(&babydra_common::i18n::t("settings.update_check"));
    refresh_btn.add_css_class("connect-pill-btn");
    refresh_btn.set_cursor_from_name(Some("pointer"));

    let update_all_btn = Button::with_label(&babydra_common::i18n::t("settings.update_all"));
    update_all_btn.add_css_class("connect-pill-btn");
    update_all_btn.set_cursor_from_name(Some("pointer"));

    header_box.append(&title_label);
    header_box.append(&count_badge);
    header_box.append(&spinner);
    header_box.append(&refresh_btn);
    header_box.append(&update_all_btn);
    container.append(&header_box);

    // Package List Glass Card
    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    if updates.is_empty() {
        list_box.append(&create_empty_up_to_date_row());
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

    // Console Log Panel (with header bar and close button)
    let console_card = Box::new(Orientation::Vertical, 0);
    console_card.add_css_class("glass-panel");
    console_card.add_css_class("console-log-panel");
    console_card.set_vexpand(true);
    console_card.set_valign(gtk4::Align::Fill);
    console_card.set_visible(false);

    let console_header = Box::new(Orientation::Horizontal, 10);
    console_header.add_css_class("console-header");

    let console_icon = babydra_utils::ui::icon::get_icon("terminal", 16);
    console_icon.set_pixel_size(16);
    console_header.append(&console_icon);

    let console_title_lbl = Label::new(Some("System Update Console Output"));
    console_title_lbl.add_css_class("settings-row-title");
    console_title_lbl.set_hexpand(true);
    console_title_lbl.set_halign(gtk4::Align::Start);
    console_header.append(&console_title_lbl);

    let console_close_btn = Button::new();
    console_close_btn.add_css_class("icon-btn");
    console_close_btn.set_cursor_from_name(Some("pointer"));
    let close_icon = babydra_utils::ui::icon::get_icon("close", 14);
    close_icon.set_pixel_size(14);
    console_close_btn.set_child(Some(&close_icon));
    console_header.append(&console_close_btn);

    console_card.append(&console_header);

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_monospace(true);
    text_view.add_css_class("console-log-text");

    let text_buffer = text_view.buffer();

    let console_scroll = ScrolledWindow::new();
    console_scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    console_scroll.set_vexpand(true);
    console_scroll.set_valign(gtk4::Align::Fill);
    console_scroll.set_child(Some(&text_view));

    console_card.append(&console_scroll);
    container.append(&console_card);

    root.set_child(Some(&container));

    // Password Dialog Overlay (Modal Popup)
    let auth_overlay = Box::new(Orientation::Vertical, 16);
    auth_overlay.add_css_class("auth-dialog-card");
    auth_overlay.set_halign(gtk4::Align::Center);
    auth_overlay.set_valign(gtk4::Align::Center);
    auth_overlay.set_visible(false);

    let auth_header_box = Box::new(Orientation::Horizontal, 12);
    let lock_icon = babydra_utils::ui::icon::get_icon("lock", 24);
    lock_icon.set_pixel_size(24);
    auth_header_box.append(&lock_icon);

    let auth_title_box = Box::new(Orientation::Vertical, 2);
    let auth_title_lbl = Label::new(Some("Authentication Required"));
    auth_title_lbl.add_css_class("settings-row-title");
    auth_title_lbl.set_halign(gtk4::Align::Start);

    let auth_sub_lbl = Label::new(Some("Enter sudo password to apply system updates:"));
    auth_sub_lbl.add_css_class("settings-row-desc");
    auth_sub_lbl.set_halign(gtk4::Align::Start);

    auth_title_box.append(&auth_title_lbl);
    auth_title_box.append(&auth_sub_lbl);
    auth_header_box.append(&auth_title_box);
    auth_overlay.append(&auth_header_box);

    let password_entry = PasswordEntry::new();
    password_entry.add_css_class("sidebar-search-entry");
    password_entry.set_placeholder_text(Some("Password (leave empty for Polkit)..."));
    auth_overlay.append(&password_entry);

    let auth_actions_box = Box::new(Orientation::Horizontal, 8);
    auth_actions_box.set_halign(gtk4::Align::End);

    let auth_cancel_btn = Button::with_label("Cancel");
    auth_cancel_btn.add_css_class("connect-pill-btn");
    auth_cancel_btn.set_cursor_from_name(Some("pointer"));

    let auth_confirm_btn = Button::with_label("Confirm");
    auth_confirm_btn.add_css_class("suggested-action");
    auth_confirm_btn.set_cursor_from_name(Some("pointer"));

    auth_actions_box.append(&auth_cancel_btn);
    auth_actions_box.append(&auth_confirm_btn);
    auth_overlay.append(&auth_actions_box);

    root.add_overlay(&auth_overlay);

    SystemUpdateWidget {
        root,
        container,
        count_badge,
        spinner,
        update_all_btn,
        refresh_btn,
        glass_card,
        list_box,
        auth_overlay,
        password_entry,
        auth_confirm_btn,
        auth_cancel_btn,
        console_card,
        console_title_lbl,
        console_close_btn,
        text_view,
        text_buffer,
        console_scroll,
    }
}
