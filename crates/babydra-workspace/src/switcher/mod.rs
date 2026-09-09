pub mod card;
pub mod events;
pub mod window;

use crate::manager::{
    get_current_workspace, get_workspaces, next_workspace, prev_workspace, switch_workspace,
};
use card::build_workspace_card;
use events::{attach_dismiss_gesture, attach_key_controller, attach_scroll_controller};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use window::create_switcher_window;

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
    let components = create_switcher_window(app);
    let window = components.window;
    let overlay_box = components.overlay_box;
    let deck_container = components.deck_container;
    let cards_row = components.cards_row;

    // Shared state
    let current_idx: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let card_buttons: Rc<RefCell<Vec<gtk4::Button>>> = Rc::new(RefCell::new(Vec::new()));

    // Dismiss when clicking outside the centered deck
    attach_dismiss_gesture(&overlay_box, &deck_container, &window);

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

                let win_c = window.clone();
                let btn = build_workspace_card(ws, move |target_id| {
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

    // Attach event controllers
    attach_key_controller(&window, current_idx.clone(), do_activate, update_selection.clone());
    attach_scroll_controller(&deck_container, current_idx, update_selection);

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
