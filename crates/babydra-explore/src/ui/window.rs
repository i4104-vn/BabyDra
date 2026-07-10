use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box, Orientation, Paned};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use babydra_common::SessionState;
use crate::ui::header_bar::HeaderBar;
use crate::ui::sidebar::Sidebar;
use crate::ui::content_view::ContentView;
use crate::ui::status_bar::StatusBar;

pub struct MainWindow {
    window: ApplicationWindow,
    session: Rc<RefCell<SessionState>>,
    header_bar: Rc<RefCell<Option<HeaderBar>>>,
    sidebar: Rc<RefCell<Option<Sidebar>>>,
    content_view: Rc<RefCell<Option<ContentView>>>,
    status_bar: Rc<RefCell<Option<StatusBar>>>,
    self_weak: RefCell<Option<std::rc::Weak<MainWindow>>>,
    watcher: RefCell<Option<babydra_common::FileWatcher>>,
    watch_tx: tokio::sync::mpsc::UnboundedSender<()>,
}

impl MainWindow {
    pub fn new(app: &gtk4::Application, session: Rc<RefCell<SessionState>>) -> Rc<Self> {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("BabyDra Explore")
            .default_width(1000)
            .default_height(700)
            .build();

        let vbox = Box::new(Orientation::Vertical, 0);
        window.set_child(Some(&vbox));

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

        let self_ = Rc::new(Self {
            window,
            session,
            header_bar: Rc::new(RefCell::new(None)),
            sidebar: Rc::new(RefCell::new(None)),
            content_view: Rc::new(RefCell::new(None)),
            status_bar: Rc::new(RefCell::new(None)),
            self_weak: RefCell::new(None),
            watcher: RefCell::new(None),
            watch_tx: tx,
        });

        // Store weak reference
        *self_.self_weak.borrow_mut() = Some(Rc::downgrade(&self_));

        // Connect receiver to reload directory
        let self_weak = Rc::downgrade(&self_);
        glib::MainContext::default().spawn_local(async move {
            while let Some(_) = rx.recv().await {
                if let Some(win) = self_weak.upgrade() {
                    let path = win.session.borrow().active_tab().current_path.clone();
                    win.navigate_to_no_watch(path);
                }
            }
        });

        // Setup navigation callback
        let self_clone = self_.clone();
        let nav_callback = move |path: PathBuf| {
            self_clone.navigate_to(path);
        };

        // HeaderBar
        let content_view_ref = self_.content_view.clone();
        let view_mode_callback = move |mode: String| {
            if let Some(ref c) = *content_view_ref.borrow() {
                c.set_view_mode(&mode);
            }
        };

        let header = HeaderBar::new(
            self_.session.clone(),
            nav_callback.clone(),
            view_mode_callback,
        );
        vbox.append(header.widget());
        self_.header_bar.replace(Some(header));

        // Paned (Sidebar + ContentView)
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_hexpand(true);
        paned.set_vexpand(true);
        vbox.append(&paned);

        // Sidebar
        let sidebar = Sidebar::new(self_.session.clone(), nav_callback.clone());
        paned.set_start_child(Some(sidebar.widget()));
        self_.sidebar.replace(Some(sidebar));

        // Content Area VBox
        let content_vbox = Box::new(Orientation::Vertical, 0);
        paned.set_end_child(Some(&content_vbox));

        // ContentView
        let content_view = ContentView::new(nav_callback);
        content_vbox.append(content_view.widget());
        self_.content_view.replace(Some(content_view));

        // StatusBar
        let status_bar = StatusBar::new();
        vbox.append(status_bar.widget());
        self_.status_bar.replace(Some(status_bar));

        self_
    }

    pub fn show(&self) {
        self.window.present();
        // Trigger initial navigation to current session path
        let path = self.session.borrow().active_tab().current_path.clone();
        self.navigate_to(path);
    }

    pub fn navigate_to(&self, path: PathBuf) {
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

        self.navigate_to_no_watch(path);
    }

    pub fn navigate_to_no_watch(&self, path: PathBuf) {
        let show_hidden = self.session.borrow().active_tab().current_path == path 
            && self.session.borrow().active_tab().history_index > 0; // standard default false
        
        let header_bar = self.header_bar.clone();
        let content_view = self.content_view.clone();
        let status_bar = self.status_bar.clone();

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
                    if let Some(ref c) = *content_view.borrow() {
                        c.update(&entries, self_rc, path);
                    }

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
