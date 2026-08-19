//! Desktop FileWatcher daemon and layout flush timer.

use crate::state::DesktopState;
use std::rc::Rc;

pub fn start_file_watcher(refresh_fn: Rc<dyn Fn()>) {
    let desktop_path = DesktopState::desktop_dir();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    let ref_cb_watch = refresh_fn.clone();

    glib::spawn_future_local(async move {
        while rx.recv().await.is_some() {
            ref_cb_watch();
        }
    });

    if let Ok(_watcher) = babydra_core::FileWatcher::new(desktop_path, move |_event| {
        let _ = tx.send(());
    }) {
        std::mem::forget(_watcher);
    }

    glib::timeout_add_local(std::time::Duration::from_millis(500), || {
        babydra_core::config::desktop_layout::flush_if_dirty();
        glib::ControlFlow::Continue
    });
}
