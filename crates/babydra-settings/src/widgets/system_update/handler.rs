use gtk4::prelude::*;
use babydra_common::models::system_update::PackageUpdate;
use babydra_common::models::system_update::SystemUpdateWidget;
use babydra_utils::components::modal::PasswordDialog;
use super::render;
use babydra_common::services::system::updates::{
    is_update_in_progress, read_update_log, start_background_update,
};

pub fn wire_events(widget: &SystemUpdateWidget, auth_dialog: PasswordDialog) {
    let list_box = widget.list_box.clone();
    let count_badge = widget.count_badge.clone();
    let spinner = widget.spinner.clone();
    let refresh_btn = widget.refresh_btn.clone();

    // Helper closure to trigger async update check
    let trigger_check = {
        let list_box = list_box.clone();
        let count_badge = count_badge.clone();
        let spinner = spinner.clone();
        let refresh_btn = refresh_btn.clone();

        move || {
            spinner.set_visible(true);
            spinner.start();
            refresh_btn.set_sensitive(false);

            let list_box = list_box.clone();
            let count_badge = count_badge.clone();
            let spinner = spinner.clone();
            let refresh_btn = refresh_btn.clone();

            let (tx, rx) = std::sync::mpsc::channel::<Vec<PackageUpdate>>();

            std::thread::spawn(move || {
                let updates = babydra_common::services::system::updates::check_updates().unwrap_or_default();
                let _ = tx.send(updates);
            });

            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if let Ok(updates) = rx.try_recv() {
                    let count_text = if updates.is_empty() {
                        babydra_common::i18n::t("settings.up_to_date")
                    } else {
                        format!("{} {}", updates.len(), babydra_common::i18n::t("settings.updates_available"))
                    };
                    count_badge.set_text(&count_text);

                    while let Some(child) = list_box.first_child() {
                        list_box.remove(&child);
                    }

                    if updates.is_empty() {
                        list_box.append(&render::create_empty_up_to_date_row());
                    } else {
                        for pkg in &updates {
                            list_box.append(&render::create_update_row(pkg));
                        }
                    }

                    spinner.stop();
                    spinner.set_visible(false);
                    refresh_btn.set_sensitive(true);

                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        }
    };

    // Helper to start watching background log stream in UI
    let text_buffer = widget.text_buffer.clone();
    let console_scroll = widget.console_scroll.clone();
    let update_all_btn = widget.update_all_btn.clone();
    let trigger_check_finish = trigger_check.clone();

    let start_log_stream_watcher = move || {
        let text_buffer_c = text_buffer.clone();
        let console_scroll_c = console_scroll.clone();
        let update_all_btn_c = update_all_btn.clone();
        let trigger_check_c = trigger_check_finish.clone();

        glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            let log_text = read_update_log();
            text_buffer_c.set_text(&log_text);

            let adj = console_scroll_c.vadjustment();
            adj.set_value(adj.upper() - adj.page_size());

            let in_progress = is_update_in_progress();
            if !in_progress {
                update_all_btn_c.set_sensitive(true);
                trigger_check_c();
                glib::ControlFlow::Break
            } else {
                update_all_btn_c.set_sensitive(false);
                glib::ControlFlow::Continue
            }
        });
    };

    // Check if an update is already running in background (e.g. from previous run / app restart)
    if is_update_in_progress() {
        widget.glass_card.set_visible(false);
        widget.console_card.set_visible(true);
        widget.update_all_btn.set_sensitive(false);
        let log_text = read_update_log();
        widget.text_buffer.set_text(&log_text);
        start_log_stream_watcher();
    } else {
        // Auto-trigger check in background after window presentation
        let auto_check = trigger_check.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
            auto_check();
            glib::ControlFlow::Break
        });
    }

    let trigger_check_btn = trigger_check.clone();
    widget.refresh_btn.connect_clicked(move |_| {
        trigger_check_btn();
    });

    // Handle Update All -> Show reusable PasswordDialog
    let auth_dialog_rc = std::rc::Rc::new(auth_dialog);
    let auth_dialog_show = auth_dialog_rc.clone();
    widget.update_all_btn.connect_clicked(move |_| {
        auth_dialog_show.show_for("Authentication Required", "Enter sudo password to apply system updates:");
    });

    // Handle Confirm & Start -> Run update in background with persistent log
    let glass_card = widget.glass_card.clone();
    let console_card = widget.console_card.clone();
    let text_buffer = widget.text_buffer.clone();
    let update_all_btn = widget.update_all_btn.clone();

    auth_dialog_rc.connect_submit(move |password| {
        glass_card.set_visible(false);
        console_card.set_visible(true);
        update_all_btn.set_sensitive(false);
        text_buffer.set_text("");

        start_background_update(password);
        start_log_stream_watcher();
    });
}
