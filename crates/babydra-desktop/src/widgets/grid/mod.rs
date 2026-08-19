//! Fixed icon grid container and interaction manager for the desktop surface.

pub mod dnd;
mod keyboard;
mod render;
mod watcher;

pub use dnd::{create_desktop_drop, create_folder_drop, create_icon_drag};

use crate::state::DesktopState;
use crate::widgets::context_menu::show_empty_menu;
use crate::widgets::selection::{attach_rubberband, update_icon_sel};
use gtk4::prelude::*;
use gtk4::{Box, Fixed, GestureClick};
use std::cell::RefCell;
use std::rc::Rc;

fn make_refresh_cb(
    fixed: &Fixed,
    state: &Rc<RefCell<DesktopState>>,
    parent_window: &gtk4::ApplicationWindow,
    rubberband: &Box,
) -> Rc<dyn Fn()> {
    let fixed_c = fixed.clone();
    let state_c = state.clone();
    let parent_win_c = parent_window.clone();
    let rubberband_c = rubberband.clone();

    Rc::new(move || {
        let f = fixed_c.clone();
        let s = state_c.clone();
        let p = parent_win_c.clone();
        let r = rubberband_c.clone();
        glib::spawn_future_local(async move {
            let sort_by = s.borrow().config.sort_by.clone();
            let new_entries =
                babydra_core::models::shell::desktop_state::DesktopState::fetch_entries(&sort_by)
                    .await;
            s.borrow_mut().update_entries(new_entries);
            render::rebuild_grid_icons(&f, &s, &p, &r);
        });
    })
}

fn make_refresh_positions_cb(
    fixed: &Fixed,
    state: &Rc<RefCell<DesktopState>>,
    parent_window: &gtk4::ApplicationWindow,
    rubberband: &Box,
) -> Rc<dyn Fn()> {
    let fixed_c = fixed.clone();
    let state_c = state.clone();
    let parent_win_c = parent_window.clone();
    let rubberband_c = rubberband.clone();
    Rc::new(move || {
        render::rebuild_grid_icons(&fixed_c, &state_c, &parent_win_c, &rubberband_c);
    })
}

/// Builds the desktop grid fixed layout, attaching gestures, keyboard shortcuts, and file watching.
pub fn create_desktop_grid(
    parent_window: &gtk4::ApplicationWindow,
) -> (Fixed, Rc<RefCell<DesktopState>>, Rc<dyn Fn()>) {
    let fixed = Fixed::new();
    fixed.set_hexpand(true);
    fixed.set_vexpand(true);
    fixed.set_focusable(true);
    fixed.set_can_focus(true);
    fixed.add_css_class("desktop-grid");

    let state = Rc::new(RefCell::new(DesktopState::new()));

    let rubberband = Box::new(gtk4::Orientation::Vertical, 0);
    rubberband.add_css_class("desktop-rubberband");
    rubberband.set_visible(false);
    fixed.put(&rubberband, 0.0, 0.0);

    let refresh_fn = make_refresh_cb(&fixed, &state, parent_window, &rubberband);

    // Left click: deselect + focus
    let bg_click = GestureClick::new();
    bg_click.set_button(1);
    let state_bg = state.clone();
    let fixed_bg = fixed.clone();
    let rubberband_bg = rubberband.clone();
    bg_click.connect_pressed(move |_, _, _, _| {
        fixed_bg.grab_focus();
        state_bg.borrow_mut().clear_selection();
        update_icon_sel(&fixed_bg, &state_bg, &rubberband_bg);
    });
    fixed.add_controller(bg_click);

    // Right click: empty context menu
    let bg_right_click = GestureClick::new();
    bg_right_click.set_button(3);
    let parent_win_rc = parent_window.clone();
    let fixed_right = fixed.clone();
    let ref_cb_right = refresh_fn.clone();
    bg_right_click.connect_pressed(move |_, _, x, y| {
        fixed_right.grab_focus();
        show_empty_menu(
            fixed_right.upcast_ref::<gtk4::Widget>(),
            x,
            y,
            ref_cb_right.clone(),
            &parent_win_rc,
        );
    });
    fixed.add_controller(bg_right_click);

    // Desktop drop target
    let refresh_pos_fn = make_refresh_positions_cb(&fixed, &state, parent_window, &rubberband);
    let drop_target = create_desktop_drop(state.clone(), refresh_pos_fn.clone());
    fixed.add_controller(drop_target);

    // Rubberband selection
    attach_rubberband(&fixed, state.clone(), rubberband.clone());

    // Keyboard shortcuts
    keyboard::wire_keyboard(
        &fixed,
        state.clone(),
        rubberband.clone(),
        refresh_fn.clone(),
    );

    // Initial load + file watcher
    refresh_fn();
    watcher::start_file_watcher(refresh_fn.clone());

    (fixed, state, refresh_fn)
}
