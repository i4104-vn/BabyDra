use super::render;
use babydra_core::models::system_update::{PackageUpdate, SystemUpdateWidget, UpdateStatus};
use babydra_core::services::system::updates::{
    check_updates, clear_update_state, is_pacman_running, load_update_state, save_update_state,
};
use babydra_ui_kit::components::modal::PasswordDialog;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::io::Write;
use std::process::{Command, Stdio};
use std::rc::Rc;

/// Status rank.
fn status_rank(status: &UpdateStatus) -> u8 {
    match status {
        UpdateStatus::Done => 0,
        UpdateStatus::Updating => 1,
        UpdateStatus::Failed => 2,
        UpdateStatus::Pending => 3,
    }
}

/// Wire events.
pub fn wire_events(widget: &SystemUpdateWidget, auth_dialog: PasswordDialog) {
    let list_box = widget.list_box.clone();
    let count_badge = widget.count_badge.clone();
    let spinner = widget.spinner.clone();
    let refresh_btn = widget.refresh_btn.clone();
    let update_all_btn = widget.update_all_btn.clone();
    let progress_bar = widget.progress_bar.clone();
    let status_label = widget.status_label.clone();
    let progress_box = progress_bar
        .parent()
        .and_then(|p| p.downcast::<gtk4::Box>().ok());

    let current_updates: Rc<RefCell<Vec<PackageUpdate>>> = Rc::new(RefCell::new(Vec::new()));
    let is_updating = Rc::new(RefCell::new(false));

    // Helper closure to render current package updates into ListBox (Done packages on top)
    let render_packages = {
        let list_box = list_box.clone();
        let current_updates = current_updates.clone();
        move || {
            while let Some(child) = list_box.first_child() {
                list_box.remove(&child);
            }

            let mut pkgs = current_updates.borrow().clone();
            if pkgs.is_empty() {
                list_box.append(&render::create_empty_up_to_date_row());
            } else {
                pkgs.sort_by_key(|p| status_rank(&p.status));
                for pkg in pkgs.iter() {
                    list_box.append(&render::create_update_row(pkg));
                }
            }
        }
    };

    // Helper closure to start live polling of update_state file
    let start_state_poller = {
        let current_updates = current_updates.clone();
        let render_packages = render_packages.clone();
        let progress_bar = progress_bar.clone();
        let status_label = status_label.clone();
        let count_badge = count_badge.clone();
        let update_all_btn = update_all_btn.clone();
        let refresh_btn = refresh_btn.clone();
        let is_updating = is_updating.clone();

        move || {
            let current_updates_poller = current_updates.clone();
            let render_packages_poller = render_packages.clone();
            let progress_bar_poller = progress_bar.clone();
            let status_label_poller = status_label.clone();
            let count_badge_poller = count_badge.clone();
            let update_all_btn_poller = update_all_btn.clone();
            let refresh_btn_poller = refresh_btn.clone();
            let is_updating_poller = is_updating.clone();

            glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
                if let Some(state) = load_update_state() {
                    let packages_changed = {
                        let current = current_updates_poller.borrow();
                        if current.len() != state.packages.len() {
                            true
                        } else {
                            current
                                .iter()
                                .zip(state.packages.iter())
                                .any(|(a, b)| a.status != b.status)
                        }
                    };

                    if packages_changed {
                        *current_updates_poller.borrow_mut() = state.packages.clone();
                        render_packages_poller();
                    }

                    let total = state.packages.len();
                    let completed = state
                        .packages
                        .iter()
                        .filter(|p| {
                            p.status == UpdateStatus::Done || p.status == UpdateStatus::Failed
                        })
                        .count();

                    let fraction = if total > 0 {
                        completed as f64 / total as f64
                    } else {
                        1.0
                    };
                    progress_bar_poller.set_fraction(fraction);

                    if state.is_updating && completed < total {
                        if state.is_syncing && completed == 0 {
                            status_label_poller
                                .set_text(&babydra_core::i18n::t("settings.update_syncing"));
                            progress_bar_poller.set_fraction(0.0);
                        } else {
                            let prog_text = babydra_core::i18n::t("settings.update_progress")
                                .replace("{current}", &completed.to_string())
                                .replace("{total}", &total.to_string());
                            status_label_poller.set_text(&prog_text);
                        }
                        update_all_btn_poller.set_sensitive(false);
                        update_all_btn_poller.add_css_class("disabled");
                        refresh_btn_poller.set_sensitive(false);
                        refresh_btn_poller.add_css_class("disabled");
                        glib::ControlFlow::Continue
                    } else {
                        let failed_count = state
                            .packages
                            .iter()
                            .filter(|p| p.status == UpdateStatus::Failed)
                            .count();
                        if failed_count == 0 {
                            status_label_poller
                                .set_text(&babydra_core::i18n::t("settings.update_complete"));
                            count_badge_poller
                                .set_text(&babydra_core::i18n::t("settings.up_to_date"));
                            update_all_btn_poller.set_visible(false);
                            refresh_btn_poller.set_visible(true);
                        } else {
                            let fail_text = babydra_core::i18n::t("settings.update_failed")
                                .replace("{count}", &failed_count.to_string());
                            status_label_poller.set_text(&fail_text);
                            update_all_btn_poller.set_visible(true);
                            refresh_btn_poller.set_visible(false);
                        }
                        progress_bar_poller.set_fraction(1.0);
                        *is_updating_poller.borrow_mut() = false;
                        update_all_btn_poller.set_sensitive(true);
                        update_all_btn_poller.remove_css_class("disabled");
                        refresh_btn_poller.set_sensitive(true);
                        refresh_btn_poller.remove_css_class("disabled");
                        glib::ControlFlow::Break
                    }
                } else {
                    glib::ControlFlow::Continue
                }
            });
        }
    };

    // Helper closure to trigger async update check (clears saved state)
    let trigger_check = {
        let list_box_c = list_box.clone();
        let count_badge_c = count_badge.clone();
        let spinner_c = spinner.clone();
        let refresh_btn_c = refresh_btn.clone();
        let update_all_btn_c = update_all_btn.clone();
        let current_updates_c = current_updates.clone();
        let progress_box_c = progress_box.clone();

        move || {
            clear_update_state();
            spinner_c.set_visible(true);
            spinner_c.start();
            refresh_btn_c.set_sensitive(false);

            if let Some(ref pbox) = progress_box_c {
                pbox.set_visible(false);
            }

            let list_box_sub = list_box_c.clone();
            let count_badge_sub = count_badge_c.clone();
            let spinner_sub = spinner_c.clone();
            let refresh_btn_sub = refresh_btn_c.clone();
            let update_all_btn_sub = update_all_btn_c.clone();
            let current_updates_sub = current_updates_c.clone();

            let (tx, rx) = std::sync::mpsc::channel::<Vec<PackageUpdate>>();

            std::thread::spawn(move || {
                let updates = check_updates().unwrap_or_default();
                let _ = tx.send(updates);
            });

            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if let Ok(updates) = rx.try_recv() {
                    let count_text = if updates.is_empty() {
                        babydra_core::i18n::t("settings.up_to_date")
                    } else {
                        format!(
                            "{} {}",
                            updates.len(),
                            babydra_core::i18n::t("settings.updates_available")
                        )
                    };
                    count_badge_sub.set_text(&count_text);
                    *current_updates_sub.borrow_mut() = updates.clone();

                    while let Some(child) = list_box_sub.first_child() {
                        list_box_sub.remove(&child);
                    }

                    if updates.is_empty() {
                        list_box_sub.append(&render::create_empty_up_to_date_row());
                        update_all_btn_sub.set_visible(false);
                        refresh_btn_sub.set_visible(true);
                    } else {
                        for pkg in &updates {
                            list_box_sub.append(&render::create_update_row(pkg));
                        }
                        update_all_btn_sub.set_visible(true);
                        refresh_btn_sub.set_visible(false);
                    }

                    spinner_sub.stop();
                    spinner_sub.set_visible(false);
                    refresh_btn_sub.set_sensitive(true);

                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        }
    };

    // Auto check or restore saved state on presentation
    let count_badge_init = count_badge.clone();
    let update_all_btn_init = update_all_btn.clone();
    let refresh_btn_init = refresh_btn.clone();
    let progress_bar_init = progress_bar.clone();
    let status_label_init = status_label.clone();
    let progress_box_init = progress_box.clone();
    let current_updates_init = current_updates.clone();
    let is_updating_init = is_updating.clone();
    let render_packages_init = render_packages.clone();
    let auto_check = trigger_check.clone();
    let start_poller_init = start_state_poller.clone();

    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        if let Some(saved_state) = load_update_state() {
            if !saved_state.packages.is_empty() {
                *current_updates_init.borrow_mut() = saved_state.packages.clone();
                render_packages_init();

                let total = saved_state.packages.len();
                let completed = saved_state
                    .packages
                    .iter()
                    .filter(|p| p.status == UpdateStatus::Done || p.status == UpdateStatus::Failed)
                    .count();

                let count_text = format!(
                    "{} {}",
                    total,
                    babydra_core::i18n::t("settings.updates_available")
                );
                count_badge_init.set_text(&count_text);
                update_all_btn_init.set_visible(true);
                refresh_btn_init.set_visible(false);

                if let Some(ref pbox) = progress_box_init {
                    pbox.set_visible(true);
                }
                let fraction = if total > 0 {
                    completed as f64 / total as f64
                } else {
                    1.0
                };
                progress_bar_init.set_fraction(fraction);

                if saved_state.is_updating && completed < total {
                    *is_updating_init.borrow_mut() = true;
                    update_all_btn_init.set_sensitive(false);
                    update_all_btn_init.add_css_class("disabled");
                    refresh_btn_init.set_sensitive(false);
                    refresh_btn_init.add_css_class("disabled");

                    if saved_state.is_syncing && completed == 0 {
                        status_label_init
                            .set_text(&babydra_core::i18n::t("settings.update_syncing"));
                        progress_bar_init.set_fraction(0.0);
                    } else {
                        let prog_text = babydra_core::i18n::t("settings.update_progress")
                            .replace("{current}", &completed.to_string())
                            .replace("{total}", &total.to_string());
                        status_label_init.set_text(&prog_text);
                    }

                    start_poller_init();
                } else {
                    let failed_count = saved_state
                        .packages
                        .iter()
                        .filter(|p| p.status == UpdateStatus::Failed)
                        .count();
                    if failed_count == 0 {
                        status_label_init
                            .set_text(&babydra_core::i18n::t("settings.update_complete"));
                        count_badge_init.set_text(&babydra_core::i18n::t("settings.up_to_date"));
                        update_all_btn_init.set_visible(false);
                        refresh_btn_init.set_visible(true);
                    } else {
                        let fail_text = babydra_core::i18n::t("settings.update_failed")
                            .replace("{count}", &failed_count.to_string());
                        status_label_init.set_text(&fail_text);
                        update_all_btn_init.set_visible(true);
                        refresh_btn_init.set_visible(false);
                    }
                    update_all_btn_init.set_sensitive(true);
                    update_all_btn_init.remove_css_class("disabled");
                    refresh_btn_init.set_sensitive(true);
                    refresh_btn_init.remove_css_class("disabled");
                }
                return glib::ControlFlow::Break;
            }
        }

        auto_check();
        glib::ControlFlow::Break
    });

    let trigger_check_btn = trigger_check.clone();
    widget.refresh_btn.connect_clicked(move |_| {
        trigger_check_btn();
    });

    // Handle Update All click -> Show PasswordDialog
    let auth_dialog_rc = Rc::new(auth_dialog);
    let auth_dialog_show = auth_dialog_rc.clone();
    let is_updating_click = is_updating.clone();
    widget.update_all_btn.connect_clicked(move |_| {
        if *is_updating_click.borrow() || is_pacman_running() {
            return;
        }
        auth_dialog_show.show_for(
            "Authentication Required",
            "Enter sudo password to apply system updates:",
        );
    });

    // Handle Password Submit -> Launch detached background update process
    let current_updates_start = current_updates.clone();
    let update_all_btn_start = widget.update_all_btn.clone();
    let refresh_btn_start = widget.refresh_btn.clone();
    let progress_bar_start = widget.progress_bar.clone();
    let status_label_start = widget.status_label.clone();
    let progress_box_start = progress_box.clone();
    let is_updating_start = is_updating.clone();
    let render_packages_start = render_packages.clone();
    let start_poller_start = start_state_poller.clone();

    auth_dialog_rc.connect_submit(move |password| {
        let pwd = match password {
            Some(p) if !p.trim().is_empty() => p,
            _ => return,
        };

        let pkgs_to_update = current_updates_start.borrow().clone();
        if pkgs_to_update.is_empty() {
            return;
        }

        *is_updating_start.borrow_mut() = true;
        update_all_btn_start.set_sensitive(false);
        update_all_btn_start.add_css_class("disabled");
        refresh_btn_start.set_sensitive(false);
        refresh_btn_start.add_css_class("disabled");

        if let Some(ref pbox) = progress_box_start {
            pbox.set_visible(true);
        }
        progress_bar_start.set_fraction(0.0);

        let initial_status_text = babydra_core::i18n::t("settings.update_syncing");
        status_label_start.set_text(&initial_status_text);

        // Mark all as Pending initially and save state with is_syncing = true
        for pkg in current_updates_start.borrow_mut().iter_mut() {
            pkg.status = UpdateStatus::Pending;
        }
        save_update_state(true, true, &current_updates_start.borrow());
        render_packages_start();

        // Spawn independent detached process running babydra-settings --run-background-update
        let exe = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("babydra-settings"));
        if let Ok(mut child) = Command::new(exe)
            .arg("--run-background-update")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = writeln!(stdin, "{}", pwd);
                let _ = stdin.flush();
            }
        }

        start_poller_start();
    });
}
