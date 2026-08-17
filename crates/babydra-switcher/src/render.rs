//! UI renderer and event handlers for the switcher overlay window.

use crate::widgets::list::build_apps_list;
use babydra_core::DesktopApp;
use babydra_core::{activate_app, save_history};
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;

/// Shared state for the daemon window controller.
pub struct SwitcherController {
    pub window: gtk4::ApplicationWindow,
    pub show_fn: Box<dyn Fn()>,
    pub hide_fn: Box<dyn Fn()>,
    pub next_fn: Box<dyn Fn()>,
}

/// Builds the switcher overlay window once and returns a controller that can
/// show, hide, or cycle selection without rebuilding the entire widget tree.
///
/// In daemon mode this is called once at startup; the window is simply
/// hidden/shown on subsequent Alt+Tab presses rather than destroyed/recreated.
pub fn build_switcher_ui(app: &gtk4::Application) -> SwitcherController {
    babydra_ui_kit::ui::theme::init_theme();

    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_exclusive_zone(-1);

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

    // The apps list and buttons will be rebuilt each time the switcher is shown.
    // We store them in a RefCell so the show closure can replace them.
    let list_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    list_container.set_vexpand(true);

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled.set_kinetic_scrolling(true);
    scrolled.set_child(Some(&list_container));
    scrolled.set_vexpand(true);

    panel.append(&scrolled);
    main_box.append(&panel);

    // Dismiss on click outside the panel
    let click_gesture = gtk4::GestureClick::new();
    let window_hide_click = window.clone();
    click_gesture.connect_pressed(move |gesture, _, x, _y| {
        if x > 250.0 {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            window_hide_click.set_visible(false);
        }
    });
    main_box.add_controller(click_gesture);

    window.set_child(Some(&main_box));

    // --- Shared mutable state ---
    let apps_state: Rc<RefCell<Vec<DesktopApp>>> = Rc::new(RefCell::new(Vec::new()));
    let buttons_state: Rc<RefCell<Vec<gtk4::Button>>> = Rc::new(RefCell::new(Vec::new()));
    let current_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let closed: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let alt_check_enabled: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let last_cycle_time: Rc<RefCell<std::time::Instant>> =
        Rc::new(RefCell::new(std::time::Instant::now()));

    // --- update_selection closure ---
    let update_selection = {
        let current_index = current_index.clone();
        let buttons_state = buttons_state.clone();
        Rc::new(move |new_idx: usize| {
            let buttons = buttons_state.borrow();
            let mut idx = new_idx;
            if idx >= buttons.len() {
                idx = 0;
            }
            *current_index.borrow_mut() = idx;
            for (i, btn) in buttons.iter().enumerate() {
                if i == idx {
                    btn.add_css_class("selected");
                    btn.grab_focus();
                } else {
                    btn.remove_css_class("selected");
                }
            }
        })
    };

    // --- do_activate closure ---
    let do_activate = {
        let current_index = current_index.clone();
        let apps_state = apps_state.clone();
        let window = window.clone();
        let closed = closed.clone();
        Rc::new(move || {
            let idx = *current_index.borrow();
            let apps = apps_state.borrow();
            if idx < apps.len() {
                let app_item = apps[idx].clone();
                save_history(app_item.window_title.as_deref().unwrap_or(&app_item.name));
                activate_app(&app_item);
            }
            drop(apps);
            *closed.borrow_mut() = true;
            let win = window.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(50), move || {
                win.set_visible(false);
            });
        })
    };

    // --- Keyboard controller ---
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

    {
        let update_sel_key = update_selection.clone();
        let current_idx_key = current_index.clone();
        let apps_state_key = apps_state.clone();
        let window_esc = window.clone();
        let do_activate_press = do_activate.clone();
        let last_cycle_key = last_cycle_time.clone();

        key_controller.connect_key_pressed(move |_, key, _, _modifiers| {
            let idx = *current_idx_key.borrow();
            let apps_len = apps_state_key.borrow().len();
            if apps_len == 0 {
                return gtk4::glib::Propagation::Proceed;
            }
            let now = std::time::Instant::now();
            match key {
                gtk4::gdk::Key::Tab | gtk4::gdk::Key::Down => {
                    if now.duration_since(*last_cycle_key.borrow())
                        > std::time::Duration::from_millis(80)
                    {
                        *last_cycle_key.borrow_mut() = now;
                        update_sel_key((idx + 1) % apps_len);
                    }
                    gtk4::glib::Propagation::Stop
                }
                gtk4::gdk::Key::ISO_Left_Tab | gtk4::gdk::Key::Up => {
                    if now.duration_since(*last_cycle_key.borrow())
                        > std::time::Duration::from_millis(80)
                    {
                        *last_cycle_key.borrow_mut() = now;
                        let prev = if idx == 0 { apps_len - 1 } else { idx - 1 };
                        update_sel_key(prev);
                    }
                    gtk4::glib::Propagation::Stop
                }
                gtk4::gdk::Key::Return | gtk4::gdk::Key::space => {
                    do_activate_press();
                    gtk4::glib::Propagation::Stop
                }
                gtk4::gdk::Key::Escape => {
                    window_esc.set_visible(false);
                    gtk4::glib::Propagation::Stop
                }
                _ => gtk4::glib::Propagation::Proceed,
            }
        });
    }

    {
        let do_activate_release = do_activate.clone();
        let alt_check_release = alt_check_enabled.clone();
        let closed_release = closed.clone();
        key_controller.connect_key_released(move |_, key, _, modifiers| {
            if *closed_release.borrow() || !*alt_check_release.borrow() {
                return;
            }
            let is_alt_key = matches!(
                key,
                gtk4::gdk::Key::Alt_L
                    | gtk4::gdk::Key::Alt_R
                    | gtk4::gdk::Key::Meta_L
                    | gtk4::gdk::Key::Meta_R
            );
            let alt_held = modifiers.contains(gtk4::gdk::ModifierType::ALT_MASK);
            if is_alt_key || !alt_held {
                *closed_release.borrow_mut() = true;
                do_activate_release();
            }
        });
    }

    window.add_controller(key_controller);

    // Fallback: poll modifier state every 50ms
    {
        let do_activate_poll = do_activate.clone();
        let alt_check_poll = alt_check_enabled.clone();
        let closed_poll = closed.clone();
        let window_poll = window.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            if !window_poll.is_visible() {
                return gtk4::glib::ControlFlow::Continue;
            }
            if *closed_poll.borrow() {
                return gtk4::glib::ControlFlow::Continue;
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
                        }
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });
    }

    // --- show_fn: rebuilds the app list and presents the window ---
    let show_fn = {
        let window = window.clone();
        let list_container = list_container.clone();
        let apps_state = apps_state.clone();
        let buttons_state = buttons_state.clone();
        let update_selection = update_selection.clone();
        let current_index = current_index.clone();
        let closed = closed.clone();
        let alt_check_enabled = alt_check_enabled.clone();
        let last_cycle_show = last_cycle_time.clone();

        Box::new(move || {
            *closed.borrow_mut() = false;
            *alt_check_enabled.borrow_mut() = false;
            // Show the window immediately for 0-latency feedback
            *last_cycle_show.borrow_mut() = std::time::Instant::now();

            window.set_visible(true);
            window.present();

            // Spawn a background thread to query the compositor for running apps
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let apps = babydra_core::get_running_apps();
                let _ = tx.send(apps);
            });

            let list_container = list_container.clone();
            let apps_state = apps_state.clone();
            let buttons_state = buttons_state.clone();
            let update_selection = update_selection.clone();
            let current_index = current_index.clone();
            let window = window.clone();
            let closed = closed.clone();
            let alt_check_enabled = alt_check_enabled.clone();

            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(2), move || {
                if let Ok(apps) = rx.try_recv() {
                    if apps.is_empty() {
                        window.set_visible(false);
                        return gtk4::glib::ControlFlow::Break;
                    }

                    while let Some(child) = list_container.first_child() {
                        list_container.remove(&child);
                    }

                    let (icons_column, item_buttons) = build_apps_list(&apps);

                    // Wire up click handlers
                    for (i, btn) in item_buttons.iter().enumerate() {
                        let update_sel = update_selection.clone();
                        let window_close = window.clone();
                        let apps_click = apps.clone();
                        let closed_click = closed.clone();
                        btn.connect_clicked(move |_| {
                            update_sel(i);
                            let app_item = apps_click[i].clone();
                            save_history(
                                app_item.window_title.as_deref().unwrap_or(&app_item.name),
                            );
                            activate_app(&app_item);
                            *closed_click.borrow_mut() = true;
                            let win = window_close.clone();
                            gtk4::glib::timeout_add_local_once(
                                std::time::Duration::from_millis(50),
                                move || {
                                    win.set_visible(false);
                                },
                            );
                        });
                    }

                    list_container.append(&icons_column);

                    *apps_state.borrow_mut() = apps.clone();
                    *buttons_state.borrow_mut() = item_buttons;

                    // Apply the selection immediately — no frame delay needed
                    let initial_idx = if apps.len() > 1 { 1 } else { 0 };
                    *current_index.borrow_mut() = initial_idx;

                    update_selection(initial_idx);

                    // Grace period for GTK layer shell keyboard focus assignment
                    let alt_check = alt_check_enabled.clone();
                    gtk4::glib::timeout_add_local_once(
                        std::time::Duration::from_millis(30),
                        move || {
                            *alt_check.borrow_mut() = true;
                        },
                    );

                    gtk4::glib::ControlFlow::Break
                } else {
                    gtk4::glib::ControlFlow::Continue
                }
            });
        })
    };

    // --- hide_fn ---
    let hide_fn = {
        let window = window.clone();
        Box::new(move || {
            window.set_visible(false);
        })
    };

    // --- next_fn: cycle selection if window is already visible ---
    let next_fn = {
        let update_selection = update_selection.clone();
        let current_index = current_index.clone();
        let apps_state = apps_state.clone();
        let last_cycle_next = last_cycle_time.clone();
        Box::new(move || {
            let now = std::time::Instant::now();
            if now.duration_since(*last_cycle_next.borrow()) < std::time::Duration::from_millis(80)
            {
                return;
            }
            *last_cycle_next.borrow_mut() = now;

            let idx = *current_index.borrow();
            let apps_len = apps_state.borrow().len();
            if apps_len > 0 {
                update_selection((idx + 1) % apps_len);
            }
        })
    };

    SwitcherController {
        window,
        show_fn,
        hide_fn,
        next_fn,
    }
}
