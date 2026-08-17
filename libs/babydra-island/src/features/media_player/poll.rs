//! Background playerctl polling.
//!
//! A worker thread queries `playerctl metadata` every second and ships the raw
//! line over a channel; a main-thread receiver caches the latest value in an
//! `Rc<RefCell<Option<String>>>` that the feature reads each tick.

use std::cell::RefCell;
use std::rc::Rc;

use babydra_core::run_playerctl;

const METADATA_FORMAT: &str = "{{ status }}|//|{{ title }}|//|{{ artist }}|//|{{ playerName }}|//|{{ mpris:artUrl }}|//|{{ position }}|//|{{ mpris:length }}";

/// Spawns the polling thread + main-thread cache and returns the shared cache.
pub(crate) fn spawn_playerctl_polling() -> Rc<RefCell<Option<String>>> {
    let latest_metadata = Rc::new(RefCell::new(None));

    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<Option<String>>();
    std::thread::spawn(move || {
        loop {
            let metadata = run_playerctl(&["metadata", "--format", METADATA_FORMAT]);
            if sender.send(metadata).is_err() {
                break; // Receiver dropped (feature disposed).
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    let cache = latest_metadata.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Some(metadata) = receiver.recv().await {
            *cache.borrow_mut() = metadata;
        }
    });

    latest_metadata
}
