use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, Align};
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::SessionState;

/// Creates a tab bar box widget and schedules its initial build.
pub fn create_tab_bar(
    session: Rc<RefCell<SessionState>>,
    on_tab_activated: impl Fn(usize) + 'static,
    on_tab_closed: impl Fn(usize) + 'static,
    on_tab_created: impl Fn() + 'static,
) -> Box {
    let container = Box::new(Orientation::Horizontal, 0);
    container.set_css_classes(&["tab-bar"]);

    let on_tab_activated_rc = Rc::new(on_tab_activated) as Rc<dyn Fn(usize)>;
    let on_tab_closed_rc = Rc::new(on_tab_closed) as Rc<dyn Fn(usize)>;
    let on_tab_created_rc = Rc::new(on_tab_created) as Rc<dyn Fn()>;

    rebuild_tab_bar(
        &container,
        &session,
        &on_tab_activated_rc,
        &on_tab_closed_rc,
        &on_tab_created_rc,
    );

    container
}

/// Clears and repopulates the tab bar box with tab elements.
pub fn rebuild_tab_bar(
    container: &Box,
    session: &Rc<RefCell<SessionState>>,
    on_tab_activated: &Rc<dyn Fn(usize)>,
    on_tab_closed: &Rc<dyn Fn(usize)>,
    on_tab_created: &Rc<dyn Fn()>,
) {
    // Clear existing tabs
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }

    let session_borrow = session.borrow();
    let active_idx = session_borrow.active_tab_index;

    // Render each tab
    for (idx, tab) in session_borrow.tabs.iter().enumerate() {
        let tab_box = Box::new(Orientation::Horizontal, 6);
        tab_box.set_valign(Align::Center);
        tab_box.set_css_classes(&["tab-item"]);

        if idx == active_idx {
            tab_box.add_css_class("active-tab");
        }

        let display_name = if tab.current_path == glib::home_dir() {
            "Home".to_string()
        } else {
            tab.current_path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "/".to_string())
        };

        let lbl = Label::new(Some(&display_name));
        lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        lbl.set_max_width_chars(15);
        tab_box.append(&lbl);

        // Close button (only shown if there is more than 1 tab)
        if session_borrow.tabs.len() > 1 {
            let btn_close = Button::builder()
                .label("×")
                .css_classes(vec!["tab-close-btn".to_string(), "flat".to_string()])
                .build();
            btn_close.set_valign(Align::Center);
            
            let on_close = on_tab_closed.clone();
            btn_close.connect_clicked(move |_| {
                on_close(idx);
            });
            tab_box.append(&btn_close);
        }

        // Gesture to activate the tab on click (handles clicking anywhere on the tab)
        let gesture = gtk4::GestureClick::new();
        let on_activate = on_tab_activated.clone();
        gesture.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            on_activate(idx);
        });
        tab_box.add_controller(gesture);

        container.append(&tab_box);
    }

    // New Tab Button (+)
    let btn_new = Button::builder()
        .label("+")
        .css_classes(vec!["tab-new-btn".to_string(), "flat".to_string()])
        .build();
    
    let on_created = on_tab_created.clone();
    btn_new.connect_clicked(move |_| {
        on_created();
    });
    container.append(&btn_new);
}
