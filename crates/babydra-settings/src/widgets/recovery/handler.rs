//! Event handlers and background execution for Factory Reset.

use super::render::RecoveryWidgets;
use babydra_core::i18n::trans;
use babydra_core::services::system::reset::{reboot_system, run_factory_reset_stream};
use babydra_core::services::system::updates::validate_sudo;
use gtk4::prelude::*;
use std::sync::mpsc;

pub fn wire_events(widgets: &RecoveryWidgets) {
    // 1. Click "Start Factory Reset..." -> Show Auth Dialog
    let auth_card_show = widgets.auth_card.clone();
    let pwd_entry_show = widgets.pwd_entry.clone();
    let understand_check_show = widgets.understand_check.clone();
    let confirm_btn_show = widgets.confirm_btn.clone();
    let error_lbl_show = widgets.error_lbl.clone();
    let warn_all_apps_show = widgets.warn_all_apps_lbl.clone();
    let remove_all_apps_show = widgets.remove_all_apps_check.clone();

    widgets.start_btn.connect_clicked(move |_| {
        pwd_entry_show.set_text("");
        understand_check_show.set_active(false);
        confirm_btn_show.set_sensitive(false);
        error_lbl_show.set_visible(false);
        warn_all_apps_show.set_visible(remove_all_apps_show.is_active());
        auth_card_show.set_visible(true);
        pwd_entry_show.grab_focus();
    });

    // 1.1 Toggle "Remove all apps" checkbox -> sync warning and shell packages checkbox
    let warn_all_apps_toggle = widgets.warn_all_apps_lbl.clone();
    let remove_pkgs_toggle = widgets.remove_pkgs_check.clone();
    widgets.remove_all_apps_check.connect_toggled(move |btn| {
        let is_all = btn.is_active();
        warn_all_apps_toggle.set_visible(is_all);
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

    // 3. Cancel Button
    let auth_card_cancel = widgets.auth_card.clone();
    let pwd_entry_cancel = widgets.pwd_entry.clone();
    widgets.cancel_btn.connect_clicked(move |_| {
        pwd_entry_cancel.set_text("");
        auth_card_cancel.set_visible(false);
    });

    // 4. Confirm Reset Clicked -> Verify Sudo Password & Run Stream
    let auth_card_confirm = widgets.auth_card.clone();
    let pwd_entry_confirm = widgets.pwd_entry.clone();
    let error_lbl_confirm = widgets.error_lbl.clone();
    let console_card_confirm = widgets.console_card.clone();
    let status_lbl_confirm = widgets.status_lbl.clone();
    let progress_bar_confirm = widgets.progress_bar.clone();
    let text_view_confirm = widgets.text_view.clone();
    let close_btn_confirm = widgets.close_btn.clone();
    let reboot_btn_confirm = widgets.reboot_btn.clone();
    let remove_pkgs_check = widgets.remove_pkgs_check.clone();
    let remove_all_apps_check = widgets.remove_all_apps_check.clone();

    widgets.confirm_btn.connect_clicked(move |_| {
        let password = pwd_entry_confirm.text().trim().to_string();
        if password.is_empty() {
            error_lbl_confirm.set_text(&trans("settings.password_empty"));
            error_lbl_confirm.set_visible(true);
            return;
        }

        // Validate password
        if !validate_sudo(&password) {
            error_lbl_confirm.set_text(&trans("common.password_incorrect"));
            error_lbl_confirm.set_visible(true);
            return;
        }

        // Hide auth modal and show console log modal
        error_lbl_confirm.set_visible(false);
        auth_card_confirm.set_visible(false);

        console_card_confirm.set_visible(true);
        status_lbl_confirm.set_text(&trans("settings.recovery_status_running"));
        progress_bar_confirm.set_fraction(0.05);
        close_btn_confirm.set_visible(false);
        reboot_btn_confirm.set_visible(false);

        let buffer = text_view_confirm.buffer();
        buffer.set_text("");

        let remove_pkgs = remove_pkgs_check.is_active();
        let remove_all_apps = remove_all_apps_check.is_active();
        let (tx, rx) = mpsc::channel::<String>();

        let pwd_thread = password.clone();
        std::thread::spawn(move || {
            let _ = run_factory_reset_stream(&pwd_thread, remove_pkgs, remove_all_apps, tx);
        });

        let status_lbl_poller = status_lbl_confirm.clone();
        let progress_bar_poller = progress_bar_confirm.clone();
        let text_view_poller = text_view_confirm.clone();
        let close_btn_poller = close_btn_confirm.clone();
        let reboot_btn_poller = reboot_btn_confirm.clone();

        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            let mut keep_going = true;

            while let Ok(line) = rx.try_recv() {
                let buf = text_view_poller.buffer();
                let mut end_iter = buf.end_iter();
                buf.insert(&mut end_iter, &format!("{}\n", line));

                // Auto scroll to bottom
                let mark = buf.create_mark(None, &end_iter, false);
                text_view_poller.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);

                // Update progress fraction heuristically
                if line.contains("[1/6]") {
                    progress_bar_poller.set_fraction(0.15);
                } else if line.contains("[2/6]") {
                    progress_bar_poller.set_fraction(0.35);
                } else if line.contains("[3/6]") {
                    progress_bar_poller.set_fraction(0.50);
                } else if line.contains("[4/6]") {
                    progress_bar_poller.set_fraction(0.70);
                } else if line.contains("[5/6]") {
                    progress_bar_poller.set_fraction(0.85);
                } else if line.contains("[6/6]") {
                    progress_bar_poller.set_fraction(0.95);
                } else if line.contains("Factory Reset Complete") {
                    progress_bar_poller.set_fraction(1.0);
                    status_lbl_poller.set_text(&trans("settings.recovery_status_done"));
                    close_btn_poller.set_visible(true);
                    reboot_btn_poller.set_visible(true);
                    keep_going = false;
                    break;
                } else if line.contains("[Error]") {
                    status_lbl_poller.set_text(&trans("settings.recovery_status_failed"));
                    close_btn_poller.set_visible(true);
                    keep_going = false;
                    break;
                }
            }

            if keep_going {
                gtk4::glib::ControlFlow::Continue
            } else {
                gtk4::glib::ControlFlow::Break
            }
        });
    });

    // 5. Close Button inside Console Log Dialog
    let console_card_close = widgets.console_card.clone();
    widgets.close_btn.connect_clicked(move |_| {
        console_card_close.set_visible(false);
    });

    // 6. Reboot Button inside Console Log Dialog
    widgets.reboot_btn.connect_clicked(move |_| {
        let _ = reboot_system(None);
    });
}
