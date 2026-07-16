use notify::{Watcher, RecursiveMode, Event};
use std::path::{Path, PathBuf};

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
}

impl FileWatcher {
    pub fn new<F>(path: PathBuf, mut callback: F) -> Result<Self, notify::Error>
    where
        F: FnMut(Event) + Send + 'static,
    {
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                callback(event);
            }
        })?;

        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(Self { watcher })
    }

    pub fn watch(&mut self, path: &Path) -> Result<(), notify::Error> {
        // Stop watching previous paths if any (recommended watcher handles multiple path targets)
        self.watcher.watch(path, RecursiveMode::NonRecursive)
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(path)
    }
}
