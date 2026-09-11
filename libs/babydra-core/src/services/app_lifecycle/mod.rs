use crate::services::apps;
use crate::services::logger;
use crate::services::tray;
use crate::services::window::tracker;

pub use crate::models::system::lifecycle::{
    AppCacheHandle, AppLifecycle, SwitcherHandle, TrayWatcherHandle,
};

pub fn init_app(app_name: &str) -> AppLifecycle {
    let log_path = logger::init_logger(app_name, &format!("{}.log", app_name));
    tracing::info!("App lifecycle initialized, log: {}", log_path.display());

    let tray_watcher = TrayWatcherHandle::default();
    tray::spawn_watcher();

    let app_cache = AppCacheHandle::default();
    std::thread::spawn(|| {
        apps::refresh_desktop_apps();
    });

    let switcher = SwitcherHandle::default();
    tracker::spawn_switcher();

    AppLifecycle {
        tray_watcher,
        app_cache,
        switcher,
    }
}