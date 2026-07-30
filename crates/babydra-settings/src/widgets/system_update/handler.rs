use gtk4::prelude::*;
use babydra_common::models::system_update::PackageUpdate;
use babydra_common::models::system_update::SystemUpdateWidget;
use babydra_utils::components::modal::PasswordDialog;
use super::render;
use babydra_common::services::system::updates::{
    is_pacman_running, read_update_log, start_background_update,
};

/// Parses current and total steps from log output lines (e.g. "(15/104)")
fn parse_latest_progress(log_text: &str) -> Option<(u32, u32)> {
    for line in log_text.lines().rev() {
        let line_trimmed = line.trim();
        if let Some(start) = line_trimmed.find('(') {
            if let Some(end) = line_trimmed[start..].find(')') {
                let inner = &line_trimmed[start + 1..start + end];
                let parts: Vec<&str> = inner.split('/').collect();
                if parts.len() == 2 {
                    if let (Ok(curr), Ok(total)) = (parts[0].trim().parse::<u32>(), parts[1].trim().parse::<u32>()) {
                        if total > 0 && curr <= total {
                            return Some((curr, total));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Applies linear-gradient outline progress style and updates button label
fn update_btn_progress(btn: &gtk4::Button, provider: &gtk4::CssProvider, label_text: &str, pct: f64) {
    btn.set_label(label_text);
    let css = format!(
        ".suggested-action {{ background-image: linear-gradient(to right, #3b82f6 0%, #3b82f6 {:.1}%, rgba(255, 255, 255, 0.12) {:.1}%); border: 1px solid #3b82f6; color: #ffffff; font-weight: 700; border-radius: 9999px; }}",
        pct, pct
    );
    provider.load_from_data(&css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

/// Resets button style back to default suggested action
fn reset_btn_progress(btn: &gtk4::Button, provider: &gtk4::CssProvider) {
    btn.set_label(&babydra_common::i18n::t("settings.update_all"));
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_remove_provider_for_display(&display, provider);
    }
}

pub fn wire_events(widget: &SystemUpdateWidget, auth_dialog: PasswordDialog) {
    let list_box = widget.list_box.clone();
    let count_badge = widget.count_badge.clone();
    let spinner = widget.spinner.clone();
    let refresh_btn = widget.refresh_btn.clone();
    let update_all_btn = widget.update_all_btn.clone();
    let btn_provider = gtk4::CssProvider::new();

    // Helper closure to trigger async update check
    let trigger_check = {
        let list_box = list_box.clone();
        let count_badge = count_badge.clone();
        let spinner = spinner.clone();
        let refresh_btn = refresh_btn.clone();
        let update_all_btn = update_all_btn.clone();

        move || {
            spinner.set_visible(true);
            spinner.start();
            refresh_btn.set_sensitive(false);

            let list_box = list_box.clone();
            let count_badge = count_badge.clone();
            let spinner = spinner.clone();
            let refresh_btn = refresh_btn.clone();
            let update_all_btn = update_all_btn.clone();

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
                        update_all_btn.set_visible(false);
                        refresh_btn.set_visible(true);
                    } else {
                        for pkg in &updates {
                            list_box.append(&render::create_update_row(pkg));
                        }
                        update_all_btn.set_visible(true);
                        refresh_btn.set_visible(false);
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

    // Helper to start watching background log stream in UI and updating button progress
    let text_buffer = widget.text_buffer.clone();
    let console_scroll = widget.console_scroll.clone();
    let update_all_btn_stream = widget.update_all_btn.clone();
    let trigger_check_finish = trigger_check.clone();
    let btn_provider_watcher = btn_provider.clone();

    let start_log_stream_watcher = move || {
        let text_buffer_c = text_buffer.clone();
        let console_scroll_c = console_scroll.clone();
        let update_all_btn_c = update_all_btn_stream.clone();
        let trigger_check_c = trigger_check_finish.clone();
        let btn_provider_c = btn_provider_watcher.clone();

        glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            let log_text = read_update_log();
            text_buffer_c.set_text(&log_text);

            let adj = console_scroll_c.vadjustment();
            adj.set_value(adj.upper() - adj.page_size());

            if let Some((curr, total)) = parse_latest_progress(&log_text) {
                let pct = (curr as f64 / total as f64 * 100.0).clamp(0.0, 100.0);
                update_btn_progress(&update_all_btn_c, &btn_provider_c, &format!("{}/{}", curr, total), pct);
            } else {
                update_btn_progress(&update_all_btn_c, &btn_provider_c, &babydra_common::i18n::t("settings.update_all"), 0.0);
            }

            let in_progress = is_pacman_running();
            if !in_progress {
                reset_btn_progress(&update_all_btn_c, &btn_provider_c);
                trigger_check_c();
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    };

    // Check if an update is already running in background (e.g. from previous run / app restart)
    if is_pacman_running() {
        widget.update_all_btn.set_visible(true);
        widget.refresh_btn.set_visible(false);
        widget.glass_card.set_visible(false);
        widget.console_card.set_visible(true);
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
        if is_pacman_running() {
            return;
        }
        auth_dialog_show.show_for("Authentication Required", "Enter sudo password to apply system updates:");
    });

    // Handle Confirm & Start -> Run update in background with persistent log & outline progress
    let glass_card = widget.glass_card.clone();
    let console_card = widget.console_card.clone();
    let text_buffer = widget.text_buffer.clone();
    let update_all_btn = widget.update_all_btn.clone();
    let btn_provider_start = btn_provider.clone();

    auth_dialog_rc.connect_submit(move |password| {
        glass_card.set_visible(false);
        console_card.set_visible(true);
        text_buffer.set_text("");
        update_btn_progress(&update_all_btn, &btn_provider_start, &babydra_common::i18n::t("settings.update_all"), 0.0);

        start_background_update(password);
        start_log_stream_watcher();
    });
}
