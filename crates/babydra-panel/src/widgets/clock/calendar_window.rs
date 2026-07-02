use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use super::notifications;
use super::render;

pub fn show_calendar_window(
    app: &gtk4::Application,
    cw_clone: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::ApplicationWindow {
    let (
        c_win,
        main_box,
        date_label,
        dummy_time,
        _calendar,
        clear_btn,
        notif_stack,
    ) = render::build_calendar_window_ui(app);

    // 7. Setup and Render Notifications list
    notifications::setup_notifications_list(&notif_stack, &clear_btn, &dummy_time, &date_label);

    // Dismiss when clicking outside the calendar box area using common helper
    babydra_common::window::setup_click_outside_dismiss(&c_win, &main_box);

    // Handle closing when window loses focus
    let cw_inner = cw_clone.clone();
    c_win.connect_is_active_notify(move |win| {
        if !win.is_active() {
            win.close();
        }
    });

    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_clone = is_animating.clone();
    let cw_inner_cb_clone = cw_inner.clone();
    let c_win_clone = c_win.clone();
    let main_box_clone = main_box.clone();
    c_win.connect_close_request(move |_| {
        if is_animating_clone.get() {
            return glib::Propagation::Stop;
        }
        is_animating_clone.set(true);
        if let Ok(mut borrow) = cw_inner_cb_clone.try_borrow_mut() {
            *borrow = None;
        }
        let h = main_box_clone.height().max(480);
        let c_win_cb = c_win_clone.clone();
        babydra_common::animation::genie_out(
            main_box_clone.upcast_ref(),
            360,
            h,
            450,
            move || {
                c_win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    let (_, natural_size) = main_box.preferred_size();
    let target_height = if natural_size.height() > 20 { natural_size.height() } else { 480 };
    c_win.present();
    babydra_common::animation::genie_in(main_box.upcast_ref(), 360, target_height, 450);

    c_win
}
