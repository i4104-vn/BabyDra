//! Recovery / Factory Reset collapsible card UI builder.

use babydra_core::i18n::trans;
use babydra_ui_kit::components::cards::create_collapsible_card;
use babydra_ui_kit::components::modals::{
    create_ack_card, create_error_label, create_form_label, create_modern_password_entry,
    create_terminal_console, create_warning_banner, BadgeVariant, ButtonVariant,
    ModernDialogBuilder,
};
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, CheckButton, Label, Orientation, PasswordEntry, ProgressBar, TextView,
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
    // 1. Collapsible card matching settings standard
    let card = create_collapsible_card(
        &trans("settings.recovery_title"),
        Some(&trans("settings.recovery_desc")),
        Some("history"),
        false,
    );

    let content = card.content;

    // 2. Recovery Scope Section Header
    let scope_label = Label::new(Some(&trans("settings.recovery_scope_title")));
    scope_label.add_css_class("settings-row-desc");
    scope_label.set_halign(Align::Start);
    scope_label.set_margin_start(8);
    scope_label.set_margin_top(4);
    scope_label.set_margin_bottom(4);
    content.append(&scope_label);

    // 3. Recovery Scope Card Box (Static GtkBox instead of ListBox to eliminate row hover highlight)
    let scope_card = Box::new(Orientation::Vertical, 0);
    scope_card.add_css_class("settings-card");
    scope_card.add_css_class("recovery-scope-card");
    scope_card.set_margin_start(4);
    scope_card.set_margin_end(4);

    let scope_items = [
        ("close", "settings.recovery_scope_1"),
        ("desktop", "settings.recovery_scope_2"),
        ("folder", "settings.recovery_scope_3"),
        ("palette", "settings.recovery_scope_4"),
    ];

    for (i, (icon_name, text_key)) in scope_items.iter().enumerate() {
        let hbox = Box::new(Orientation::Horizontal, 14);
        hbox.add_css_class("recovery-scope-row");
        if i == scope_items.len() - 1 {
            hbox.add_css_class("no-border");
        }

        let icon_badge = Box::new(Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(Align::Center);
        icon_badge.set_halign(Align::Center);
        icon_badge.set_size_request(32, 32);

        let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 16);
        icon.set_pixel_size(16);
        icon.set_valign(Align::Center);
        icon.set_halign(Align::Center);
        icon.set_vexpand(false);
        icon.set_hexpand(false);
        icon_badge.append(&icon);
        hbox.append(&icon_badge);

        let item_lbl = Label::new(Some(&trans(text_key)));
        item_lbl.add_css_class("settings-row-title");
        item_lbl.set_halign(Align::Start);
        item_lbl.set_valign(Align::Center);
        item_lbl.set_xalign(0.0);
        item_lbl.set_justify(gtk4::Justification::Left);
        item_lbl.set_hexpand(true);
        item_lbl.set_wrap(true);
        hbox.append(&item_lbl);

        scope_card.append(&hbox);
    }
    content.append(&scope_card);

    // 4. Options Container (No hover background effect)
    let options_box = Box::new(Orientation::Vertical, 10);
    options_box.add_css_class("recovery-options-container");
    options_box.set_margin_top(12);
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

    // ── Modal 1: Confirmation & Sudo Password Dialog (Unified Modern Dialog) ──
    let auth_dialog = ModernDialogBuilder::new(440)
        .with_badge("alert", BadgeVariant::Danger)
        .with_title(&trans("settings.recovery_dialog_title"))
        .with_subtitle(&trans("settings.recovery_dialog_sub"))
        .build();

    let auth_modal_overlay = auth_dialog.container().clone();
    let auth_card = auth_dialog.card().clone();

    // Callout box for removing all apps warning
    let warn_all_apps_box =
        create_warning_banner("alert", &trans("settings.recovery_warn_all_apps"));
    warn_all_apps_box.set_visible(false);
    auth_dialog.add_child(&warn_all_apps_box);

    // Acknowledgment Checkbox Card (Clean modern layout)
    let (ack_card, understand_check) =
        create_ack_card(&trans("settings.recovery_confirm_understand"));
    auth_dialog.add_child(&ack_card);

    // Password input field
    let pwd_lbl = create_form_label(&trans("settings.sudo_password_placeholder"));
    auth_dialog.add_child(&pwd_lbl);

    let pwd_entry = create_modern_password_entry(&trans("settings.sudo_password_placeholder"));
    auth_dialog.add_child(&pwd_entry);

    let error_lbl = create_error_label();
    auth_dialog.add_child(&error_lbl);

    let cancel_btn = Button::with_label(&trans("common.cancel"));
    let confirm_btn = Button::with_label(&trans("settings.recovery_confirm_btn"));
    confirm_btn.set_sensitive(false);

    auth_dialog.add_action_buttons(&[
        (&cancel_btn, ButtonVariant::Cancel),
        (&confirm_btn, ButtonVariant::Danger),
    ]);

    // ── Modal 2: Streaming Console & Progress Dialog ──
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

    let c_title = Label::new(Some(&trans("settings.recovery_log_title")));
    c_title.add_css_class("modern-dialog-title");
    c_title.set_halign(Align::Start);
    console_title_box.append(&c_title);

    let status_lbl = Label::new(Some(&trans("settings.recovery_status_preparing")));
    status_lbl.add_css_class("modern-dialog-subtitle");
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

    let (console_scroll, text_view) = create_terminal_console();
    console_card.append(&console_scroll);

    let console_footer = Box::new(Orientation::Horizontal, 10);
    console_footer.set_halign(Align::End);
    console_footer.set_margin_top(4);

    let close_btn = Button::with_label(&trans("common.close"));
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
