use notify::{Event, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    watched: Vec<PathBuf>,
}

impl FileWatcher {
    pub fn new<F>(path: PathBuf, mut callback: F) -> Result<Self, notify::Error>
    where
        F: FnMut(Event) + Send + 'static,
    {
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                if !matches!(event.kind, notify::EventKind::Access(_)) {
                    callback(event);
                }
            }
        })?;

        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(Self {
            watcher,
            watched: vec![path],
        })
    }

    pub fn watch(&mut self, path: &Path) -> Result<(), notify::Error> {
        // Stop watching previously navigated directories first; otherwise the
        // recommended watcher accumulates a watch for every visited directory
        // and keeps firing events for folders that are no longer on screen.
        let stale = std::mem::take(&mut self.watched);
        for old in &stale {
            if old != path {
                let _ = self.watcher.unwatch(old);
            }
        }

        self.watcher.watch(path, RecursiveMode::NonRecursive)?;
        self.watched.push(path.to_path_buf());

        Ok(())
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(path)
    }
}
