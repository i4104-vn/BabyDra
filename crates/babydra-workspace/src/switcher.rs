use crate::manager::{
    get_current_workspace, get_workspaces, next_workspace, prev_workspace, switch_workspace,
    DEFAULT_WORKSPACE_COUNT,
};
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::cell::RefCell;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::rc::Rc;

pub const WORKSPACE_SOCKET_PATH: &str = "/tmp/babydra-workspace.socket";

/// Tries to signal an existing running workspace daemon.
pub fn try_signal_daemon(msg: &[u8]) -> bool {
    if let Ok(mut stream) = UnixStream::connect(WORKSPACE_SOCKET_PATH) {
        let _ = stream.write_all(msg);
        return true;
    }
    false
}

/// Controller handle for the workspace switcher window.
pub struct WorkspaceSwitcherController {
    pub window: gtk4::ApplicationWindow,
    pub show_fn: Box<dyn Fn()>,
    pub hide_fn: Box<dyn Fn()>,
    pub next_fn: Box<dyn Fn()>,
    pub prev_fn: Box<dyn Fn()>,
}

/// Builds the centered workspace switcher overlay UI.
pub fn build_workspace_switcher_ui(app: &gtk4::Application) -> WorkspaceSwitcherController {
    babydra_ui_kit::ui::theme::init_theme();

    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);
    babydra_ui_kit::ui::window::init_layer_window(
        &window,
        Layer::Overlay,
        KeyboardMode::Exclusive,
        -1,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        0,
        None,
    );
    window.add_css_class("switcher-window");
    window.add_css_class("workspace-switcher-window");

    // Fullscreen centered backdrop container
    let overlay_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    overlay_box.set_valign(gtk4::Align::Center);
    overlay_box.set_halign(gtk4::Align::Center);

    // Centered floating deck container
    let deck_container = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    deck_container.add_css_class("switcher-deck-container");
    deck_container.add_css_class("workspace-deck-container");
    deck_container.set_valign(gtk4::Align::Center);
    deck_container.set_halign(gtk4::Align::Center);

    // Header label
    let header_lbl = gtk4::Label::new(Some("Workspaces"));
    header_lbl.add_css_class("switcher-deck-header");
    deck_container.append(&header_lbl);

    // Cards row (horizontal stacked/spread card deck)
    let cards_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    cards_row.add_css_class("switcher-card-deck");
    cards_row.set_halign(gtk4::Align::Center);
    deck_container.append(&cards_row);

    // Hint label footer
    let hint_lbl = gtk4::Label::new(Some("Scroll or use Arrow keys to navigate • Enter to select"));
    hint_lbl.add_css_class("switcher-deck-hint");
    deck_container.append(&hint_lbl);

    overlay_box.append(&deck_container);
    window.set_child(Some(&overlay_box));

    // Shared state
    let current_idx: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let card_buttons: Rc<RefCell<Vec<gtk4::Button>>> = Rc::new(RefCell::new(Vec::new()));

    // Dismiss when clicking outside the centered deck
    let click_gesture = gtk4::GestureClick::new();
    let win_dismiss = window.clone();
    let deck_ref = deck_container.clone();
    click_gesture.connect_pressed(move |gesture, _, x, y| {
        let (deck_x, deck_y) = (deck_ref.allocated_width(), deck_ref.allocated_height());
        let (alloc_x, alloc_y) = (deck_ref.allocation().x() as f64, deck_ref.allocation().y() as f64);
        let inside = x >= alloc_x && x <= alloc_x + deck_x as f64 && y >= alloc_y && y <= alloc_y + deck_y as f64;
        if !inside {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            win_dismiss.set_visible(false);
        }
    });
    overlay_box.add_controller(click_gesture);

    // Update selection highlight
    let update_selection = {
        let current_idx = current_idx.clone();
        let card_buttons = card_buttons.clone();
        Rc::new(move |new_idx: usize| {
            let btns = card_buttons.borrow();
            if btns.is_empty() {
                return;
            }
            let idx = new_idx % btns.len();
            *current_idx.borrow_mut() = idx;
            for (i, btn) in btns.iter().enumerate() {
                if i == idx {
                    btn.add_css_class("selected");
                    btn.grab_focus();
                } else {
                    btn.remove_css_class("selected");
                }
            }
        })
    };

    // Activate selected workspace and hide window
    let do_activate = {
        let current_idx = current_idx.clone();
        let window = window.clone();
        Rc::new(move || {
            let idx = *current_idx.borrow();
            let target_id = (idx as u32) + 1;
            switch_workspace(target_id);
            window.set_visible(false);
        })
    };

    // Rebuild cards function
    let rebuild_cards = {
        let cards_row = cards_row.clone();
        let card_buttons = card_buttons.clone();
        let window = window.clone();
        let current_idx = current_idx.clone();
        let update_sel = update_selection.clone();
        Rc::new(move || {
            while let Some(child) = cards_row.first_child() {
                cards_row.remove(&child);
            }
            let mut btns = Vec::new();
            let workspaces = get_workspaces();
            let active_id = get_current_workspace();
            let mut active_index = 0;

            for (i, ws) in workspaces.iter().enumerate() {
                if ws.id == active_id {
                    active_index = i;
                }

                let btn = gtk4::Button::new();
                btn.add_css_class("switcher-card");
                btn.add_css_class("workspace-card");
                if ws.is_active {
                    btn.add_css_class("active");
                }

                let card_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
                card_box.set_valign(gtk4::Align::Center);
                card_box.set_halign(gtk4::Align::Center);
                card_box.set_size_request(160, 120);

                // Number badge
                let num_lbl = gtk4::Label::new(Some(&ws.id.to_string()));
                num_lbl.add_css_class("workspace-card-number");
                card_box.append(&num_lbl);

                // Title
                let title_lbl = gtk4::Label::new(Some(&ws.name));
                title_lbl.add_css_class("workspace-card-title");
                card_box.append(&title_lbl);

                // Active dot indicator
                let dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                dot.add_css_class("workspace-card-dot");
                if ws.is_active {
                    dot.add_css_class("active");
                }
                card_box.append(&dot);

                btn.set_child(Some(&card_box));

                let target_id = ws.id;
                let win_c = window.clone();
                btn.connect_clicked(move |_| {
                    switch_workspace(target_id);
                    win_c.set_visible(false);
                });

                cards_row.append(&btn);
                btns.push(btn);
            }

            *card_buttons.borrow_mut() = btns;
            *current_idx.borrow_mut() = active_index;
            update_sel(active_index);
        })
    };

    // Keyboard controller
    let key_controller = gtk4::EventControllerKey::new();
    let sel_for_key = update_selection.clone();
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
                let next = if idx == 0 { (DEFAULT_WORKSPACE_COUNT as usize) - 1 } else { idx - 1 };
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

    // Scroll controller on deck
    let scroll_controller = gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let sel_for_scroll = update_selection.clone();
    let curr_for_scroll = current_idx.clone();
    scroll_controller.connect_scroll(move |_, _, dy| {
        let idx = *curr_for_scroll.borrow();
        if dy > 0.0 {
            sel_for_scroll((idx + 1) % (DEFAULT_WORKSPACE_COUNT as usize));
        } else if dy < 0.0 {
            let prev = if idx == 0 { (DEFAULT_WORKSPACE_COUNT as usize) - 1 } else { idx - 1 };
            sel_for_scroll(prev);
        }
        gtk4::glib::Propagation::Stop
    });
    deck_container.add_controller(scroll_controller);

    // Controller callbacks
    let win_show = window.clone();
    let rebuild_show = rebuild_cards.clone();
    let show_fn = Box::new(move || {
        rebuild_show();
        win_show.set_visible(true);
        win_show.present();
    });

    let win_hide = window.clone();
    let hide_fn = Box::new(move || {
        win_hide.set_visible(false);
    });

    let next_fn = Box::new(move || {
        next_workspace();
    });

    let prev_fn = Box::new(move || {
        prev_workspace();
    });

    WorkspaceSwitcherController {
        window,
        show_fn,
        hide_fn,
        next_fn,
        prev_fn,
    }
}
