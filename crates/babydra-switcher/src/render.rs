//! UI renderer and event handlers for the switcher overlay window.

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;
use std::os::unix::net::UnixListener;
use std::io::Read;
use babydra_common::DesktopApp;
use babydra_common::{activate_app, save_history};
use crate::widgets::list::build_apps_list;

/// Builds and runs the Alt-Tab overlay switcher window.
/// Listens to key release events (specifically Alt release) or socket messages to commit selection.
pub fn build_switcher_ui(app: &gtk4::Application, apps: Vec<DesktopApp>) {
    babydra_utils::ui::theme::init_theme();

    let window = gtk4::ApplicationWindow::new(app);
    babydra_utils::ui::theme::apply_theme_class(&window);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    // Setup window geometry: stretch fullscreen across the entire screen
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.add_css_class("switcher-window");

    let main_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    main_box.add_css_class("stage-manager-container");
    main_box.set_valign(gtk4::Align::Fill);
    main_box.set_halign(gtk4::Align::Fill);

    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("stage-manager-panel");
    panel.set_valign(gtk4::Align::Fill);
    panel.set_halign(gtk4::Align::Start);

    let (icons_column, item_buttons) = build_apps_list(&apps);
    
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled.set_kinetic_scrolling(true);
    scrolled.set_child(Some(&icons_column));
    scrolled.set_vexpand(true);

    panel.append(&scrolled);
    main_box.append(&panel);

    // Dismiss on click outside of panel (width of panel is ~250px)
    let click_gesture = gtk4::GestureClick::new();
    let window_close = window.clone();
    click_gesture.connect_pressed(move |gesture, _, x, _y| {
        if x > 250.0 {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            window_close.close();
        }
    });
    main_box.add_controller(click_gesture);

    window.set_child(Some(&main_box));

    let current_index = Rc::new(RefCell::new(0));

    let update_selection = {
        let current_index = current_index.clone();
        let item_buttons = item_buttons.clone();

        move |new_idx: usize| {
            let mut idx = new_idx;
            if idx >= item_buttons.len() {
                idx = 0;
            }
            *current_index.borrow_mut() = idx;

            for (i, btn) in item_buttons.iter().enumerate() {
                if i == idx {
                    btn.add_css_class("selected");
                    btn.grab_focus();
                } else {
                    btn.remove_css_class("selected");
                }
            }
        }
    };

    let update_selection_rc = Rc::new(update_selection);
    let initial_idx = if apps.len() > 1 { 1 } else { 0 };
    update_selection_rc(initial_idx);

    for (i, btn) in item_buttons.iter().enumerate() {
        let update_sel = update_selection_rc.clone();
        let window_close = window.clone();
        let apps_click = apps.clone();
        btn.connect_clicked(move |_| {
            update_sel(i);
            let app_item = apps_click[i].clone();
            save_history(app_item.window_title.as_deref().unwrap_or(&app_item.name));
            activate_app(&app_item);
            let win = window_close.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(50), move || {
                win.close();
            });
        });
    }

    // Unix Socket Listener to handle subsequent Alt-Tab signals
    let (sender, receiver) = std::sync::mpsc::channel::<()>();
    std::thread::spawn(move || {
        let socket_path = "/tmp/babydra-switcher.socket";
        if let Ok(listener) = UnixListener::bind(socket_path) {
            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut buf = [0; 4];
                    if let Ok(_) = stream.read(&mut buf) {
                        if &buf[0..4] == b"next" {
                            let _ = sender.send(());
                        }
                    }
                }
            }
        }
    });

    let alt_check_enabled = Rc::new(RefCell::new(false));
    let alt_check_enabled_clone = alt_check_enabled.clone();
    gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(300), move || {
        *alt_check_enabled_clone.borrow_mut() = true;
    });

    let update_sel_socket = update_selection_rc.clone();
    let current_idx_socket = current_index.clone();
    let apps_len = apps.len();
    let alt_check_socket = alt_check_enabled.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(10), move || {
        while let Ok(_) = receiver.try_recv() {
            let idx = *current_idx_socket.borrow();
            let next = (idx + 1) % apps_len;
            update_sel_socket(next);

            // Reset grace period on each "next" signal to prevent premature close during rapid Alt+Tab cycling
            *alt_check_socket.borrow_mut() = false;
            let alt_re_enable = alt_check_socket.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(300), move || {
                *alt_re_enable.borrow_mut() = true;
            });
        }
        gtk4::glib::ControlFlow::Continue
    });

    // Helper: activate the selected app and close the switcher
    let do_activate = {
        let current_index = current_index.clone();
        let apps = apps.clone();
        let window = window.clone();
        Rc::new(move || {
            let idx = *current_index.borrow();
            if idx < apps.len() {
                let app_item = apps[idx].clone();
                save_history(app_item.window_title.as_deref().unwrap_or(&app_item.name));
                activate_app(&app_item);
            }
            let win = window.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(50), move || {
                win.close();
            });
        })
    };

    // Keyboard navigation handlers
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    let current_idx_key = current_index.clone();
    let update_sel_key = update_selection_rc.clone();
    let window_close = window.clone();
    let apps_key = apps.clone();
    let do_activate_press = do_activate.clone();
    
    key_controller.connect_key_pressed(move |_, key, _, _modifiers| {
        let idx = *current_idx_key.borrow();
        match key {
            gtk4::gdk::Key::Tab | gtk4::gdk::Key::Down => {
                let next = (idx + 1) % apps_key.len();
                update_sel_key(next);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::ISO_Left_Tab | gtk4::gdk::Key::Up => {
                let prev = if idx == 0 { apps_key.len() - 1 } else { idx - 1 };
                update_sel_key(prev);
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Return | gtk4::gdk::Key::space => {
                do_activate_press();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Escape => {
                window_close.close();
                gtk4::glib::Propagation::Stop
            }
            _ => gtk4::glib::Propagation::Proceed,
        }
    });

    // Track state to prevent double-firing activation
    let closed = Rc::new(RefCell::new(false));

    // Handle key releases to detect when Alt is lifted
    let do_activate_release = do_activate.clone();
    let alt_check_release = alt_check_enabled.clone();
    let closed_release = closed.clone();
    key_controller.connect_key_released(move |_, key, _, modifiers| {
        if *closed_release.borrow() || !*alt_check_release.borrow() {
            return;
        }

        let is_alt_key = matches!(
            key,
            gtk4::gdk::Key::Alt_L | gtk4::gdk::Key::Alt_R |
            gtk4::gdk::Key::Meta_L | gtk4::gdk::Key::Meta_R
        );
        let alt_held = modifiers.contains(gtk4::gdk::ModifierType::ALT_MASK);

        // Commit selection if the Alt modifier is released
        if is_alt_key || !alt_held {
            *closed_release.borrow_mut() = true;
            do_activate_release();
        }
    });

    window.add_controller(key_controller);
    window.present();

    if !item_buttons.is_empty() {
        item_buttons[0].grab_focus();
    }

    // Fallback: poll keyboard modifier state to catch edge cases
    let do_activate_poll = do_activate.clone();
    let alt_check_poll = alt_check_enabled.clone();
    let closed_poll = closed.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if *closed_poll.borrow() {
            return gtk4::glib::ControlFlow::Break;
        }
        if !*alt_check_poll.borrow() {
            return gtk4::glib::ControlFlow::Continue;
        }

        if let Some(display) = gtk4::gdk::Display::default() {
            if let Some(seat) = display.default_seat() {
                if let Some(keyboard) = seat.keyboard() {
                    let modifiers = keyboard.modifier_state();
                    let alt_held = modifiers.contains(gtk4::gdk::ModifierType::ALT_MASK);
                    if !alt_held {
                        *closed_poll.borrow_mut() = true;
                        do_activate_poll();
                        return gtk4::glib::ControlFlow::Break;
                    }
                }
            }
        }

        gtk4::glib::ControlFlow::Continue
    });
}
