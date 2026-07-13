use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box, Orientation, Paned};
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use babydra_common::SessionState;
use crate::ui::header_bar::HeaderBar;
use crate::ui::sidebar::Sidebar;
use crate::ui::content_view::ContentView;
use crate::ui::status_bar::StatusBar;
use crate::ui::info_panel::InfoPanel;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivePane {
    Left,
    Right,
}

pub struct MainWindow {
    window: ApplicationWindow,
    session: Rc<RefCell<SessionState>>,
    header_bar: Rc<RefCell<Option<Rc<RefCell<HeaderBar>>>>>,
    tab_bar: Rc<RefCell<Option<crate::ui::tab_bar::TabBar>>>,
    sidebar: Rc<RefCell<Option<Sidebar>>>,
    left_content_view: Rc<ContentView>,
    right_content_view: Rc<RefCell<Option<Rc<ContentView>>>>,
    is_split: Cell<bool>,
    active_pane: Cell<ActivePane>,
    split_paned: Paned,
    status_bar: Rc<RefCell<Option<StatusBar>>>,
    self_weak: RefCell<Option<std::rc::Weak<MainWindow>>>,
    watcher: RefCell<Option<babydra_common::FileWatcher>>,
    watch_tx: tokio::sync::mpsc::UnboundedSender<()>,
    info_panel: Rc<RefCell<Option<Rc<InfoPanel>>>>,
}

impl MainWindow {
    pub fn new(app: &gtk4::Application, session: Rc<RefCell<SessionState>>) -> Rc<Self> {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("BabyDra Explore")
            .default_width(1000)
            .default_height(700)
            .build();
        window.add_css_class("explore-window");

        // Apply Windows 11 dark theme from babydra-common
        babydra_common::apply_explore_theme();

        let vbox = Box::new(Orientation::Vertical, 0);
        window.set_child(Some(&vbox));

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

        // Create Split Paned container
        let split_paned = Paned::new(Orientation::Horizontal);
        split_paned.set_hexpand(true);
        split_paned.set_vexpand(true);

        // HeaderBar navigation setup
        let (left_tx, mut left_rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();

        let left_nav_callback = move |path: PathBuf| {
            let _ = left_tx.send(path);
        };

        // Create content views
        let left_content_view = Rc::new(ContentView::new(left_nav_callback));
        split_paned.set_start_child(Some(left_content_view.widget()));

        let self_ = Rc::new(Self {
            window: window.clone(),
            session,
            header_bar: Rc::new(RefCell::new(None)),
            tab_bar: Rc::new(RefCell::new(None)),
            sidebar: Rc::new(RefCell::new(None)),
            left_content_view,
            right_content_view: Rc::new(RefCell::new(None)),
            is_split: Cell::new(false),
            active_pane: Cell::new(ActivePane::Left),
            split_paned,
            status_bar: Rc::new(RefCell::new(None)),
            self_weak: RefCell::new(None),
            watcher: RefCell::new(None),
            watch_tx: tx,
            info_panel: Rc::new(RefCell::new(None)),
        });

        // Store weak reference
        *self_.self_weak.borrow_mut() = Some(Rc::downgrade(&self_));

        // Connect receiver to reload active directory
        let self_weak = Rc::downgrade(&self_);
        glib::MainContext::default().spawn_local(async move {
            while let Some(_) = rx.recv().await {
                if let Some(win) = self_weak.upgrade() {
                    let path = win.session.borrow().active_tab().current_path.clone();
                    win.navigate_to_no_watch(path);
                }
            }
        });

        // Wire left/right pane navigation channels
        let self_weak = Rc::downgrade(&self_);
        glib::MainContext::default().spawn_local(async move {
            while let Some(path) = left_rx.recv().await {
                if let Some(win) = self_weak.upgrade() {
                    win.navigate_pane(ActivePane::Left, path);
                }
            }
        });

        // Setup global window navigation callback
        let self_clone = self_.clone();
        let nav_callback = move |path: PathBuf| {
            self_clone.navigate_to(path);
        };

        // HeaderBar
        let content_view_ref = self_.left_content_view.clone();
        let right_view_ref = self_.right_content_view.clone();
        let view_mode_callback = move |mode: String| {
            content_view_ref.set_view_mode(&mode);
            if let Some(ref r) = *right_view_ref.borrow() {
                r.set_view_mode(&mode);
            }
        };

        let left_view_ref = self_.left_content_view.clone();
        let right_view_ref_for_search = self_.right_content_view.clone();
        let self_weak_for_search = Rc::downgrade(&self_);
        let search_callback = move |query: String| {
            if let Some(win) = self_weak_for_search.upgrade() {
                if win.active_pane.get() == ActivePane::Left {
                    left_view_ref.filter(&query);
                } else if let Some(ref r) = *right_view_ref_for_search.borrow() {
                    r.filter(&query);
                }
            }
        };

        // TabBar setup
        let self_weak_for_tabs = Rc::downgrade(&self_);
        let on_tab_activated = {
            let self_weak = self_weak_for_tabs.clone();
            move |idx: usize| {
                if let Some(win) = self_weak.upgrade() {
                    {
                        let mut sess = win.session.borrow_mut();
                        sess.active_tab_index = idx;
                    }
                    let active_path = win.session.borrow().active_tab().current_path.clone();
                    win.navigate_to(active_path);
                    win.update_tab_bar();
                }
            }
        };

        let on_tab_closed = {
            let self_weak = self_weak_for_tabs.clone();
            move |idx: usize| {
                if let Some(win) = self_weak.upgrade() {
                    let should_navigate = {
                        let mut sess = win.session.borrow_mut();
                        let is_active = sess.active_tab_index == idx;
                        let closed = sess.close_tab(idx);
                        closed && is_active
                    };
                    if should_navigate {
                        let active_path = win.session.borrow().active_tab().current_path.clone();
                        win.navigate_to(active_path);
                    }
                    win.update_tab_bar();
                }
            }
        };

        let on_tab_created = {
            let self_weak = self_weak_for_tabs.clone();
            move || {
                if let Some(win) = self_weak.upgrade() {
                    let home = glib::home_dir();
                    {
                        let mut sess = win.session.borrow_mut();
                        sess.add_tab(home.clone());
                    }
                    win.navigate_to(home);
                    win.update_tab_bar();
                }
            }
        };

        let tab_bar = crate::ui::tab_bar::TabBar::new(
            self_.session.clone(),
            on_tab_activated,
            on_tab_closed,
            on_tab_created,
        );
        vbox.append(tab_bar.widget());
        self_.tab_bar.replace(Some(tab_bar));

        let header = HeaderBar::new(
            self_.session.clone(),
            nav_callback.clone(),
            view_mode_callback,
            search_callback,
        );
        vbox.append(header.borrow().widget());
        self_.header_bar.replace(Some(header));

        // Paned (Sidebar + Main Split Content View Area)
        let main_paned = Paned::new(Orientation::Horizontal);
        main_paned.set_hexpand(true);
        main_paned.set_vexpand(true);
        main_paned.set_position(220); // Allocate sidebar space
        vbox.append(&main_paned);

        // Sidebar
        let sidebar = Sidebar::new(self_.session.clone(), nav_callback.clone());
        main_paned.set_start_child(Some(sidebar.widget()));
        self_.sidebar.replace(Some(sidebar));

        // Content Area VBox (contains SplitPaned + InfoPanel side-by-side)
        let content_vbox = Box::new(Orientation::Vertical, 0);
        content_vbox.set_hexpand(true);
        content_vbox.set_vexpand(true);
        main_paned.set_end_child(Some(&content_vbox));

        // Horizontal Paned to show InfoPanel resizable next to split view
        let layout_paned = Paned::new(Orientation::Horizontal);
        layout_paned.set_hexpand(true);
        layout_paned.set_vexpand(true);
        layout_paned.set_position(530); // Allocate space for InfoPanel
        content_vbox.append(&layout_paned);

        layout_paned.set_start_child(Some(&self_.split_paned));

        // InfoPanel
        let info_panel = Rc::new(InfoPanel::new());
        layout_paned.set_end_child(Some(info_panel.widget()));
        self_.info_panel.replace(Some(info_panel.clone()));

        // Connect selection callback on Left pane
        let info_panel_clone = info_panel.clone();
        let self_weak = Rc::downgrade(&self_);
        self_.left_content_view.connect_selection_changed(move |selected| {
            if let Some(win) = self_weak.upgrade() {
                win.set_active_pane(ActivePane::Left);
            }
            info_panel_clone.update(&selected);
        });

        // Add visual styling active pane classes
        self_.left_content_view.widget().add_css_class("active-pane");

        // Key Press Controller for F3 split toggling
        let key_controller = gtk4::EventControllerKey::new();
        let self_clone = self_.clone();
        key_controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
            if keyval == gtk4::gdk::Key::F3 {
                self_clone.toggle_split_view();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        window.add_controller(key_controller);

        // StatusBar
        let status_bar = StatusBar::new();
        vbox.append(status_bar.widget());
        self_.status_bar.replace(Some(status_bar));

        self_
    }

    pub fn show(&self) {
        self.window.present();
        // Trigger initial navigation
        let path = self.session.borrow().active_tab().current_path.clone();
        self.navigate_pane(ActivePane::Left, path);
    }

    pub fn set_active_pane(&self, pane: ActivePane) {
        self.active_pane.set(pane);
        if pane == ActivePane::Left {
            self.left_content_view.widget().add_css_class("active-pane");
            if let Some(ref right) = *self.right_content_view.borrow() {
                right.widget().remove_css_class("active-pane");
            }
        } else {
            self.left_content_view.widget().remove_css_class("active-pane");
            if let Some(ref right) = *self.right_content_view.borrow() {
                right.widget().add_css_class("active-pane");
            }
        }
    }

    pub fn toggle_split_view(&self) {
        if self.is_split.get() {
            // Remove right view
            self.split_paned.set_end_child(None::<&gtk4::Widget>);
            self.right_content_view.replace(None);
            self.is_split.set(false);
            self.set_active_pane(ActivePane::Left);
        } else {
            // Add right view starting at left current path
            let current_p = self.session.borrow().active_tab().current_path.clone();
            
            // Build Right pane navigation channel callback
            let self_weak = Rc::downgrade(&self.self_weak.borrow().as_ref().unwrap().upgrade().unwrap());
            let right_nav_cb = move |path: PathBuf| {
                if let Some(win) = self_weak.upgrade() {
                    win.navigate_pane(ActivePane::Right, path);
                }
            };
            let right_view = Rc::new(ContentView::new(right_nav_cb));
            
            self.split_paned.set_end_child(Some(right_view.widget()));
            self.split_paned.set_position(390); // Split view panes sized equally
            self.right_content_view.replace(Some(right_view.clone()));
            self.is_split.set(true);

            // Connect selection callback on Right pane
            let info_panel_opt = self.info_panel.borrow();
            let info_panel = info_panel_opt.as_ref().unwrap().clone();
            let info_panel_clone = info_panel.clone();
            let self_weak = Rc::downgrade(&self.self_weak.borrow().as_ref().unwrap().upgrade().unwrap());
            right_view.connect_selection_changed(move |selected| {
                if let Some(win) = self_weak.upgrade() {
                    win.set_active_pane(ActivePane::Right);
                }
                info_panel_clone.update(&selected);
            });

            self.set_active_pane(ActivePane::Right);
            self.navigate_pane(ActivePane::Right, current_p);
        }
    }

    pub fn navigate_to(&self, path: PathBuf) {
        self.navigate_pane(self.active_pane.get(), path);
    }

    pub fn navigate_to_no_watch(&self, path: PathBuf) {
        self.navigate_pane_no_watch(self.active_pane.get(), path);
    }

    pub fn navigate_pane(&self, pane: ActivePane, path: PathBuf) {
        // Setup or update watcher
        let mut watcher_borrow = self.watcher.borrow_mut();
        if let Some(ref mut w) = *watcher_borrow {
            let _ = w.watch(&path);
        } else {
            let tx_clone = self.watch_tx.clone();
            if let Ok(w) = babydra_common::FileWatcher::new(path.clone(), move |_event| {
                let _ = tx_clone.send(());
            }) {
                *watcher_borrow = Some(w);
            }
        }

        self.navigate_pane_no_watch(pane, path);
    }

    pub fn navigate_pane_no_watch(&self, pane: ActivePane, path: PathBuf) {
        // Highlight active pane
        self.set_active_pane(pane);

        let show_hidden = self.session.borrow().active_tab().current_path == path 
            && self.session.borrow().active_tab().history_index > 0; // standard default false
        
        let header_bar = self.header_bar.clone();
        let content_view = if pane == ActivePane::Left {
            self.left_content_view.clone()
        } else {
            match &*self.right_content_view.borrow() {
                Some(r) => r.clone(),
                None => return,
            }
        };
        let status_bar = self.status_bar.clone();

        // Update session path
        self.session.borrow_mut().active_tab_mut().current_path = path.clone();

        // Upgrade weak self reference
        let self_weak_opt = self.self_weak.borrow();
        let self_rc = match &*self_weak_opt {
            Some(weak) => match weak.upgrade() {
                Some(rc) => rc,
                None => return,
            },
            None => return,
        };

        glib::spawn_future_local(async move {
            match babydra_common::load_directory(path.clone(), show_hidden).await {
                Ok(entries) => {
                    // Update Header breadcrumbs
                    if let Some(ref h_rc) = *header_bar.borrow() {
                        h_rc.borrow().update_address(&path);
                    }

                    // Calculate total size
                    let total_size: u64 = entries.iter().map(|e| e.size).sum();

                    // Update Content
                    content_view.update(&entries, self_rc.clone(), path);

                    // Update Status Bar
                    if let Some(ref s) = *status_bar.borrow() {
                        s.update(entries.len(), total_size);
                    }

                    // Update Tab Bar titles
                    self_rc.update_tab_bar();
                }
                Err(err) => {
                    eprintln!("Failed to load directory: {}", err);
                }
            }
        });
    }

    pub fn update_tab_bar(&self) {
        if let Some(ref tb) = *self.tab_bar.borrow() {
            tb.rebuild();
        }
    }
}
