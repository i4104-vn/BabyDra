//! Recovery / Factory Reset UI layout generator.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, CheckButton, Label, ListBox, ListBoxRow, Orientation, Overlay,
    PasswordEntry, ProgressBar, ScrolledWindow, TextView,
};

#[allow(dead_code)]
pub struct RecoveryWidgets {
    pub root: Overlay,
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

pub fn build_recovery_ui() -> RecoveryWidgets {
    let root = Overlay::new();

    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(Align::Fill);

    // Page Title & Header
    let header_box = Box::new(Orientation::Vertical, 4);
    let title_lbl = Label::new(Some(&trans("settings.recovery_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(Align::Start);
    header_box.append(&title_lbl);

    let desc_lbl = Label::new(Some(&trans("settings.recovery_desc")));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(Align::Start);
    header_box.append(&desc_lbl);
    container.append(&header_box);

    let content_scroll = ScrolledWindow::new();
    content_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    content_scroll.set_vexpand(true);

    let content_box = Box::new(Orientation::Vertical, 14);

    // ── Card 1: Top Hero Summary Card ──
    let hero_card = Box::new(Orientation::Horizontal, 16);
    hero_card.add_css_class("glass-panel");
    hero_card.set_margin_top(2);
    hero_card.set_margin_bottom(4);
    hero_card.set_margin_start(4);
    hero_card.set_margin_end(4);

    let hero_icon_box = Box::new(Orientation::Vertical, 0);
    hero_icon_box.add_css_class("red-icon-badge-sm");
    hero_icon_box.set_size_request(48, 48);
    hero_icon_box.set_valign(Align::Center);
    hero_icon_box.set_halign(Align::Center);

    let hero_icon = babydra_ui_kit::ui::icon::get_icon("history", 24);
    hero_icon.set_pixel_size(24);
    hero_icon.set_vexpand(true);
    hero_icon.set_valign(Align::Center);
    hero_icon_box.append(&hero_icon);
    hero_card.append(&hero_icon_box);

    let hero_text_box = Box::new(Orientation::Vertical, 3);
    hero_text_box.set_hexpand(true);
    hero_text_box.set_valign(Align::Center);

    let hero_title = Label::new(Some(&trans("settings.recovery_title")));
    hero_title.add_css_class("hero-hostname");
    hero_title.set_halign(Align::Start);
    hero_text_box.append(&hero_title);

    let hero_sub = Label::new(Some(&trans("settings.recovery_desc")));
    hero_sub.add_css_class("hero-subtitle");
    hero_sub.set_halign(Align::Start);
    hero_sub.set_wrap(true);
    hero_text_box.append(&hero_sub);
    hero_card.append(&hero_text_box);

    content_box.append(&hero_card);

    // ── Card 2: Recovery Scope & Changes ──
    let scope_label = Label::new(Some(&trans("settings.recovery_scope_title")));
    scope_label.add_css_class("settings-row-desc");
    scope_label.set_halign(Align::Start);
    scope_label.set_margin_start(8);
    scope_label.set_margin_top(8);
    scope_label.set_margin_bottom(2);
    content_box.append(&scope_label);

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
        icon_badge.set_size_request(32, 32);

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
    content_box.append(&scope_list);

    // ── Card 3: Danger Zone Action Card ──
    let danger_title = Label::new(Some(&trans("settings.recovery_danger_title")));
    danger_title.add_css_class("settings-row-desc");
    danger_title.add_css_class("destructive-action");
    danger_title.set_halign(Align::Start);
    danger_title.set_margin_start(8);
    danger_title.set_margin_top(12);
    danger_title.set_margin_bottom(2);
    content_box.append(&danger_title);

    let danger_card = Box::new(Orientation::Vertical, 14);
    danger_card.add_css_class("settings-card");
    danger_card.set_margin_start(4);
    danger_card.set_margin_end(4);

    let danger_desc = Label::new(Some(&trans("settings.recovery_danger_desc")));
    danger_desc.add_css_class("settings-row-desc");
    danger_desc.set_halign(Align::Start);
    danger_desc.set_wrap(true);
    danger_card.append(&danger_desc);

    // Checkbox for removing packages
    let remove_pkgs_check = CheckButton::with_label(&trans("settings.recovery_remove_pkgs_opt"));
    remove_pkgs_check.set_active(true);
    remove_pkgs_check.set_cursor_from_name(Some("pointer"));
    danger_card.append(&remove_pkgs_check);

    // Checkbox for removing all user-installed applications
    let remove_all_apps_check =
        CheckButton::with_label(&trans("settings.recovery_remove_all_apps_opt"));
    remove_all_apps_check.set_active(false);
    remove_all_apps_check.set_cursor_from_name(Some("pointer"));
    danger_card.append(&remove_all_apps_check);

    let all_apps_desc = Label::new(Some(&trans("settings.recovery_remove_all_apps_desc")));
    all_apps_desc.add_css_class("settings-row-desc");
    all_apps_desc.set_halign(Align::Start);
    all_apps_desc.set_wrap(true);
    all_apps_desc.set_margin_start(24);
    danger_card.append(&all_apps_desc);

    // Trigger action row
    let action_row = Box::new(Orientation::Horizontal, 12);
    action_row.set_halign(Align::End);
    action_row.set_margin_top(4);

    let start_btn = Button::with_label(&trans("settings.recovery_start_btn"));
    start_btn.add_css_class("connect-pill-btn");
    start_btn.add_css_class("destructive-action");
    start_btn.set_cursor_from_name(Some("pointer"));
    start_btn.set_valign(Align::Center);
    action_row.append(&start_btn);

    danger_card.append(&action_row);
    content_box.append(&danger_card);

    content_scroll.set_child(Some(&content_box));
    container.append(&content_scroll);
    root.set_child(Some(&container));

    // ── Modal 1: Confirmation & Sudo Password Dialog with Backdrop Scrim ──
    let auth_modal_overlay = Box::new(Orientation::Vertical, 0);
    auth_modal_overlay.add_css_class("modal-scrim-layer");
    auth_modal_overlay.set_hexpand(true);
    auth_modal_overlay.set_vexpand(true);
    auth_modal_overlay.set_halign(Align::Fill);
    auth_modal_overlay.set_valign(Align::Fill);
    auth_modal_overlay.set_visible(false);

    // Consume clicks so underlying controls cannot be triggered
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

    // Header: Glowing squircle badge + Title & Subtitle
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

    let auth_title = Label::new(Some(&trans("settings.recovery_dialog_title")));
    auth_title.add_css_class("recovery-modal-title");
    auth_title.set_halign(Align::Start);

    let auth_sub = Label::new(Some(&trans("settings.recovery_dialog_sub")));
    auth_sub.add_css_class("recovery-modal-subtitle");
    auth_sub.set_halign(Align::Start);
    auth_sub.set_wrap(true);

    auth_title_box.append(&auth_title);
    auth_title_box.append(&auth_sub);
    auth_header.append(&auth_title_box);
    auth_card.append(&auth_header);

    // General Warning Callout
    let warning_callout = Box::new(Orientation::Horizontal, 10);
    warning_callout.add_css_class("recovery-warning-banner");
    warning_callout.set_valign(Align::Center);

    let warn_icon = babydra_ui_kit::ui::icon::get_icon("info", 16);
    warn_icon.set_pixel_size(16);
    warn_icon.set_valign(Align::Center);
    warning_callout.append(&warn_icon);

    let warn_callout_lbl = Label::new(Some(&trans("settings.recovery_warning_callout")));
    warn_callout_lbl.add_css_class("recovery-warning-text");
    warn_callout_lbl.set_valign(Align::Center);
    warn_callout_lbl.set_halign(Align::Start);
    warn_callout_lbl.set_wrap(true);
    warn_callout_lbl.set_hexpand(true);
    warning_callout.append(&warn_callout_lbl);
    auth_card.append(&warning_callout);

    // Extra Callout when "Remove all apps" is selected
    let warn_all_apps_box = Box::new(Orientation::Horizontal, 10);
    warn_all_apps_box.add_css_class("recovery-warning-banner");
    warn_all_apps_box.set_valign(Align::Center);
    warn_all_apps_box.set_visible(false);

    let all_apps_icon = babydra_ui_kit::ui::icon::get_icon("trash", 16);
    all_apps_icon.set_pixel_size(16);
    all_apps_icon.set_valign(Align::Center);
    warn_all_apps_box.append(&all_apps_icon);

    let warn_all_apps_lbl = Label::new(Some(&trans("settings.recovery_warn_all_apps")));
    warn_all_apps_lbl.add_css_class("recovery-warning-text");
    warn_all_apps_lbl.set_valign(Align::Center);
    warn_all_apps_lbl.set_halign(Align::Start);
    warn_all_apps_lbl.set_wrap(true);
    warn_all_apps_lbl.set_hexpand(true);
    warn_all_apps_box.append(&warn_all_apps_lbl);
    auth_card.append(&warn_all_apps_box);

    // Sudo password field section
    let pwd_section = Box::new(Orientation::Vertical, 6);
    let pwd_lbl_box = Box::new(Orientation::Horizontal, 6);
    pwd_lbl_box.set_valign(Align::Center);

    let key_icon = babydra_ui_kit::ui::icon::get_icon("lock", 14);
    key_icon.set_pixel_size(14);
    key_icon.set_valign(Align::Center);
    pwd_lbl_box.append(&key_icon);

    let pwd_lbl = Label::new(Some(&trans("settings.sudo_password_placeholder")));
    pwd_lbl.add_css_class("settings-row-desc");
    pwd_lbl.set_valign(Align::Center);
    pwd_lbl.set_halign(Align::Start);
    pwd_lbl_box.append(&pwd_lbl);
    pwd_section.append(&pwd_lbl_box);

    let pwd_entry = PasswordEntry::new();
    pwd_entry.add_css_class("sidebar-search-entry");
    pwd_entry.add_css_class("modern-dialog-entry");
    pwd_entry.set_placeholder_text(Some(&trans("settings.sudo_password_placeholder")));
    pwd_section.append(&pwd_entry);

    // Error Label
    let error_lbl = Label::new(None);
    error_lbl.add_css_class("dialog-error-text");
    error_lbl.set_halign(Align::Start);
    error_lbl.set_visible(false);
    pwd_section.append(&error_lbl);
    auth_card.append(&pwd_section);

    // Safety Acknowledgment Row with wrapped CheckButton label
    let ack_card = Box::new(Orientation::Horizontal, 10);
    ack_card.add_css_class("recovery-ack-card");
    ack_card.set_valign(Align::Center);

    let understand_check = CheckButton::new();
    understand_check.set_valign(Align::Center);
    understand_check.set_cursor_from_name(Some("pointer"));
    understand_check.set_hexpand(true);

    let check_lbl = Label::new(Some(&trans("settings.recovery_confirm_understand")));
    check_lbl.set_wrap(true);
    check_lbl.set_halign(Align::Start);
    check_lbl.set_valign(Align::Center);
    check_lbl.set_hexpand(true);
    check_lbl.add_css_class("recovery-ack-label");
    understand_check.set_child(Some(&check_lbl));
    understand_check.set_active(false);
    ack_card.append(&understand_check);
    auth_card.append(&ack_card);

    // Action buttons
    let auth_actions = Box::new(Orientation::Horizontal, 10);
    auth_actions.set_halign(Align::End);
    auth_actions.set_valign(Align::Center);

    let cancel_btn = Button::with_label(&trans("common.cancel"));
    cancel_btn.add_css_class("modern-dialog-cancel-btn");
    cancel_btn.set_cursor_from_name(Some("pointer"));
    auth_actions.append(&cancel_btn);

    let confirm_btn = Button::with_label(&trans("settings.recovery_confirm_btn"));
    confirm_btn.add_css_class("modern-dialog-danger-btn");
    confirm_btn.set_sensitive(false);
    confirm_btn.set_cursor_from_name(Some("pointer"));
    auth_actions.append(&confirm_btn);

    auth_card.append(&auth_actions);
    auth_modal_overlay.append(&auth_card);
    root.add_overlay(&auth_modal_overlay);

    // ── Modal 2: Modern Terminal Console Dialog with Backdrop Scrim ──
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

    let console_card = Box::new(Orientation::Vertical, 14);
    console_card.add_css_class("modern-terminal-dialog");
    console_card.set_halign(Align::Center);
    console_card.set_valign(Align::Center);
    console_card.set_width_request(520);
    console_card.set_height_request(460);
    console_card.set_margin_start(16);
    console_card.set_margin_end(16);

    // Terminal Titlebar: Traffic light dots + Title + Live Status Badge
    let terminal_top_bar = Box::new(Orientation::Horizontal, 12);
    terminal_top_bar.add_css_class("terminal-title-bar");
    terminal_top_bar.set_valign(Align::Center);

    let traffic_dots = Box::new(Orientation::Horizontal, 6);
    traffic_dots.set_valign(Align::Center);

    let dot_close = Box::new(Orientation::Vertical, 0);
    dot_close.add_css_class("traffic-dot-close");

    let dot_min = Box::new(Orientation::Vertical, 0);
    dot_min.add_css_class("traffic-dot-minimize");

    let dot_max = Box::new(Orientation::Vertical, 0);
    dot_max.add_css_class("traffic-dot-maximize");

    traffic_dots.append(&dot_close);
    traffic_dots.append(&dot_min);
    traffic_dots.append(&dot_max);
    terminal_top_bar.append(&traffic_dots);

    let term_title_box = Box::new(Orientation::Horizontal, 8);
    term_title_box.set_hexpand(true);
    term_title_box.set_halign(Align::Center);

    let term_icon = babydra_ui_kit::ui::icon::get_icon("terminal", 16);
    term_icon.set_pixel_size(16);
    term_title_box.append(&term_icon);

    let console_title = Label::new(Some(&trans("settings.recovery_log_title")));
    console_title.add_css_class("terminal-window-title");
    term_title_box.append(&console_title);
    terminal_top_bar.append(&term_title_box);

    let status_badge = Label::new(Some(&trans("settings.recovery_badge_running")));
    status_badge.add_css_class("terminal-badge-running");
    status_badge.set_valign(Align::Center);
    terminal_top_bar.append(&status_badge);

    console_card.append(&terminal_top_bar);

    // Step Status & Modern Progress Bar
    let status_box = Box::new(Orientation::Vertical, 6);
    let status_lbl = Label::new(Some(&trans("settings.recovery_step_init")));
    status_lbl.add_css_class("terminal-step-text");
    status_lbl.set_halign(Align::Start);
    status_box.append(&status_lbl);

    let progress_bar = ProgressBar::new();
    progress_bar.add_css_class("modern-terminal-progress");
    progress_bar.set_fraction(0.05);
    status_box.append(&progress_bar);
    console_card.append(&status_box);

    // Terminal Log Monospace Area
    let terminal_wrapper = Box::new(Orientation::Vertical, 0);
    terminal_wrapper.add_css_class("terminal-console-window");
    terminal_wrapper.set_vexpand(true);

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_monospace(true);
    text_view.add_css_class("console-log-text");

    let console_scroll = ScrolledWindow::new();
    console_scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    console_scroll.set_vexpand(true);
    console_scroll.set_child(Some(&text_view));
    terminal_wrapper.append(&console_scroll);
    console_card.append(&terminal_wrapper);

    // Footer actions
    let console_actions = Box::new(Orientation::Horizontal, 10);
    console_actions.set_halign(Align::End);
    console_actions.set_margin_top(4);

    let close_btn = Button::with_label(&trans("settings.recovery_close_btn"));
    close_btn.add_css_class("connect-pill-btn");
    close_btn.add_css_class("modern-dialog-cancel-btn");
    close_btn.set_cursor_from_name(Some("pointer"));
    close_btn.set_visible(false);
    console_actions.append(&close_btn);

    let reboot_btn = Button::with_label(&trans("settings.recovery_reboot_btn"));
    reboot_btn.add_css_class("modern-reboot-btn");
    reboot_btn.set_cursor_from_name(Some("pointer"));
    reboot_btn.set_visible(false);
    console_actions.append(&reboot_btn);

    console_card.append(&console_actions);
    console_modal_overlay.append(&console_card);
    root.add_overlay(&console_modal_overlay);

    RecoveryWidgets {
        root,
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
