use crate::manager::{switch_workspace, DEFAULT_WORKSPACE_COUNT};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Attaches click gesture to dismiss overlay when clicking outside the deck.
pub fn attach_dismiss_gesture(
    overlay_box: &gtk4::Box,
    deck_container: &gtk4::Box,
    window: &gtk4::ApplicationWindow,
) {
    let click_gesture = gtk4::GestureClick::new();
    let win_dismiss = window.clone();
    let deck_ref = deck_container.clone();
    click_gesture.connect_pressed(move |gesture, _, x, y| {
        let (deck_x, deck_y) = (deck_ref.allocated_width(), deck_ref.allocated_height());
        let (alloc_x, alloc_y) = (
            deck_ref.allocation().x() as f64,
            deck_ref.allocation().y() as f64,
        );
        let inside = x >= alloc_x
            && x <= alloc_x + deck_x as f64
            && y >= alloc_y
            && y <= alloc_y + deck_y as f64;
        if !inside {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            win_dismiss.set_visible(false);
        }
    });
    overlay_box.add_controller(click_gesture);
}

/// Attaches keyboard navigation controller to the window.
pub fn attach_key_controller(
    window: &gtk4::ApplicationWindow,
    current_idx: Rc<RefCell<usize>>,
    do_activate: Rc<dyn Fn()>,
    update_sel: Rc<dyn Fn(usize)>,
) {
    let key_controller = gtk4::EventControllerKey::new();
    let sel_for_key = update_sel.clone();
    let curr_for_key = current_idx.clone();
    let act_for_key = do_activate.clone();
    let win_for_key = window.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        match keyval {
            gtk4::gdk::Key::Escape => {
                win_for_key.set_visible(false);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter | gtk4::gdk::Key::space => {
                act_for_key();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Left | gtk4::gdk::Key::Up => {
                let idx = *curr_for_key.borrow();
                let next = if idx == 0 {
                    (DEFAULT_WORKSPACE_COUNT as usize) - 1
                } else {
                    idx - 1
                };
                sel_for_key(next);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Right | gtk4::gdk::Key::Down | gtk4::gdk::Key::Tab => {
                let idx = *curr_for_key.borrow();
                let next = (idx + 1) % (DEFAULT_WORKSPACE_COUNT as usize);
                sel_for_key(next);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::_1 | gtk4::gdk::Key::KP_1 => {
                switch_workspace(1);
                win_for_key.set_visible(false);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::_2 | gtk4::gdk::Key::KP_2 => {
                switch_workspace(2);
                win_for_key.set_visible(false);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::_3 | gtk4::gdk::Key::KP_3 => {
                switch_workspace(3);
                win_for_key.set_visible(false);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::_4 | gtk4::gdk::Key::KP_4 => {
                switch_workspace(4);
                win_for_key.set_visible(false);
                gtk4::glib::Propagation::Stop
            }
            _ => gtk4::glib::Propagation::Proceed,
        }
    });
    window.add_controller(key_controller);
}

/// Attaches mouse scroll wheel controller to the deck container.
pub fn attach_scroll_controller(
    deck_container: &gtk4::Box,
    current_idx: Rc<RefCell<usize>>,
    update_sel: Rc<dyn Fn(usize)>,
) {
    let scroll_controller =
        gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let sel_for_scroll = update_sel.clone();
    let curr_for_scroll = current_idx.clone();

    scroll_controller.connect_scroll(move |_, _, dy| {
        let idx = *curr_for_scroll.borrow();
        if dy > 0.0 {
            sel_for_scroll((idx + 1) % (DEFAULT_WORKSPACE_COUNT as usize));
        } else if dy < 0.0 {
            let prev = if idx == 0 {
                (DEFAULT_WORKSPACE_COUNT as usize) - 1
            } else {
                idx - 1
            };
            sel_for_scroll(prev);
        }
        gtk4::glib::Propagation::Stop
    });
    deck_container.add_controller(scroll_controller);
}
