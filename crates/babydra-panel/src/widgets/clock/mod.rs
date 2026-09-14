use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod calendar;
mod notifications;
mod render;

/// Creates and returns a clock button widget that updates every second and
/// spawns a centered, glassmorphic calendar popup dropdown when clicked.
pub fn create_clock_widget(
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    popdown_tooltips: Option<Rc<dyn Fn()>>,
) -> gtk4::Button {
    let (clock_button, clock_label, red_dot, bell_box) = render::build_clock_ui();
    babydra_core::services::notification::spawn_notif_dbus();

    let notification_popup = Rc::new(notifications::NotificationPopup::new(&clock_button));
    let last_notification = Rc::new(std::cell::Cell::new(None));

    let bell_popover = notifications::setup_bell_popover(
        &bell_box,
        calendar_window.clone(),
        control_center_window.clone(),
        launcher_window.clone(),
        notification_popup.clone(),
    );

    let update_clock = {
        let clock_label = clock_label.clone();
        let red_dot = red_dot.clone();
        let notification_popup = notification_popup.clone();
        let last_notification = last_notification.clone();
        let bell_popover = bell_popover.clone();
        move || {
            let now = chrono::Local::now();
            let time_str = format!(
                "{}   {}",
                now.format("%d/%m").to_string(),
                now.format("%I:%M %p").to_string().to_uppercase()
            );
            clock_label.set_text(&time_str);

            let notif_count =
                babydra_core::services::notification::service::HISTORICAL_NOTIFICATIONS
                    .with(|list| list.borrow().len());
            red_dot.set_visible(notif_count > 0);

            let active_notification =
                babydra_core::services::notification::service::SHARED_NOTIFICATION
                    .with(|notification| notification.borrow().clone());
            match active_notification {
                Some(notification) if last_notification.get() != Some(notification.timestamp) => {
                    last_notification.set(Some(notification.timestamp));
                    bell_popover.popdown();
                    notification_popup.show(&notification);
                }
                None => {
                    last_notification.set(None);
                    notification_popup.close();
                }
                _ => {}
            }

            glib::ControlFlow::Continue
        }
    };
    update_clock();
    glib::timeout_add_local(std::time::Duration::from_secs(1), update_clock);

    let cw_clone = calendar_window.clone();
    let ccw_clone = control_center_window.clone();
    let lw_clone = launcher_window.clone();
    let app_clone = app.clone();
    let popdown_c = popdown_tooltips.clone();
    let notification_popup_c = notification_popup.clone();
    let bell_popover_c = bell_popover.clone();

    clock_button.connect_clicked(move |_| {
        notification_popup_c.close();
        bell_popover_c.popdown();
        if let Some(ref popdown) = popdown_c {
            popdown();
        }

        let cc_win = ccw_clone.borrow().clone();
        if let Some(win) = cc_win {
            win.close();
        }

        let launch_win = lw_clone.borrow().clone();
        if let Some(win) = launch_win {
            win.close();
        }

        let existing = cw_clone.borrow().clone();
        if let Some(existing_window) = existing {
            existing_window.close();
        } else {
            let window = calendar::show_calendar_window(&app_clone, cw_clone.clone());
            if let Ok(mut borrow) = cw_clone.try_borrow_mut() {
                *borrow = Some(window);
            }
        }
    });

    clock_button
}
