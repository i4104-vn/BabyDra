//! Wallpaper FileWatcher and periodic fallback polling.

use std::path::PathBuf;
use std::rc::Rc;

/// Starts background file watching on ~/.babydra and ~/.babydra/wallpaper,
/// and sets up a 2-second fallback poll timer to guarantee sync in all edge cases.
pub fn start_wallpaper_watcher(trigger_transition: Rc<dyn Fn()>) {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let babydra_dir = home.join(".babydra");
    let wallpaper_dir = babydra_dir.join("wallpaper");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

    let trigger_watch = trigger_transition.clone();
    glib::spawn_future_local(async move {
        while rx.recv().await.is_some() {
            trigger_watch();
        }
    });

    let tx1 = tx.clone();
    if let Ok(_w1) = babydra_core::FileWatcher::new(babydra_dir, move |_| {
        let _ = tx1.send(());
    }) {
        std::mem::forget(_w1);
    }

    let tx2 = tx.clone();
    if let Ok(_w2) = babydra_core::FileWatcher::new(wallpaper_dir, move |_| {
        let _ = tx2.send(());
    }) {
        std::mem::forget(_w2);
    }

    // Fallback poll (every 2s) to guarantee synchronization across any edge cases
    let trigger_poll = trigger_transition.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(2000), move || {
        trigger_poll();
        glib::ControlFlow::Continue
    });
}
