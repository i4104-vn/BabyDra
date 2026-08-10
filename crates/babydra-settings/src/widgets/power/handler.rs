use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_utils::components::modal::PasswordDialog;
use babydra_common::{
    PerformanceProfile, get_current_profile, set_performance_profile, set_performance_profile_with_password,
    get_battery_info,
};
use super::render::{PowerWidget, update_battery_card_ui, update_profile_selection};

pub fn wire_events(widget: &PowerWidget, auth_dialog: PasswordDialog) {
    let pending_profile: Rc<RefCell<Option<PerformanceProfile>>> = Rc::new(RefCell::new(None));

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

    // Initial Fetch for Battery & Current Performance Profile
    let battery_card_init = widget.battery_card.clone();
    let balanced_btn_init = widget.profile_balanced_btn.clone();
    let normal_btn_init = widget.profile_normal_btn.clone();
    let high_btn_init = widget.profile_high_btn.clone();

    let (tx_init, rx_init) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let bat_info = get_battery_info();
        let cur_prof = get_current_profile();
        let _ = tx_init.send((bat_info, cur_prof));
    });

    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if let Ok((bat_info, cur_prof)) = rx_init.try_recv() {
            update_battery_card_ui(&battery_card_init, bat_info);
            update_profile_selection(
                &balanced_btn_init,
                &normal_btn_init,
                &high_btn_init,
                cur_prof,
            );
            glib::ControlFlow::Break
        } else {
            glib::ControlFlow::Continue
        }
    });

    // 3-Second Periodic Timer to refresh battery data automatically
    let battery_card_timer = widget.battery_card.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(3), move || {
        let card_c = battery_card_timer.clone();

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

        glib::ControlFlow::Continue
    });

    // Wire PasswordDialog submit
    let battery_card_sub = widget.battery_card.clone();
    let balanced_sub = widget.profile_balanced_btn.clone();
    let normal_sub = widget.profile_normal_btn.clone();
    let high_sub = widget.profile_high_btn.clone();
    let pending_sub = pending_profile.clone();

    auth_dialog.connect_submit(move |pwd_opt| {
        let password = match pwd_opt {
            Some(p) => p,
            None => return,
        };
        let prof_opt = *pending_sub.borrow();
        let prof = match prof_opt {
            Some(p) => p,
            None => return,
        };

        let card_inner = battery_card_sub.clone();
        let balanced_inner = balanced_sub.clone();
        let normal_inner = normal_sub.clone();
        let high_inner = high_sub.clone();

        let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();
        std::thread::spawn(move || {
            let res = set_performance_profile_with_password(prof, &password);
            let _ = tx.send(res);
        });

        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(res) = rx.try_recv() {
                if res.is_ok() {
                    update_profile_selection(&balanced_inner, &normal_inner, &high_inner, prof);
                    refresh_battery(&card_inner);
                }
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    });

    let auth_dialog_rc = Rc::new(auth_dialog);

    // Profile click handler helper
    let wire_profile_click = |btn: &gtk4::Button, prof: PerformanceProfile| {
        let card_c = widget.battery_card.clone();
        let balanced_c = widget.profile_balanced_btn.clone();
        let normal_c = widget.profile_normal_btn.clone();
        let high_c = widget.profile_high_btn.clone();
        let auth_dlg_c = auth_dialog_rc.clone();
        let pending_c = pending_profile.clone();

        btn.connect_clicked(move |_| {
            let card_inner = card_c.clone();
            let balanced_inner = balanced_c.clone();
            let normal_inner = normal_c.clone();
            let high_inner = high_c.clone();
            let auth_dlg_inner = auth_dlg_c.clone();
            let pending_inner = pending_c.clone();

            let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();
            std::thread::spawn(move || {
                let res = set_performance_profile(prof);
                let _ = tx.send(res);
            });

            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if let Ok(res) = rx.try_recv() {
                    match res {
                        Ok(()) => {
                            update_profile_selection(&balanced_inner, &normal_inner, &high_inner, prof);
                            refresh_battery(&card_inner);
                        }
                        Err(_) => {
                            // Prompt PasswordDialog for root elevation
                            *pending_inner.borrow_mut() = Some(prof);
                            auth_dlg_inner.show_for(
                                "Authentication Required",
                                &format!("Enter sudo password to apply '{}' performance profile:", prof.label()),
                            );
                        }
                    }
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        });
    };

    wire_profile_click(&widget.profile_balanced_btn, PerformanceProfile::Balanced);
    wire_profile_click(&widget.profile_normal_btn, PerformanceProfile::Normal);
    wire_profile_click(&widget.profile_high_btn, PerformanceProfile::HighPerformance);
}
