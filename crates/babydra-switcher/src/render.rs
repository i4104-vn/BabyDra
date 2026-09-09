use crate::widgets::render::build_apps_list;
use babydra_core::DesktopApp;
use babydra_core::{activate_app, save_history};
use babydra_ui_kit::ui::overlay::OverlayWindowComponents;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SwitcherController {
    pub window: gtk4::ApplicationWindow,
    pub show_fn: Box<dyn Fn()>,
    pub hide_fn: Box<dyn Fn()>,
    pub next_fn: Box<dyn Fn()>,
}

pub fn build_switcher_ui(app: &gtk4::Application) -> SwitcherController {
    let OverlayWindowComponents {
        window,
        deck_container,
        cards_row: list_container,
        meta_title,
        meta_subtitle,
        ..
    } = babydra_ui_kit::ui::overlay::create_overlay_window(app);

    let apps_state: Rc<RefCell<Vec<DesktopApp>>> = Rc::new(RefCell::new(Vec::new()));
    let buttons_state: Rc<RefCell<Vec<gtk4::Button>>> = Rc::new(RefCell::new(Vec::new()));
    let current_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let closed: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let alt_check_enabled: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let last_cycle_time: Rc<RefCell<std::time::Instant>> =
        Rc::new(RefCell::new(std::time::Instant::now()));

    let update_selection = {
        let current_index = current_index.clone();
        let buttons_state = buttons_state.clone();
        let apps_state = apps_state.clone();
        let meta_title_c = meta_title.clone();
        let meta_subtitle_c = meta_subtitle.clone();
        Rc::new(move |new_idx: usize| {
            let buttons = buttons_state.borrow();
            let apps = apps_state.borrow();
            if buttons.is_empty() {
                return;
            }
            let idx = new_idx % buttons.len();
            *current_index.borrow_mut() = idx;

            for (i, btn) in buttons.iter().enumerate() {
                if i == idx {
                    btn.add_css_class("selected");
                    btn.grab_focus();
                } else {
                    btn.remove_css_class("selected");
                }
            }

            if idx < apps.len() {
                let app = &apps[idx];
                meta_title_c.set_text(&app.name);
                match app.window_title.as_deref() {
                    Some(title) if title != app.name && !title.is_empty() => {
                        meta_subtitle_c.set_text(title);
                        meta_subtitle_c.set_visible(true);
                    }
                    _ => {
                        meta_subtitle_c.set_text("");
                        meta_subtitle_c.set_visible(false);
                    }
                }
            }
        })
    };

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
                gtk4::gdk::Key::Tab | gtk4::gdk::Key::Right | gtk4::gdk::Key::Down => {
                    if now.duration_since(*last_cycle_key.borrow())
                        < std::time::Duration::from_millis(150)
                    {
                        return gtk4::glib::Propagation::Stop;
                    }
                    *last_cycle_key.borrow_mut() = now;
                    update_sel_key((idx + 1) % apps_len);
                    gtk4::glib::Propagation::Stop
                }
                gtk4::gdk::Key::ISO_Left_Tab | gtk4::gdk::Key::Left | gtk4::gdk::Key::Up => {
                    if now.duration_since(*last_cycle_key.borrow())
                        < std::time::Duration::from_millis(150)
                    {
                        return gtk4::glib::Propagation::Stop;
                    }
                    *last_cycle_key.borrow_mut() = now;
                    let prev = if idx == 0 { apps_len - 1 } else { idx - 1 };
                    update_sel_key(prev);
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

    let scroll_controller = gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let sel_for_scroll = update_selection.clone();
    let curr_for_scroll = current_index.clone();
    let apps_for_scroll = apps_state.clone();
    let last_cycle_scroll = last_cycle_time.clone();
    scroll_controller.connect_scroll(move |_, _, dy| {
        let apps_len = apps_for_scroll.borrow().len();
        if apps_len == 0 {
            return gtk4::glib::Propagation::Proceed;
        }
        let now = std::time::Instant::now();
        if now.duration_since(*last_cycle_scroll.borrow()) < std::time::Duration::from_millis(100) {
            return gtk4::glib::Propagation::Stop;
        }
        *last_cycle_scroll.borrow_mut() = now;

        let idx = *curr_for_scroll.borrow();
        if dy > 0.0 {
            sel_for_scroll((idx + 1) % apps_len);
        } else if dy < 0.0 {
            let prev = if idx == 0 { apps_len - 1 } else { idx - 1 };
            sel_for_scroll(prev);
        }
        gtk4::glib::Propagation::Stop
    });
    deck_container.add_controller(scroll_controller);

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

            let display = gtk4::gdk::Display::default();
            if let Some(seat) = display.as_ref().and_then(|d| d.default_seat()) {
                if let Some(device) = seat.keyboard() {
                    let modifier_type = device.modifier_state();
                    if !modifier_type.contains(gtk4::gdk::ModifierType::ALT_MASK) {
                        *closed_poll.borrow_mut() = true;
                        do_activate_poll();
                    }
                }
            }
            gtk4::glib::ControlFlow::Continue
        });
    }

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
            *last_cycle_show.borrow_mut() = std::time::Instant::now();

            let apps = babydra_core::get_running_apps();
            if apps.is_empty() {
                window.set_visible(false);
                return;
            }

            while let Some(child) = list_container.first_child() {
                list_container.remove(&child);
            }

            let (cards_row, item_buttons) = build_apps_list(&apps);

            for (i, btn) in item_buttons.iter().enumerate() {
                let update_sel = update_selection.clone();
                let window_close = window.clone();
                let apps_click = apps.clone();
                let closed_click = closed.clone();
                btn.connect_clicked(move |_| {
                    update_sel(i);
                    let app_item = apps_click[i].clone();
                    save_history(app_item.window_title.as_deref().unwrap_or(&app_item.name));
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

            list_container.append(&cards_row);

            *apps_state.borrow_mut() = apps.clone();
            *buttons_state.borrow_mut() = item_buttons;

            let initial_idx = if apps.len() > 1 { 1 } else { 0 };
            *current_index.borrow_mut() = initial_idx;
            update_selection(initial_idx);

            window.set_visible(true);
            window.present();

            let alt_check = alt_check_enabled.clone();
            gtk4::glib::timeout_add_local_once(
                std::time::Duration::from_millis(60),
                move || {
                    *alt_check.borrow_mut() = true;
                },
            );
        })
    };

    let hide_fn = {
        let window = window.clone();
        Box::new(move || {
            window.set_visible(false);
        })
    };

    let next_fn = {
        let update_selection = update_selection.clone();
        let current_index = current_index.clone();
        let apps_state = apps_state.clone();
        let last_cycle_next = last_cycle_time.clone();
        Box::new(move || {
            let now = std::time::Instant::now();
            if now.duration_since(*last_cycle_next.borrow()) < std::time::Duration::from_millis(150)
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
