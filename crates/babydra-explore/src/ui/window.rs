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
    header_bar: Rc<RefCell<Option<HeaderBar>>>,
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

        // Load Windows 11 Dark Mode styling
        let provider = gtk4::CssProvider::new();
        provider.load_from_data("
            window, .main-window {
                background-color: #1c1c1c;
                color: #ffffff;
                font-family: 'Segoe UI', 'Inter', sans-serif;
            }
            .header-bar {
                background-color: #1c1c1c;
                border-bottom: 1px solid #2b2b2b;
                padding: 8px;
            }
            .sidebar {
                background-color: #202020;
                border-right: 1px solid #2b2b2b;
                padding: 6px;
            }
            .sidebar list {
                background: transparent;
            }
            .sidebar row {
                background: transparent;
                border-radius: 4px;
                margin: 2px 4px;
                padding: 8px 12px;
                transition: background 0.15s ease;
            }
            .sidebar row:hover {
                background-color: rgba(255, 255, 255, 0.06);
            }
            .sidebar row:selected {
                background-color: rgba(255, 255, 255, 0.1);
                color: #ffffff;
            }
            entry {
                background-color: rgba(255, 255, 255, 0.06);
                border: 1px solid rgba(255, 255, 255, 0.08);
                border-bottom: 2px solid rgba(255, 255, 255, 0.2);
                border-radius: 4px;
                color: #ffffff;
                padding: 6px 12px;
                transition: all 0.15s ease;
            }
            entry:focus {
                background-color: #202020;
                border: 1px solid #60cdff;
                border-bottom: 2px solid #60cdff;
                box-shadow: none;
            }
            .content-view {
                background-color: #1e1e1e;
            }
            .file-item {
                padding: 8px;
                border-radius: 6px;
                transition: all 0.15s ease;
            }
            button.flat {
                background: transparent;
                border: none;
                border-radius: 6px;
                padding: 4px;
                color: #ffffff;
            }
            button.flat:hover {
                background-color: rgba(255, 255, 255, 0.06);
            }
            button.flat:active {
                background-color: rgba(255, 255, 255, 0.1);
            }
            .active-pane {
                outline: 2px solid #60cdff;
                outline-offset: -2px;
                border-radius: 4px;
            }
            .info-panel {
                background-color: #202020;
                border-left: 1px solid #2b2b2b;
                padding: 12px;
            }
            .info-panel frame {
                border: 1px solid #2b2b2b;
                border-radius: 6px;
                background-color: rgba(255, 255, 255, 0.02);
            }
            .context-menu-box {
                background-color: #2c2c2c;
                border: 1px solid #3c3c3c;
                border-radius: 8px;
                padding: 4px;
                box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
            }
            .context-menu-item {
                border-radius: 4px;
                padding: 6px 12px;
                transition: background 0.12s ease;
            }
            .context-menu-item:hover {
                background-color: rgba(255, 255, 255, 0.08);
            }
        ");
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

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

        let header = HeaderBar::new(
            self_.session.clone(),
            nav_callback.clone(),
            view_mode_callback,
        );
        vbox.append(header.widget());
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
                    // Update Header
                    if let Some(ref h) = *header_bar.borrow() {
                        h.update(&path);
                    }

                    // Calculate total size
                    let total_size: u64 = entries.iter().map(|e| e.size).sum();

                    // Update Content
                    content_view.update(&entries, self_rc, path);

                    // Update Status Bar
                    if let Some(ref s) = *status_bar.borrow() {
                        s.update(entries.len(), total_size);
                    }
                }
                Err(err) => {
                    eprintln!("Failed to load directory: {}", err);
                }
            }
        });
    }
}
