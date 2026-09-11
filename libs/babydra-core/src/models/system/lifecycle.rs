//! Application lifecycle handles and state models.

/// Holds active background service handles for the application lifecycle.
#[derive(Debug, Default)]
pub struct AppLifecycle {
    pub tray_watcher: TrayWatcherHandle,
    pub app_cache: AppCacheHandle,
    pub switcher: SwitcherHandle,
}

/// Handle representing the active System Tray DBus watcher service.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TrayWatcherHandle;

/// Handle representing the background desktop app cache refresher.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AppCacheHandle;

/// Handle representing the active window switcher tracker service.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SwitcherHandle;
