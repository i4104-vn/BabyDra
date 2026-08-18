//! Tab bar widget and its window integration (build, rebuild, navigation wiring).

use babydra_core::{ActivePane, SessionState};
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Orientation};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

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
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }

    let session_borrow = session.borrow();
    let active_idx = session_borrow.active_tab_index;

    for (idx, tab) in session_borrow.tabs.iter().enumerate() {
        let tab_box = Box::new(Orientation::Horizontal, 6);
        tab_box.set_valign(Align::Center);
        tab_box.set_hexpand(false);
        tab_box.set_halign(Align::Start);
        tab_box.set_css_classes(&["tab-item"]);

        if idx == active_idx {
            tab_box.add_css_class("active-tab");
        }

        let display_name = tab
            .current_path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        let lbl = Label::new(Some(&display_name));
        lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        lbl.set_max_width_chars(15);
        lbl.set_hexpand(true);
        lbl.set_halign(Align::Start);
        tab_box.append(&lbl);

        // Close button (only shown if there is more than 1 tab)
        if session_borrow.tabs.len() > 1 {
            let btn_close = babydra_ui_kit::components::create_button("×");
            btn_close.remove_css_class("baby-button");
            btn_close.add_css_class("tab-close-btn");
            btn_close.add_css_class("flat");
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
    let btn_new = babydra_ui_kit::components::create_button("+");
    btn_new.remove_css_class("baby-button");
    btn_new.add_css_class("tab-new-btn");
    btn_new.add_css_class("flat");

    let on_created = on_tab_created.clone();
    btn_new.connect_clicked(move |_| {
        on_created();
    });
    container.append(&btn_new);
}

/// Initializes the tab bar widget, mounts it to the window box, and sets up
/// recursive rebuild/navigation callbacks.
pub fn setup_tab_bar(
    vbox: &gtk4::Box,
    session: Rc<RefCell<SessionState>>,
    nav: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    tab_bar_box: Rc<RefCell<Option<gtk4::Box>>>,
    rebuild_tabs_cell: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> Rc<dyn Fn()> {
    let session_c = session.clone();
    let nav_c = nav.clone();

    let rebuild_tabs = {
        let session = session.clone();
        let tab_bar_box = tab_bar_box.clone();
        let nav = nav.clone();
        move || {
            if let Some(ref tbb) = *tab_bar_box.borrow() {
                let rebuild_tabs_c = Rc::new(RefCell::new(None::<std::boxed::Box<dyn Fn()>>));
                let rebuild_tabs_c_clone = rebuild_tabs_c.clone();

                let on_tab_activated = {
                    let session = session.clone();
                    let nav = nav.clone();
                    let rebuild = rebuild_tabs_c_clone.clone();
                    move |idx: usize| {
                        {
                            let mut sess = session.borrow_mut();
                            sess.active_tab_index = idx;
                        }
                        let path = session.borrow().active_tab().current_path.clone();
                        if let Some(ref f) = *nav.borrow() {
                            f(ActivePane::Left, path);
                        }
                        if let Some(ref reb) = *rebuild.borrow() {
                            reb();
                        }
                    }
                };

                let on_tab_closed = {
                    let session = session.clone();
                    let nav = nav.clone();
                    let rebuild = rebuild_tabs_c_clone.clone();
                    move |idx: usize| {
                        let should_navigate = {
                            let mut sess = session.borrow_mut();
                            let is_active = sess.active_tab_index == idx;
                            let closed = sess.close_tab(idx);
                            closed && is_active
                        };
                        if should_navigate {
                            let path = session.borrow().active_tab().current_path.clone();
                            if let Some(ref f) = *nav.borrow() {
                                f(ActivePane::Left, path);
                            }
                        }
                        if let Some(ref reb) = *rebuild.borrow() {
                            reb();
                        }
                    }
                };

                let on_tab_created = {
                    let session = session.clone();
                    let nav = nav.clone();
                    let rebuild = rebuild_tabs_c_clone.clone();
                    move || {
                        let home = glib::home_dir();
                        {
                            let mut sess = session.borrow_mut();
                            sess.add_tab(home.clone());
                        }
                        if let Some(ref f) = *nav.borrow() {
                            f(ActivePane::Left, home);
                        }
                        if let Some(ref reb) = *rebuild.borrow() {
                            reb();
                        }
                    }
                };

                let tbb_c = tbb.clone();
                let sess_c = session.clone();
                let on_act: Rc<dyn Fn(usize)> = Rc::new(on_tab_activated);
                let on_cls: Rc<dyn Fn(usize)> = Rc::new(on_tab_closed);
                let on_cre: Rc<dyn Fn()> = Rc::new(on_tab_created);
                *rebuild_tabs_c.borrow_mut() = Some(std::boxed::Box::new(move || {
                    rebuild_tab_bar(&tbb_c, &sess_c, &on_act, &on_cls, &on_cre);
                }) as std::boxed::Box<dyn Fn()>);

                let borrow = rebuild_tabs_c.borrow();
                if let Some(ref reb) = *borrow {
                    reb();
                }
            }
        }
    };

    let rebuild_tabs_rc = Rc::new(rebuild_tabs);
    rebuild_tabs_cell.replace(Some(rebuild_tabs_rc.clone()));

    // Initial TabBar creation
    {
        let session_c = session_c.clone();
        let nav_c = nav_c.clone();
        let rebuild_tabs_c = rebuild_tabs_rc.clone();
        let tab_bar = create_tab_bar(
            session_c,
            {
                let rebuild = rebuild_tabs_c.clone();
                let nav = nav_c.clone();
                move |_idx| {
                    if let Some(ref f) = *nav.borrow() {
                        f(ActivePane::Left, PathBuf::from("/"));
                    }
                    rebuild();
                }
            },
            {
                let rebuild = rebuild_tabs_c.clone();
                move |_idx| {
                    rebuild();
                }
            },
            {
                let rebuild = rebuild_tabs_c.clone();
                move || {
                    rebuild();
                }
            },
        );
        vbox.prepend(&tab_bar);
        tab_bar.add_css_class("tab-bar-container");
        tab_bar_box.replace(Some(tab_bar));
        rebuild_tabs_rc();
    }

    rebuild_tabs_rc
}
