use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation, Stack, Separator};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use babydra_common::SessionState;

pub struct HeaderBar {
    container: Box,
    // Navigation row
    btn_back: Button,
    btn_forward: Button,
    btn_up: Button,
    // Address bar
    breadcrumb_box: Box,
    entry_address: Entry,
    address_stack: Stack,
    address_wrap: Box,
    // State
    session: Rc<RefCell<SessionState>>,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    view_mode_callback: Rc<dyn Fn(String)>,
}

impl HeaderBar {
    pub fn new(
        session: Rc<RefCell<SessionState>>,
        nav_callback: impl Fn(PathBuf) + 'static,
        view_mode_callback: impl Fn(String) + 'static,
    ) -> Rc<RefCell<Self>> {
        let container = Box::new(Orientation::Vertical, 0);

        // ── Row 1: Navigation Bar ──────────────────────────────────
        let nav_row = Box::new(Orientation::Horizontal, 4);
        nav_row.set_css_classes(&["nav-bar"]);
        nav_row.set_margin_start(6);
        nav_row.set_margin_end(6);
        container.append(&nav_row);

        let btn_back    = Button::from_icon_name("go-previous-symbolic");
        let btn_forward = Button::from_icon_name("go-next-symbolic");
        let btn_up      = Button::from_icon_name("go-up-symbolic");
        let btn_refresh = Button::from_icon_name("view-refresh-symbolic");

        for btn in &[&btn_back, &btn_forward, &btn_up, &btn_refresh] {
            btn.set_css_classes(&["nav-btn"]);
        }

        nav_row.append(&btn_back);
        nav_row.append(&btn_forward);
        nav_row.append(&btn_up);
        nav_row.append(&btn_refresh);

        // Address bar wrapper
        let address_wrap = Box::new(Orientation::Horizontal, 0);
        address_wrap.set_css_classes(&["address-bar-wrap"]);
        address_wrap.set_hexpand(true);
        address_wrap.set_valign(gtk4::Align::Center);

        let address_stack = Stack::new();
        address_stack.set_hexpand(true);

        let breadcrumb_box = Box::new(Orientation::Horizontal, 2);
        breadcrumb_box.set_valign(gtk4::Align::Center);
        address_stack.add_named(&breadcrumb_box, Some("breadcrumbs"));

        let entry_address = Entry::new();
        entry_address.set_hexpand(true);
        entry_address.set_css_classes(&["address-entry"]);
        address_stack.add_named(&entry_address, Some("address"));

        address_wrap.append(&address_stack);
        nav_row.append(&address_wrap);

        // Search entry
        let search = Entry::builder()
            .placeholder_text("Search")
            .primary_icon_name("system-search-symbolic")
            .css_classes(vec!["search-entry".to_string()])
            .build();
        search.set_size_request(200, -1);
        nav_row.append(&search);

        // ── Row 2: Command Toolbar ─────────────────────────────────
        let toolbar = Box::new(Orientation::Horizontal, 2);
        toolbar.set_css_classes(&["toolbar"]);
        toolbar.set_margin_start(6);
        toolbar.set_margin_end(6);
        container.append(&toolbar);

        let btn_new_folder   = Button::with_label("⊕ New Folder");
        let btn_cut          = Button::from_icon_name("edit-cut-symbolic");
        let btn_copy         = Button::from_icon_name("edit-copy-symbolic");
        let btn_paste        = Button::from_icon_name("edit-paste-symbolic");
        let btn_rename       = Button::from_icon_name("edit-rename-symbolic");
        let btn_delete       = Button::from_icon_name("edit-delete-symbolic");
        let sep1 = Separator::new(Orientation::Vertical);
        sep1.set_css_classes(&["toolbar-sep"]);
        let sep2 = Separator::new(Orientation::Vertical);
        sep2.set_css_classes(&["toolbar-sep"]);
        let btn_view_icons   = Button::from_icon_name("view-grid-symbolic");
        let btn_view_list    = Button::from_icon_name("view-list-symbolic");

        btn_new_folder.set_css_classes(&["toolbar-btn", "new-btn"]);
        for btn in &[&btn_cut, &btn_copy, &btn_paste, &btn_rename, &btn_delete,
                     &btn_view_icons, &btn_view_list] {
            btn.set_css_classes(&["toolbar-btn"]);
        }

        toolbar.append(&btn_new_folder);
        toolbar.append(&sep1);
        toolbar.append(&btn_cut);
        toolbar.append(&btn_copy);
        toolbar.append(&btn_paste);
        toolbar.append(&sep2);
        toolbar.append(&btn_rename);
        toolbar.append(&btn_delete);

        // push view toggle to the right
        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        toolbar.append(&spacer);
        toolbar.append(&btn_view_icons);
        toolbar.append(&btn_view_list);

        let view_mode_cb = Rc::new(view_mode_callback);
        let nav_cb       = Rc::new(nav_callback);

        let self_ = Rc::new(RefCell::new(Self {
            container,
            btn_back,
            btn_forward,
            btn_up,
            breadcrumb_box,
            entry_address,
            address_stack,
            address_wrap,
            session,
            nav_callback: nav_cb.clone(),
            view_mode_callback: view_mode_cb.clone(),
        }));

        // View toggle wiring
        btn_view_icons.connect_clicked({
            let cb = view_mode_cb.clone();
            move |_| { cb("icons".to_string()); }
        });
        btn_view_list.connect_clicked({
            let cb = view_mode_cb.clone();
            move |_| { cb("list".to_string()); }
        });

        // Navigation buttons
        {
            let s = self_.clone();
            btn_back_borrow(&*self_.borrow()).connect_clicked({
                let s = s.clone();
                move |_| {
                    let path_opt = {
                        let me = s.borrow();
                        let mut state = me.session.borrow_mut();
                        if state.active_tab_mut().go_back() {
                            Some(state.active_tab().current_path.clone())
                        } else { None }
                    };
                    if let Some(path) = path_opt {
                        s.borrow().nav_callback.as_ref()(path);
                    }
                }
            });
        }
        {
            let s = self_.clone();
            btn_forward_borrow(&*self_.borrow()).connect_clicked({
                let s = s.clone();
                move |_| {
                    let path_opt = {
                        let me = s.borrow();
                        let mut state = me.session.borrow_mut();
                        if state.active_tab_mut().go_forward() {
                            Some(state.active_tab().current_path.clone())
                        } else { None }
                    };
                    if let Some(path) = path_opt {
                        s.borrow().nav_callback.as_ref()(path);
                    }
                }
            });
        }
        {
            let s = self_.clone();
            btn_up_borrow(&*self_.borrow()).connect_clicked({
                let s = s.clone();
                move |_| {
                    let path_opt = {
                        let me = s.borrow();
                        let mut state = me.session.borrow_mut();
                        if state.active_tab_mut().go_up() {
                            Some(state.active_tab().current_path.clone())
                        } else { None }
                    };
                    if let Some(path) = path_opt {
                        s.borrow().nav_callback.as_ref()(path);
                    }
                }
            });
        }
        {
            let s = self_.clone();
            btn_refresh.connect_clicked(move |_| {
                let path = s.borrow().session.borrow().active_tab().current_path.clone();
                s.borrow().nav_callback.as_ref()(path);
            });
        }

        // Address bar toggle on click
        {
            let s = self_.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.connect_pressed({
                let s = s.clone();
                move |_, _, _, _| {
                    let me = s.borrow();
                    if me.address_stack.visible_child_name().as_deref() == Some("breadcrumbs") {
                        let path = me.session.borrow().active_tab().current_path.clone();
                        me.entry_address.set_text(&path.to_string_lossy());
                        me.address_stack.set_visible_child_name("address");
                        me.entry_address.grab_focus();
                    }
                }
            });
            self_.borrow().address_wrap.add_controller(gesture);
        }

        // Entry activate
        {
            let s = self_.clone();
            self_.borrow().entry_address.connect_activate(move |entry| {
                let text = entry.text().to_string();
                let path = PathBuf::from(&text);
                if path.exists() {
                    {
                        let me = s.borrow();
                        let mut state = me.session.borrow_mut();
                        state.active_tab_mut().navigate_to(path.clone());
                        me.address_stack.set_visible_child_name("breadcrumbs");
                    }
                    s.borrow().nav_callback.as_ref()(path);
                }
            });
        }

        self_
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    pub fn update_address(&self, path: &std::path::Path) {
        // Clear existing breadcrumbs
        while let Some(child) = self.breadcrumb_box.first_child() {
            self.breadcrumb_box.remove(&child);
        }

        let home = glib::home_dir();
        let components: Vec<_> = path.components().collect();

        let mut current = PathBuf::new();
        for (i, comp) in components.iter().enumerate() {
            let comp_str = match comp {
                std::path::Component::RootDir => "/".to_string(),
                std::path::Component::Normal(s) => s.to_string_lossy().to_string(),
                _ => continue,
            };

            current.push(comp);

            // Friendly name for home
            let display = if current == home {
                "Home".to_string()
            } else {
                comp_str.clone()
            };

            let btn = Button::builder()
                .label(&display)
                .css_classes(vec!["breadcrumb-btn".to_string()])
                .build();

            let target = current.clone();
            let session = self.session.clone();
            let nav_cb  = self.nav_callback.clone();
            btn.connect_clicked(move |_| {
                { session.borrow_mut().active_tab_mut().navigate_to(target.clone()); }
                nav_cb(target.clone());
            });

            self.breadcrumb_box.append(&btn);

            if i + 1 < components.len() {
                let sep = Label::new(Some("›"));
                sep.set_css_classes(&["breadcrumb-sep"]);
                self.breadcrumb_box.append(&sep);
            }
        }

        // Switch back to breadcrumbs view
        self.address_stack.set_visible_child_name("breadcrumbs");
    }
}

// Helper fns to borrow inner buttons without Rc<RefCell> issue
fn btn_back_borrow(hb: &HeaderBar) -> &Button { &hb.btn_back }
fn btn_forward_borrow(hb: &HeaderBar) -> &Button { &hb.btn_forward }
fn btn_up_borrow(hb: &HeaderBar) -> &Button { &hb.btn_up }
