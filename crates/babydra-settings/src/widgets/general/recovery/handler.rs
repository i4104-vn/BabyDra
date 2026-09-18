//! Event handlers and background execution for Factory Reset card.

use super::render::RecoveryCardWidgets;
use babydra_core::i18n::trans;
use babydra_core::services::system::reset::{reboot_system, run_factory_reset_stream};
use babydra_core::services::system::updates::validate_sudo;
use gtk4::prelude::*;
use std::sync::mpsc;

pub fn wire_events(widgets: &RecoveryCardWidgets) {
    // 1. Click "Start Factory Reset..." -> Show Auth Dialog Overlay
    let auth_overlay_show = widgets.auth_modal_overlay.clone();
    let pwd_entry_show = widgets.pwd_entry.clone();
    let understand_check_show = widgets.understand_check.clone();
    let confirm_btn_show = widgets.confirm_btn.clone();
    let error_lbl_show = widgets.error_lbl.clone();
    let warn_all_apps_box_show = widgets.warn_all_apps_box.clone();
    let remove_all_apps_show = widgets.remove_all_apps_check.clone();

    widgets.start_btn.connect_clicked(move |_| {
        pwd_entry_show.set_text("");
        understand_check_show.set_active(false);
        confirm_btn_show.set_sensitive(false);
        error_lbl_show.set_visible(false);
        warn_all_apps_box_show.set_visible(remove_all_apps_show.is_active());
        auth_overlay_show.set_visible(true);
        pwd_entry_show.grab_focus();
    });

    // 1.1 Toggle "Remove all apps" checkbox -> sync warning callout and shell packages checkbox
    let warn_all_apps_box_toggle = widgets.warn_all_apps_box.clone();
    let remove_pkgs_toggle = widgets.remove_pkgs_check.clone();
    widgets.remove_all_apps_check.connect_toggled(move |btn| {
        let is_all = btn.is_active();
        warn_all_apps_box_toggle.set_visible(is_all);
        if is_all {
            remove_pkgs_toggle.set_active(true);
            remove_pkgs_toggle.set_sensitive(false);
        } else {
            remove_pkgs_toggle.set_sensitive(true);
        }
    });

    // 2. Validate input conditions to enable "Confirm Reset" button
    let understand_check_sync = widgets.understand_check.clone();
    let pwd_entry_sync = widgets.pwd_entry.clone();
    let confirm_btn_sync = widgets.confirm_btn.clone();

    let check_input_validity = move || {
        let has_pwd = !pwd_entry_sync.text().trim().is_empty();
        let confirmed = understand_check_sync.is_active();
        confirm_btn_sync.set_sensitive(has_pwd && confirmed);
    };

    let civ1 = check_input_validity.clone();
    widgets.understand_check.connect_toggled(move |_| {
        civ1();
    });

    let civ2 = check_input_validity;
    widgets.pwd_entry.connect_changed(move |_| {
        civ2();
    });

    // 3. Click "Cancel" in Auth Modal
    let auth_overlay_hide = widgets.auth_modal_overlay.clone();
    widgets.cancel_btn.connect_clicked(move |_| {
        auth_overlay_hide.set_visible(false);
    });

    // 4. Click "Confirm Reset" -> Verify Sudo, then start streaming reset process
    let auth_overlay_proc = widgets.auth_modal_overlay.clone();
    let console_overlay_proc = widgets.console_modal_overlay.clone();
    let pwd_entry_proc = widgets.pwd_entry.clone();
    let error_lbl_proc = widgets.error_lbl.clone();
    let confirm_btn_proc = widgets.confirm_btn.clone();
    let remove_pkgs_proc = widgets.remove_pkgs_check.clone();
    let remove_all_apps_proc = widgets.remove_all_apps_check.clone();

    let text_view_proc = widgets.text_view.clone();
    let progress_bar_proc = widgets.progress_bar.clone();
    let status_lbl_proc = widgets.status_lbl.clone();
    let status_badge_proc = widgets.status_badge.clone();
    let close_btn_proc = widgets.close_btn.clone();
    let reboot_btn_proc = widgets.reboot_btn.clone();

    widgets.confirm_btn.connect_clicked(move |_| {
        let password = pwd_entry_proc.text().to_string();
        confirm_btn_proc.set_sensitive(false);
        error_lbl_proc.set_visible(false);

        // Validate sudo credentials
        let pwd_val = password.clone();
        let is_valid = std::thread::spawn(move || validate_sudo(&pwd_val))
            .join()
            .unwrap_or(false);

        if !is_valid {
            error_lbl_proc.set_text(&trans("settings.recovery_pwd_incorrect"));
            error_lbl_proc.set_visible(true);
            confirm_btn_proc.set_sensitive(true);
            return;
        }

        // Hide auth modal, show console modal
        auth_overlay_proc.set_visible(false);
        console_overlay_proc.set_visible(true);

        let remove_pkgs = remove_pkgs_proc.is_active();
        let remove_all = remove_all_apps_proc.is_active();

        // Reset console text view
        let buffer = text_view_proc.buffer();
        buffer.set_text("");

        progress_bar_proc.set_fraction(0.1);
        status_lbl_proc.set_text(&trans("settings.recovery_status_running"));
        status_badge_proc.set_text(&trans("settings.recovery_badge_running"));
        status_badge_proc.remove_css_class("status-success-badge");
        status_badge_proc.remove_css_class("status-error-badge");
        status_badge_proc.add_css_class("update-count-badge");

        close_btn_proc.set_visible(false);
        reboot_btn_proc.set_visible(false);

        // Spawn channel for background reset streaming
        let (tx, rx) = mpsc::channel::<String>();

        std::thread::spawn(move || {
            let _ = run_factory_reset_stream(&password, remove_pkgs, remove_all, tx);
        });

        // Handle streaming updates on UI thread
        let tv_c = text_view_proc.clone();
        let pb_c = progress_bar_proc.clone();
        let sl_c = status_lbl_proc.clone();
        let sb_c = status_badge_proc.clone();
        let close_c = close_btn_proc.clone();
        let reboot_c = reboot_btn_proc.clone();

        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            let mut keep_going = true;

            while let Ok(msg) = rx.try_recv() {
                let buffer = tv_c.buffer();
                let mut end_iter = buffer.end_iter();
                buffer.insert(&mut end_iter, &format!("{}\n", msg));

                // Auto-scroll to bottom
                let mark = buffer.create_mark(None, &buffer.end_iter(), false);
                tv_c.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);

                // Update progress bar based on milestone strings
                if msg.contains("Resetting desktop configurations") {
                    pb_c.set_fraction(0.25);
                    sl_c.set_text(&trans("settings.recovery_step_config"));
                } else if msg.contains("Clearing runtime caches") {
                    pb_c.set_fraction(0.40);
                    sl_c.set_text(&trans("settings.recovery_step_cache"));
                } else if msg.contains("Restoring default configurations") {
                    pb_c.set_fraction(0.60);
                    sl_c.set_text(&trans("settings.recovery_step_defaults"));
                } else if msg.contains("Removing installed packages") {
                    pb_c.set_fraction(0.80);
                    sl_c.set_text(&trans("settings.recovery_step_pkgs"));
                } else if msg.contains("[DONE]") {
                    pb_c.set_fraction(1.0);
                    sl_c.set_text(&trans("settings.recovery_status_done"));
                    sb_c.set_text(&trans("settings.recovery_badge_done"));
                    sb_c.remove_css_class("update-count-badge");
                    sb_c.add_css_class("status-success-badge");
                    close_c.set_visible(true);
                    reboot_c.set_visible(true);
                    keep_going = false;
                    break;
                } else if msg.contains("[ERROR]") {
                    pb_c.set_fraction(1.0);
                    sl_c.set_text(&trans("settings.recovery_status_error"));
                    sb_c.set_text(&trans("settings.recovery_badge_error"));
                    sb_c.remove_css_class("update-count-badge");
                    sb_c.add_css_class("status-error-badge");
                    close_c.set_visible(true);
                    keep_going = false;
                    break;
                }
            }

            if keep_going {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });
    });

    // 5. Console modal action buttons
    let console_overlay_close = widgets.console_modal_overlay.clone();
    widgets.close_btn.connect_clicked(move |_| {
        console_overlay_close.set_visible(false);
    });

    widgets.reboot_btn.connect_clicked(move |_| {
        let _ = reboot_system(None);
    });
}
