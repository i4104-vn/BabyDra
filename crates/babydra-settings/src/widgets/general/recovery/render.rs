//! Recovery / Factory Reset collapsible card UI builder.

use babydra_core::i18n::trans;
use babydra_ui_kit::components::cards::create_collapsible_card;
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, CheckButton, Label, ListBox, ListBoxRow, Orientation, PasswordEntry,
    ProgressBar, TextView,
};

#[allow(dead_code)]
pub struct RecoveryCardWidgets {
    pub container: Box,
    pub remove_pkgs_check: CheckButton,
    pub remove_all_apps_check: CheckButton,
    pub start_btn: Button,
    // Auth modal
    pub auth_modal_overlay: Box,
    pub auth_card: Box,
    pub warn_all_apps_box: Box,
    pub warn_all_apps_lbl: Label,
    pub understand_check: CheckButton,
    pub pwd_entry: PasswordEntry,
    pub error_lbl: Label,
    pub cancel_btn: Button,
    pub confirm_btn: Button,
    // Console log modal
    pub console_modal_overlay: Box,
    pub console_card: Box,
    pub status_badge: Label,
    pub status_lbl: Label,
    pub progress_bar: ProgressBar,
    pub text_view: TextView,
    pub close_btn: Button,
    pub reboot_btn: Button,
}

pub fn render_recovery_card() -> RecoveryCardWidgets {
    let card = create_collapsible_card(
        &trans("settings.recovery_title"),
        Some(&trans("settings.recovery_desc")),
        Some("history"),
        false,
    );

    let content = card.content;

    // Scope list of items affected
    let scope_label = Label::new(Some(&trans("settings.recovery_scope_title")));
    scope_label.add_css_class("settings-row-desc");
    scope_label.set_halign(Align::Start);
    scope_label.set_margin_start(8);
    scope_label.set_margin_top(4);
    scope_label.set_margin_bottom(2);
    content.append(&scope_label);

    let scope_list = ListBox::new();
    scope_list.set_selection_mode(gtk4::SelectionMode::None);
    scope_list.add_css_class("settings-card");
    scope_list.set_margin_start(4);
    scope_list.set_margin_end(4);

    let scope_items = [
        ("close", "settings.recovery_scope_1"),
        ("desktop", "settings.recovery_scope_2"),
        ("folder", "settings.recovery_scope_3"),
        ("palette", "settings.recovery_scope_4"),
    ];

    for (icon_name, text_key) in scope_items {
        let row = ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let hbox = Box::new(Orientation::Horizontal, 14);
        hbox.set_margin_top(6);
        hbox.set_margin_bottom(6);
        hbox.set_margin_start(6);
        hbox.set_margin_end(6);

        let icon_badge = Box::new(Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(Align::Center);
        icon_badge.set_size_request(28, 28);

        let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 16);
        icon.set_pixel_size(16);
        icon.set_vexpand(true);
        icon.set_valign(Align::Center);
        icon_badge.append(&icon);
        hbox.append(&icon_badge);

        let item_lbl = Label::new(Some(&trans(text_key)));
        item_lbl.add_css_class("settings-row-title");
        item_lbl.set_halign(Align::Start);
        item_lbl.set_hexpand(true);
        item_lbl.set_wrap(true);
        hbox.append(&item_lbl);

        row.set_child(Some(&hbox));
        scope_list.append(&row);
    }
    content.append(&scope_list);

    // Options Checkboxes & Action Row
    let options_box = Box::new(Orientation::Vertical, 10);
    options_box.add_css_class("settings-card-row");
    options_box.set_margin_top(8);
    options_box.set_margin_start(8);
    options_box.set_margin_end(8);
    options_box.set_margin_bottom(4);

    let remove_pkgs_check = CheckButton::with_label(&trans("settings.recovery_remove_pkgs_opt"));
    remove_pkgs_check.set_active(true);
    remove_pkgs_check.set_cursor_from_name(Some("pointer"));
    options_box.append(&remove_pkgs_check);

    let remove_all_apps_check =
        CheckButton::with_label(&trans("settings.recovery_remove_all_apps_opt"));
    remove_all_apps_check.set_active(false);
    remove_all_apps_check.set_cursor_from_name(Some("pointer"));
    options_box.append(&remove_all_apps_check);

    let action_row = Box::new(Orientation::Horizontal, 12);
    action_row.set_halign(Align::End);
    action_row.set_margin_top(8);

    let start_btn = Button::with_label(&trans("settings.recovery_start_btn"));
    start_btn.add_css_class("connect-pill-btn");
    start_btn.add_css_class("destructive-action");
    start_btn.set_cursor_from_name(Some("pointer"));
    start_btn.set_valign(Align::Center);
    action_row.append(&start_btn);
    options_box.append(&action_row);

    content.append(&options_box);

    // ── Modal 1: Confirmation & Sudo Password Dialog with Backdrop Scrim ──
    let auth_modal_overlay = Box::new(Orientation::Vertical, 0);
    auth_modal_overlay.add_css_class("modal-scrim-layer");
    auth_modal_overlay.set_hexpand(true);
    auth_modal_overlay.set_vexpand(true);
    auth_modal_overlay.set_halign(Align::Fill);
    auth_modal_overlay.set_valign(Align::Fill);
    auth_modal_overlay.set_visible(false);

    let auth_click_blocker = gtk4::GestureClick::new();
    auth_click_blocker.connect_pressed(|_, _, _, _| {});
    auth_modal_overlay.add_controller(auth_click_blocker);

    let auth_card = Box::new(Orientation::Vertical, 16);
    auth_card.add_css_class("modern-modal-card");
    auth_card.set_halign(Align::Center);
    auth_card.set_valign(Align::Center);
    auth_card.set_width_request(440);
    auth_card.set_margin_start(16);
    auth_card.set_margin_end(16);

    let auth_header = Box::new(Orientation::Horizontal, 14);
    auth_header.set_valign(Align::Center);

    let danger_badge = Box::new(Orientation::Vertical, 0);
    danger_badge.add_css_class("recovery-danger-badge");
    danger_badge.set_size_request(46, 46);
    danger_badge.set_halign(Align::Center);
    danger_badge.set_valign(Align::Center);

    let lock_icon = babydra_ui_kit::ui::icon::get_icon("privacy", 22);
    lock_icon.set_pixel_size(22);
    lock_icon.set_vexpand(true);
    lock_icon.set_hexpand(true);
    lock_icon.set_valign(Align::Center);
    lock_icon.set_halign(Align::Center);
    danger_badge.append(&lock_icon);
    auth_header.append(&danger_badge);

    let auth_title_box = Box::new(Orientation::Vertical, 3);
    auth_title_box.set_valign(Align::Center);
    auth_title_box.set_hexpand(true);

    let modal_title = Label::new(Some(&trans("settings.recovery_auth_title")));
    modal_title.add_css_class("modern-modal-title");
    modal_title.set_halign(Align::Start);
    auth_title_box.append(&modal_title);

    let modal_sub = Label::new(Some(&trans("settings.recovery_auth_desc")));
    modal_sub.add_css_class("modern-modal-desc");
    modal_sub.set_halign(Align::Start);
    modal_sub.set_wrap(true);
    auth_title_box.append(&modal_sub);

    auth_header.append(&auth_title_box);
    auth_card.append(&auth_header);

    // Callout box for removing all apps warning
    let warn_all_apps_box = Box::new(Orientation::Horizontal, 12);
    warn_all_apps_box.add_css_class("recovery-warning-callout");
    warn_all_apps_box.set_visible(false);

    let alert_icon = babydra_ui_kit::ui::icon::get_icon("alert", 20);
    alert_icon.set_pixel_size(20);
    alert_icon.set_valign(Align::Start);
    warn_all_apps_box.append(&alert_icon);

    let warn_all_apps_lbl = Label::new(Some(&trans("settings.recovery_remove_all_apps_warn")));
    warn_all_apps_lbl.add_css_class("recovery-warning-text");
    warn_all_apps_lbl.set_halign(Align::Start);
    warn_all_apps_lbl.set_wrap(true);
    warn_all_apps_lbl.set_hexpand(true);
    warn_all_apps_box.append(&warn_all_apps_lbl);
    auth_card.append(&warn_all_apps_box);

    // Checkbox: "I understand that all configurations will be reset"
    let understand_check = CheckButton::with_label(&trans("settings.recovery_confirm_check"));
    understand_check.add_css_class("recovery-ack-check");
    understand_check.set_cursor_from_name(Some("pointer"));
    auth_card.append(&understand_check);

    // Password input field
    let pwd_box = Box::new(Orientation::Vertical, 6);
    let pwd_entry = PasswordEntry::new();
    pwd_entry.set_placeholder_text(Some(&trans("settings.recovery_pwd_placeholder")));
    pwd_entry.set_show_peek_icon(true);
    pwd_entry.add_css_class("modern-password-entry");
    pwd_box.append(&pwd_entry);

    let error_lbl = Label::new(None);
    error_lbl.add_css_class("modern-modal-error");
    error_lbl.set_halign(Align::Start);
    error_lbl.set_visible(false);
    pwd_box.append(&error_lbl);
    auth_card.append(&pwd_box);

    // Action Buttons: Cancel and Confirm Reset
    let btn_box = Box::new(Orientation::Horizontal, 10);
    btn_box.set_halign(Align::End);
    btn_box.set_margin_top(8);

    let cancel_btn = Button::with_label(&trans("settings.cancel"));
    cancel_btn.add_css_class("connect-pill-btn");
    cancel_btn.set_cursor_from_name(Some("pointer"));
    btn_box.append(&cancel_btn);

    let confirm_btn = Button::with_label(&trans("settings.recovery_confirm_btn"));
    confirm_btn.add_css_class("connect-pill-btn");
    confirm_btn.add_css_class("destructive-action");
    confirm_btn.set_cursor_from_name(Some("pointer"));
    confirm_btn.set_sensitive(false);
    btn_box.append(&confirm_btn);

    auth_card.append(&btn_box);
    auth_modal_overlay.append(&auth_card);

    // ── Modal 2: Streaming Console & Progress Dialog with Scrim ──
    let console_modal_overlay = Box::new(Orientation::Vertical, 0);
    console_modal_overlay.add_css_class("modal-scrim-layer");
    console_modal_overlay.set_hexpand(true);
    console_modal_overlay.set_vexpand(true);
    console_modal_overlay.set_halign(Align::Fill);
    console_modal_overlay.set_valign(Align::Fill);
    console_modal_overlay.set_visible(false);

    let console_click_blocker = gtk4::GestureClick::new();
    console_click_blocker.connect_pressed(|_, _, _, _| {});
    console_modal_overlay.add_controller(console_click_blocker);

    let console_card = Box::new(Orientation::Vertical, 16);
    console_card.add_css_class("modern-modal-card");
    console_card.set_halign(Align::Center);
    console_card.set_valign(Align::Center);
    console_card.set_width_request(620);
    console_card.set_height_request(480);
    console_card.set_margin_start(16);
    console_card.set_margin_end(16);

    let console_header = Box::new(Orientation::Horizontal, 12);
    console_header.set_valign(Align::Center);

    let console_icon_badge = Box::new(Orientation::Vertical, 0);
    console_icon_badge.add_css_class("blue-icon-badge-sm");
    console_icon_badge.set_size_request(40, 40);
    console_icon_badge.set_valign(Align::Center);
    console_icon_badge.set_halign(Align::Center);

    let term_icon = babydra_ui_kit::ui::icon::get_icon("terminal", 20);
    term_icon.set_pixel_size(20);
    term_icon.set_vexpand(true);
    term_icon.set_valign(Align::Center);
    console_icon_badge.append(&term_icon);
    console_header.append(&console_icon_badge);

    let console_title_box = Box::new(Orientation::Vertical, 2);
    console_title_box.set_hexpand(true);
    console_title_box.set_valign(Align::Center);

    let c_title = Label::new(Some(&trans("settings.recovery_progress_title")));
    c_title.add_css_class("modern-modal-title");
    c_title.set_halign(Align::Start);
    console_title_box.append(&c_title);

    let status_lbl = Label::new(Some(&trans("settings.recovery_status_preparing")));
    status_lbl.add_css_class("modern-modal-desc");
    status_lbl.set_halign(Align::Start);
    console_title_box.append(&status_lbl);
    console_header.append(&console_title_box);

    let status_badge = Label::new(Some(&trans("settings.recovery_badge_running")));
    status_badge.add_css_class("update-count-badge");
    status_badge.set_valign(Align::Center);
    console_header.append(&status_badge);

    console_card.append(&console_header);

    let progress_bar = ProgressBar::new();
    progress_bar.set_fraction(0.05);
    progress_bar.add_css_class("update-progress-bar");
    console_card.append(&progress_bar);

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_monospace(true);
    text_view.add_css_class("recovery-console-terminal");

    let console_scroll = gtk4::ScrolledWindow::new();
    console_scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    console_scroll.set_vexpand(true);
    console_scroll.set_hexpand(true);
    console_scroll.add_css_class("recovery-console-scroll");
    console_scroll.set_child(Some(&text_view));
    console_card.append(&console_scroll);

    let console_footer = Box::new(Orientation::Horizontal, 10);
    console_footer.set_halign(Align::End);
    console_footer.set_margin_top(4);

    let close_btn = Button::with_label(&trans("settings.close"));
    close_btn.add_css_class("connect-pill-btn");
    close_btn.set_cursor_from_name(Some("pointer"));
    close_btn.set_visible(false);
    console_footer.append(&close_btn);

    let reboot_btn = Button::with_label(&trans("settings.recovery_reboot_btn"));
    reboot_btn.add_css_class("connect-pill-btn");
    reboot_btn.add_css_class("suggested-action");
    reboot_btn.set_cursor_from_name(Some("pointer"));
    reboot_btn.set_visible(false);
    console_footer.append(&reboot_btn);

    console_card.append(&console_footer);
    console_modal_overlay.append(&console_card);

    RecoveryCardWidgets {
        container: card.container,
        remove_pkgs_check,
        remove_all_apps_check,
        start_btn,
        auth_modal_overlay,
        auth_card,
        warn_all_apps_box,
        warn_all_apps_lbl,
        understand_check,
        pwd_entry,
        error_lbl,
        cancel_btn,
        confirm_btn,
        console_modal_overlay,
        console_card,
        status_badge,
        status_lbl,
        progress_bar,
        text_view,
        close_btn,
        reboot_btn,
    }
}
