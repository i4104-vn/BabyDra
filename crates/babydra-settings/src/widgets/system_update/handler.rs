use gtk4::prelude::*;
use babydra_common::models::system_update::PackageUpdate;
use babydra_common::models::system_update::SystemUpdateWidget;
use babydra_utils::components::modal::PasswordDialog;
use super::render;

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

    // Auto-trigger check in background after window presentation
    let auto_check = trigger_check.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
        auto_check();
        glib::ControlFlow::Break
    });

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

    // Handle Console Close Button
    let console_card_close = widget.console_card.clone();
    let glass_card_show = widget.glass_card.clone();
    let update_all_btn_close = widget.update_all_btn.clone();
    widget.console_close_btn.connect_clicked(move |_| {
        console_card_close.set_visible(false);
        glass_card_show.set_visible(true);
        update_all_btn_close.set_sensitive(true);
    });

    // Handle Confirm & Start -> Run update with streaming console output
    let glass_card = widget.glass_card.clone();
    let console_card = widget.console_card.clone();
    let text_buffer = widget.text_buffer.clone();
    let console_scroll = widget.console_scroll.clone();
    let trigger_check_after = trigger_check.clone();
    let update_all_btn = widget.update_all_btn.clone();

    auth_dialog_rc.connect_submit(move |password| {
        glass_card.set_visible(false);
        console_card.set_visible(true);
        update_all_btn.set_sensitive(false);

        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let pwd_clone = password.clone();

        std::thread::spawn(move || {
            let res = babydra_common::services::system::updates::stream_update_system(pwd_clone.as_deref(), tx.clone());
            if let Err(e) = res {
                let _ = tx.send(format!("\nError: {}", e));
            } else {
                let _ = tx.send("\nSystem update completed successfully.".to_string());
            }
        });

        let text_buffer_c = text_buffer.clone();
        let console_scroll_c = console_scroll.clone();
        let trigger_check_c = trigger_check_after.clone();
        let update_all_btn_c = update_all_btn.clone();

        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            loop {
                match rx.try_recv() {
                    Ok(line) => {
                        let mut iter = text_buffer_c.end_iter();
                        text_buffer_c.insert(&mut iter, &format!("{}\n", line));

                        let adj = console_scroll_c.vadjustment();
                        adj.set_value(adj.upper() - adj.page_size());

                        if line.contains("System update completed successfully") || line.contains("Error:") {
                            update_all_btn_c.set_sensitive(true);
                            trigger_check_c();
                            return glib::ControlFlow::Break;
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        return glib::ControlFlow::Continue;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        update_all_btn_c.set_sensitive(true);
                        trigger_check_c();
                        return glib::ControlFlow::Break;
                    }
                }
            }
        });
    });
}
