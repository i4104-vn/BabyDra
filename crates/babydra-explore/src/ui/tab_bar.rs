use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, Align};
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::SessionState;

pub struct TabBar {
    container: Box,
    session: Rc<RefCell<SessionState>>,
    on_tab_activated: Rc<dyn Fn(usize)>,
    on_tab_closed: Rc<dyn Fn(usize)>,
    on_tab_created: Rc<dyn Fn()>,
}

impl TabBar {
    pub fn new(
        session: Rc<RefCell<SessionState>>,
        on_tab_activated: impl Fn(usize) + 'static,
        on_tab_closed: impl Fn(usize) + 'static,
        on_tab_created: impl Fn() + 'static,
    ) -> Self {
        let container = Box::new(Orientation::Horizontal, 0);
        container.set_css_classes(&["tab-bar"]);

        let on_tab_activated_rc = Rc::new(on_tab_activated);
        let on_tab_closed_rc = Rc::new(on_tab_closed);
        let on_tab_created_rc = Rc::new(on_tab_created);

        let self_ = Self {
            container,
            session,
            on_tab_activated: on_tab_activated_rc,
            on_tab_closed: on_tab_closed_rc,
            on_tab_created: on_tab_created_rc,
        };

        self_.rebuild();

        self_
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    pub fn rebuild(&self) {
        // Clear existing tabs
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
        }

        let session = self.session.borrow();
        let active_idx = session.active_tab_index;

        // Render each tab
        for (idx, tab) in session.tabs.iter().enumerate() {
            let tab_box = Box::new(Orientation::Horizontal, 6);
            tab_box.set_valign(Align::Center);

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
            if session.tabs.len() > 1 {
                let btn_close = Button::builder()
                    .label("×")
                    .css_classes(vec!["nav-btn".to_string(), "flat".to_string()])
                    .build();
                btn_close.set_size_request(16, 16);
                btn_close.set_valign(Align::Center);
                
                let on_close = self.on_tab_closed.clone();
                btn_close.connect_clicked(move |_| {
                    on_close(idx);
                });
                tab_box.append(&btn_close);
            }

            let btn_tab = Button::builder()
                .child(&tab_box)
                .css_classes(vec!["tab-item".to_string(), "flat".to_string()])
                .build();

            if idx == active_idx {
                btn_tab.add_css_class("active-tab");
            }

            let on_activate = self.on_tab_activated.clone();
            btn_tab.connect_clicked(move |_| {
                on_activate(idx);
            });

            self.container.append(&btn_tab);
        }

        // New Tab Button (+)
        let btn_new = Button::builder()
            .label("+")
            .css_classes(vec!["tab-new-btn".to_string(), "flat".to_string()])
            .build();
        
        let on_created = self.on_tab_created.clone();
        btn_new.connect_clicked(move |_| {
            on_created();
        });
        self.container.append(&btn_new);
    }
}
