//! System metrics, power, hardware and daemon models.

pub mod account;
pub mod battery;
pub mod clipboard;
pub mod daemon;
pub mod lifecycle;
pub mod monitor;
pub mod power;
pub mod storage;
pub mod volume;

pub use account::UserAccountInfo;
pub use battery::BatteryInfo;
pub use clipboard::ClipboardEntry;
pub use daemon::DaemonMessage;
pub use lifecycle::{AppCacheHandle, AppLifecycle, SwitcherHandle, TrayWatcherHandle};
pub use monitor::{AppResourceUsage, CpuTime, MonitorReceiver, MonitorSnapshot};
pub use power::PerformanceProfile;
pub use storage::DiskInfo;
pub use volume::{AudioBackendType, AudioDevice};
