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
    let auth_dialog_rc = Rc::new(auth_dialog);
    let pending_profile: Rc<RefCell<Option<PerformanceProfile>>> = Rc::new(RefCell::new(None));

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let bat_info = get_battery_info();
        let cur_prof = get_current_profile();
        let _ = tx.send((bat_info, cur_prof));
    });

    let battery_card_c = widget.battery_card.clone();
    let balanced_btn_c = widget.profile_balanced_btn.clone();
    let normal_btn_c = widget.profile_normal_btn.clone();
    let high_btn_c = widget.profile_high_btn.clone();

    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if let Ok((bat_info, cur_prof)) = rx.try_recv() {
            update_battery_card_ui(&battery_card_c, bat_info);
            update_profile_selection(
                &balanced_btn_c,
                &normal_btn_c,
                &high_btn_c,
                cur_prof,
            );
            glib::ControlFlow::Break
        } else {
            glib::ControlFlow::Continue
        }
    });

    // Profile click handler helper
    let wire_profile_click = |btn: &gtk4::Button, prof: PerformanceProfile| {
        let balanced_c = widget.profile_balanced_btn.clone();
        let normal_c = widget.profile_normal_btn.clone();
        let high_c = widget.profile_high_btn.clone();
        let badge_c = widget.status_badge.clone();
        let auth_dlg_c = auth_dialog_rc.clone();
        let pending_c = pending_profile.clone();

        btn.connect_clicked(move |_| {
            let balanced_inner = balanced_c.clone();
            let normal_inner = normal_c.clone();
            let high_inner = high_c.clone();
            let badge_inner = badge_c.clone();
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
                            badge_inner.set_text(&format!("Applied '{}' performance profile.", prof.label()));
                            badge_inner.set_visible(true);
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

    // Wire PasswordDialog submit
    let balanced_sub = widget.profile_balanced_btn.clone();
    let normal_sub = widget.profile_normal_btn.clone();
    let high_sub = widget.profile_high_btn.clone();
    let badge_sub = widget.status_badge.clone();
    let pending_sub = pending_profile.clone();

    auth_dialog_rc.connect_submit(move |password_opt| {
        let password = match password_opt {
            Some(p) => p,
            None => return,
        };

        let prof = match *pending_sub.borrow() {
            Some(p) => p,
            None => return,
        };

        let balanced_c2 = balanced_sub.clone();
        let normal_c2 = normal_sub.clone();
        let high_c2 = high_sub.clone();
        let badge_c2 = badge_sub.clone();

        let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();
        std::thread::spawn(move || {
            let res = set_performance_profile_with_password(prof, &password);
            let _ = tx.send(res);
        });

        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(res) = rx.try_recv() {
                match res {
                    Ok(()) => {
                        update_profile_selection(&balanced_c2, &normal_c2, &high_c2, prof);
                        badge_c2.set_text(&format!("Applied '{}' performance profile.", prof.label()));
                        badge_c2.set_visible(true);
                    }
                    Err(err) => {
                        badge_c2.set_text(&format!("Failed to set performance profile: {}", err));
                        badge_c2.set_visible(true);
                    }
                }
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    });
}
