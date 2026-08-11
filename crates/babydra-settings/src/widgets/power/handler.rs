use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_utils::components::modal::PasswordDialog;
use babydra_common::{
    PerformanceProfile, get_current_profile, set_performance_profile, set_performance_profile_with_password,
    get_battery_info, load_babydra_config, save_babydra_config,
    battery::check_and_apply_auto_battery_saver,
};
use super::render::{PowerWidget, update_battery_card_ui, update_profile_selection, update_power_widget_labels};

pub fn wire_events(widget: &PowerWidget, auth_dialog: PasswordDialog) {
    let pending_profile: Rc<RefCell<Option<PerformanceProfile>>> = Rc::new(RefCell::new(None));

    // Watch locale changes to update labels dynamically
    let widget_rc = widget.clone();
    babydra_common::i18n::watch_locale_change(move |_| {
        update_power_widget_labels(&widget_rc);
    });

    // Initial Load for Auto Battery Saver Settings
    let mut init_conf = load_babydra_config();
    if !init_conf.power.auto_saver_enabled {
        init_conf.power.auto_saver_enabled = true;
        save_babydra_config(&init_conf);
    }
    widget.threshold_slider.set_value(init_conf.power.saver_threshold);

    // Wire Battery Threshold Slider change callback with debounce & notification
    let last_saved_threshold = Rc::new(std::cell::Cell::new(init_conf.power.saver_threshold));
    let debounce_source_id: Rc<std::cell::Cell<Option<glib::SourceId>>> = Rc::new(std::cell::Cell::new(None));

    let debounce_id_c = debounce_source_id;
    widget.threshold_slider.connect_change(move |threshold| {
        if threshold != last_saved_threshold.get() {
            last_saved_threshold.set(threshold);

            // Remove any previous pending timeout
            if let Some(source_id) = debounce_id_c.take() {
                source_id.remove();
            }

            let debounce_id_inner = debounce_id_c.clone();
            let new_source_id = glib::timeout_add_local(std::time::Duration::from_millis(350), move || {
                debounce_id_inner.set(None);

                let mut conf = load_babydra_config();
                conf.power.saver_threshold = threshold;
                conf.power.auto_saver_enabled = true;
                save_babydra_config(&conf);

                // Send Notification
                let title = babydra_common::i18n::t("settings.notif_saver_threshold_title");
                let msg = babydra_common::i18n::t("settings.notif_saver_threshold_msg")
                    .replace("{threshold}", &threshold.to_string());
                babydra_common::send_settings_notification(&title, &msg);

                // Check and apply auto battery saver
                let (tx, rx) = std::sync::mpsc::channel();
                std::thread::spawn(move || {
                    if let Some(info) = get_battery_info() {
                        let _ = tx.send(info);
                    }
                });
                glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
                    if let Ok(info) = rx.try_recv() {
                        check_and_apply_auto_battery_saver(&info);
                        glib::ControlFlow::Break
                    } else {
                        glib::ControlFlow::Continue
                    }
                });

                glib::ControlFlow::Break
            });

            debounce_id_c.set(Some(new_source_id));
        }
    });

    // Initial Load & Apply Charge Limit Settings
    widget.charge_slider.set_value(init_conf.power.charge_limit);
    let initial_limit = init_conf.power.charge_limit;
    std::thread::spawn(move || {
        let _ = babydra_common::services::system::battery::set_charge_limit(initial_limit);
    });

    // Wire Charge Limit Slider change callback with debounce, sysfs apply & sudo authentication dialog
    let last_saved_charge_limit = Rc::new(std::cell::Cell::new(init_conf.power.charge_limit));
    let debounce_charge_id: Rc<std::cell::Cell<Option<glib::SourceId>>> = Rc::new(std::cell::Cell::new(None));
    let pending_charge_limit: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(None));

    let debounce_charge_c = debounce_charge_id;
    let pending_charge_limit_c = pending_charge_limit.clone();
    let auth_dialog_charge = auth_dialog.clone();

    widget.charge_slider.connect_change(move |limit| {
        if limit != last_saved_charge_limit.get() {
            last_saved_charge_limit.set(limit);

            if let Some(source_id) = debounce_charge_c.take() {
                source_id.remove();
            }

            let debounce_charge_inner = debounce_charge_c.clone();
            let pending_charge_inner = pending_charge_limit_c.clone();
            let auth_dialog_inner = auth_dialog_charge.clone();

            let new_source_id = glib::timeout_add_local(std::time::Duration::from_millis(350), move || {
                debounce_charge_inner.set(None);

                let res = babydra_common::services::system::battery::set_charge_limit(limit);
                match res {
                    Ok(()) => {
                        let mut conf = load_babydra_config();
                        conf.power.charge_limit = limit;
                        save_babydra_config(&conf);

                        let title = babydra_common::i18n::t("settings.notif_charge_limit_title");
                        let msg = babydra_common::i18n::t("settings.notif_charge_limit_msg")
                            .replace("{limit}", &limit.to_string());
                        babydra_common::send_settings_notification(&title, &msg);
                    }
                    Err(err) if err == "permission_denied" => {
                        *pending_charge_inner.borrow_mut() = Some(limit);
                        auth_dialog_inner.show_for(
                            "Authentication Required",
                            "Enter sudo password to set battery charging limit:",
                        );
                    }
                    _ => {
                        let mut conf = load_babydra_config();
                        conf.power.charge_limit = limit;
                        save_babydra_config(&conf);

                        let title = babydra_common::i18n::t("settings.notif_charge_limit_title");
                        let msg = babydra_common::i18n::t("settings.notif_charge_limit_msg")
                            .replace("{limit}", &limit.to_string());
                        babydra_common::send_settings_notification(&title, &msg);
                    }
                }

                glib::ControlFlow::Break
            });

            debounce_charge_c.set(Some(new_source_id));
        }
    });

    // Helper to refresh battery card UI immediately
    let refresh_battery = |battery_card: &gtk4::Box| {
        let card_c = battery_card.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let bat_info = get_battery_info();
            let _ = tx.send(bat_info);
        });
        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            if let Ok(bat_info) = rx.try_recv() {
                update_battery_card_ui(&card_c, bat_info);
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    };

    // Initial battery card render
    refresh_battery(&widget.battery_card);

    // Initial Profile state update
    let current_profile = get_current_profile();
    update_profile_selection(
        &widget.profile_balanced_btn,
        &widget.profile_normal_btn,
        &widget.profile_high_btn,
        current_profile,
    );

    // Wire Performance Profile Buttons
    let setup_profile_click = |btn: &gtk4::Button, target_profile: PerformanceProfile| {
        let btn_c = btn.clone();
        let balanced_c = widget.profile_balanced_btn.clone();
        let normal_c = widget.profile_normal_btn.clone();
        let high_c = widget.profile_high_btn.clone();
        let pending_c = pending_profile.clone();
        let auth_dialog_c = auth_dialog.clone();

        btn_c.connect_clicked(move |_| {
            if get_current_profile() == target_profile {
                return;
            }

            match set_performance_profile(target_profile) {
                Ok(()) => {
                    let mut conf = load_babydra_config();
                    conf.power.profile = target_profile.key().to_string();
                    save_babydra_config(&conf);

                    update_profile_selection(&balanced_c, &normal_c, &high_c, target_profile);

                    let title = babydra_common::i18n::t("settings.notif_power_title");
                    let msg = babydra_common::i18n::t("settings.notif_power_msg").replace("{profile}", target_profile.label());
                    babydra_common::send_settings_notification(&title, &msg);
                }
                Err(_) => {
                    *pending_c.borrow_mut() = Some(target_profile);
                    auth_dialog_c.show_for(
                        "Authentication Required",
                        "Enter sudo password to change CPU performance profile:",
                    );
                }
            }
        });
    };

    setup_profile_click(&widget.profile_balanced_btn, PerformanceProfile::Balanced);
    setup_profile_click(&widget.profile_normal_btn, PerformanceProfile::Normal);
    setup_profile_click(&widget.profile_high_btn, PerformanceProfile::HighPerformance);

    // Wire Authentication Dialog Submission
    let balanced_auth = widget.profile_balanced_btn.clone();
    let normal_auth = widget.profile_normal_btn.clone();
    let high_auth = widget.profile_high_btn.clone();
    let pending_auth = pending_profile;
    let pending_charge_auth = pending_charge_limit;

    auth_dialog.connect_submit(move |pwd_opt| {
        let pwd = match pwd_opt {
            Some(p) => p,
            None => return,
        };

        if let Some(target_prof) = *pending_auth.borrow() {
            if set_performance_profile_with_password(target_prof, &pwd).is_ok() {
                let mut conf = load_babydra_config();
                conf.power.profile = target_prof.key().to_string();
                save_babydra_config(&conf);

                update_profile_selection(&balanced_auth, &normal_auth, &high_auth, target_prof);

                let title = babydra_common::i18n::t("settings.notif_power_title");
                let msg = babydra_common::i18n::t("settings.notif_power_msg").replace("{profile}", target_prof.label());
                babydra_common::send_settings_notification(&title, &msg);

                *pending_auth.borrow_mut() = None;
            }
        }

        if let Some(limit) = *pending_charge_auth.borrow() {
            if babydra_common::services::system::battery::set_charge_limit_auth(limit, &pwd).is_ok() {
                let mut conf = load_babydra_config();
                conf.power.charge_limit = limit;
                save_babydra_config(&conf);

                let title = babydra_common::i18n::t("settings.notif_charge_limit_title");
                let msg = babydra_common::i18n::t("settings.notif_charge_limit_msg")
                    .replace("{limit}", &limit.to_string());
                babydra_common::send_settings_notification(&title, &msg);

                *pending_charge_auth.borrow_mut() = None;
            }
        }
    });
}
