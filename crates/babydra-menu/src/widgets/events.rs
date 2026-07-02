//! Event listener handlers to dismiss or close the menu.

use gtk4::prelude::*;
use crate::render::close_menu_animated;

/// Configures event controllers to dismiss the popup context menu when the user
/// clicks outside its bounds or presses the Escape key.
pub fn setup_menu_dismiss_events(
    window: &gtk4::ApplicationWindow,
    menu_box: &gtk4::Box,
) {
    let click_gesture = gtk4::GestureClick::new();
    let menu_box_c = menu_box.clone();
    let window_c = window.clone();
    click_gesture.connect_pressed(move |_, _, x, y| {
        let picked = window_c.pick(x, y, gtk4::PickFlags::DEFAULT);
        let inside_menu = picked
            .map(|w| w.is_ancestor(&menu_box_c) || w == menu_box_c)
            .unwrap_or(false);

        if !inside_menu {
            close_menu_animated(&window_c, &menu_box_c, None);
        }
    });
    window.add_controller(click_gesture);

    let key_controller = gtk4::EventControllerKey::new();
    let window_c2 = window.clone();
    let menu_box_c2 = menu_box.clone();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            close_menu_animated(&window_c2, &menu_box_c2, None);
            gtk4::glib::Propagation::Proceed
        } else {
            gtk4::glib::Propagation::Stop
        }
    });
    window.add_controller(key_controller);
}

