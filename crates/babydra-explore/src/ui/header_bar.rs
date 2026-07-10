use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Orientation, Stack, Image, Label};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use babydra_common::SessionState;

pub struct HeaderBar {
    container: Box,
    btn_back: Button,
    btn_forward: Button,
    btn_up: Button,
    breadcrumb_box: Box,
    entry_address: Entry,
    address_stack: Stack,
    session: Rc<RefCell<SessionState>>,
    nav_callback: std::boxed::Box<dyn Fn(PathBuf)>,
}

impl HeaderBar {
    pub fn new(session: Rc<RefCell<SessionState>>, nav_callback: impl Fn(PathBuf) + 'static) -> Self {
        let container = Box::new(Orientation::Horizontal, 6);
        container.set_css_classes(&["header-bar"]);
        container.set_margin_top(6);
        container.set_margin_bottom(6);
        container.set_margin_start(6);
        container.set_margin_end(6);

        let btn_back = Button::from_icon_name("go-previous");
        let btn_forward = Button::from_icon_name("go-next");
        let btn_up = Button::from_icon_name("go-up");

        container.append(&btn_back);
        container.append(&btn_forward);
        container.append(&btn_up);

        // Address Bar Switcher (Breadcrumbs vs. Manual Text Entry)
        let address_stack = Stack::new();
        address_stack.set_hexpand(true);

        let breadcrumb_box = Box::new(Orientation::Horizontal, 2);
        address_stack.add_named(&breadcrumb_box, Some("breadcrumbs"));

        let entry_address = Entry::new();
        entry_address.set_hexpand(true);
        address_stack.add_named(&entry_address, Some("address"));

        container.append(&address_stack);

        // Add View Switcher placeholder buttons
        let btn_view_icons = Button::from_icon_name("view-grid");
        let btn_view_list = Button::from_icon_name("view-list");
        container.append(&btn_view_icons);
        container.append(&btn_view_list);

        let self_ = Self {
            container,
            btn_back,
            btn_forward,
            btn_up,
            breadcrumb_box,
            entry_address,
            address_stack,
            session,
            nav_callback: std::boxed::Box::new(nav_callback),
        };

        self_.setup_events();
        self_
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    fn setup_events(&self) {
        let session_clone = self.session.clone();
        let nav_cb = self.nav_callback.as_ref() as *const dyn Fn(PathBuf);
        let nav_cb = unsafe { &*nav_cb };

        self.btn_back.connect_clicked({
            let session = session_clone.clone();
            move |_| {
                let mut state = session.borrow_mut();
                if state.active_tab_mut().go_back() {
                    let path = state.active_tab().current_path.clone();
                    nav_cb(path);
                }
            }
        });

        self.btn_forward.connect_clicked({
            let session = session_clone.clone();
            move |_| {
                let mut state = session.borrow_mut();
                if state.active_tab_mut().go_forward() {
                    let path = state.active_tab().current_path.clone();
                    nav_cb(path);
                }
            }
        });

        self.btn_up.connect_clicked({
            let session = session_clone.clone();
            move |_| {
                let mut state = session.borrow_mut();
                if state.active_tab_mut().go_up() {
                    let path = state.active_tab().current_path.clone();
                    nav_cb(path);
                }
            }
        });

        // Toggle address bar manual typing when clicking empty space
        let stack = self.address_stack.clone();
        let entry = self.entry_address.clone();
        let session = self.session.clone();
        entry.connect_activate({
            let nav_cb = self.nav_callback.as_ref() as *const dyn Fn(PathBuf);
            let nav_cb = unsafe { &*nav_cb };
            move |entry| {
                let path_str = entry.text().to_string();
                let path = PathBuf::from(path_str);
                if path.exists() {
                    session.borrow_mut().active_tab_mut().navigate_to(path.clone());
                    nav_cb(path);
                    stack.set_visible_child_name("breadcrumbs");
                }
            }
        });
    }

    pub fn update(&self, path: &Path) {
        // Update Back/Forward button sensitivity
        let state = self.session.borrow();
        let tab = state.active_tab();
        self.btn_back.set_sensitive(tab.history_index > 0);
        self.btn_forward.set_sensitive(tab.history_index + 1 < tab.history.len());
        self.btn_up.set_sensitive(path.parent().is_some());

        // Update address text
        self.entry_address.set_text(&path.to_string_lossy());

        // Clear breadcrumbs
        while let Some(child) = self.breadcrumb_box.first_child() {
            self.breadcrumb_box.remove(&child);
        }

        // Build breadcrumbs
        let mut current = PathBuf::new();
        let components: Vec<_> = path.components().collect();

        for (i, comp) in components.iter().enumerate() {
            let comp_str = match comp {
                std::path::Component::RootDir => "/".to_string(),
                _ => comp.as_os_str().to_string_lossy().into_owned(),
            };

            current.push(comp);

            let btn = Button::builder()
                .label(&comp_str)
                .css_classes(vec!["flat".to_string()])
                .build();

            let target_path = current.clone();
            let nav_cb = self.nav_callback.as_ref() as *const dyn Fn(PathBuf);
            let nav_cb = unsafe { &*nav_cb };
            let session = self.session.clone();
            btn.connect_clicked(move |_| {
                session.borrow_mut().active_tab_mut().navigate_to(target_path.clone());
                nav_cb(target_path.clone());
            });

            self.breadcrumb_box.append(&btn);

            if i + 1 < components.len() {
                let sep = Label::new(Some("/"));
                sep.set_css_classes(&["dim-label"]);
                self.breadcrumb_box.append(&sep);
            }
        }
    }
}
