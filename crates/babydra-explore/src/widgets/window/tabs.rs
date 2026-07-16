use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use babydra_common::{SessionState, ActivePane};

/// Initializes the tab bar widget, mounts it to the box, and sets up recursive rebuild/navigation callbacks.
pub fn setup_tab_bar(
    vbox: &gtk4::Box,
    session: Rc<RefCell<SessionState>>,
    nav: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    tab_bar_box: Rc<RefCell<Option<gtk4::Box>>>,
) -> Rc<dyn Fn()> {
    let session_c = session.clone();
    let nav_c = nav.clone();

    let rebuild_tabs = {
        let session = session.clone();
        let tab_bar_box = tab_bar_box.clone();
        let nav = nav.clone();
        move || {
            if let Some(ref tbb) = *tab_bar_box.borrow() {
                let rebuild_tabs_c = Rc::new(RefCell::new(None::<Box<dyn Fn()>>));
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
                *rebuild_tabs_c.borrow_mut() = Some(Box::new(move || {
                    crate::widgets::tab_bar::rebuild_tab_bar(
                        &tbb_c,
                        &sess_c,
                        &on_act,
                        &on_cls,
                        &on_cre,
                    );
                }) as Box<dyn Fn()>);

                let borrow = rebuild_tabs_c.borrow();
                if let Some(ref reb) = *borrow {
                    reb();
                }
            }
        }
    };

    let rebuild_tabs_rc = Rc::new(rebuild_tabs);

    // Initial TabBar creation
    {
        let session_c = session_c.clone();
        let nav_c = nav_c.clone();
        let rebuild_tabs_c = rebuild_tabs_rc.clone();
        let tab_bar = crate::widgets::tab_bar::create_tab_bar(
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
            }
        );
        vbox.prepend(&tab_bar);
        tab_bar.add_css_class("tab-bar-container");
        tab_bar_box.replace(Some(tab_bar));
        rebuild_tabs_rc();
    }

    rebuild_tabs_rc
}
