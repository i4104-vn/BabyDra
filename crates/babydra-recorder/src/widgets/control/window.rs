//! Main window assembler for the recorder application.

use super::actions::build_actions_section;
use super::header::build_header;
use super::mode::build_mode_card;
use super::quality::build_quality_card;
use super::state::ControlWindowState;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

/// Presents the recorder window or creates it if not currently open.
pub fn show_or_create_control_window(
    app: &gtk4::Application,
    window_holder: Rc<RefCell<Option<ApplicationWindow>>>,
) {
    if let Some(ref win) = *window_holder.borrow() {
        win.present();
        return;
    }

    let window = ApplicationWindow::new(app);
    window.set_title(Some(&babydra_core::i18n::trans("recorder.title")));
    window.set_default_size(440, 540);
    window.set_resizable(false);
    window.add_css_class("recorder-window");

    let state = Rc::new(RefCell::new(ControlWindowState::default()));

    let main_box = GtkBox::new(Orientation::Vertical, 16);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    // 1. Header
    let header = build_header();
    main_box.append(&header);

    // 2. Mode Selector Card
    let mode_card = build_mode_card(&window, state.clone());
    main_box.append(&mode_card);

    // 3. Quality Settings Card
    let quality_card = build_quality_card(state.clone());
    main_box.append(&quality_card);

    // 4. Timer & Actions Section
    let (actions_box, _, _) = build_actions_section(state);
    main_box.append(&actions_box);

    // Clear window holder on close
    {
        let window_holder_c = window_holder.clone();
        window.connect_close_request(move |win| {
            win.set_visible(false);
            *window_holder_c.borrow_mut() = None;
            glib::Propagation::Proceed
        });
    }

    window.set_child(Some(&main_box));
    *window_holder.borrow_mut() = Some(window.clone());
    window.present();
}
