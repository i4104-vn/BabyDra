//! System monitor metrics data models.

use serde::{Deserialize, Serialize};

/// Raw CPU time values used to calculate delta load values.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CpuTime {
    pub total: u64,
    pub idle: u64,
}

/// Unified system resource snapshot broadcast by MonitorService.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MonitorSnapshot {
    pub cpu_percent: f64,
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub ram_percent: f64,
    pub gpu_percent: f64,
    pub uptime: String,
}

/// Per-application resource consumption data.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppResourceUsage {
    pub cpu_percent: f64,
    pub ram_mb: f64,
    pub ram_formatted: String,
    pub cpu_formatted: String,
    pub is_running: bool,
}

pub struct MonitorReceiver {
    rx: std::sync::mpsc::Receiver<MonitorSnapshot>,
}

impl MonitorReceiver {
    pub fn new(rx: std::sync::mpsc::Receiver<MonitorSnapshot>) -> Self {
        Self { rx }
    }

    pub fn attach<F: FnMut(MonitorSnapshot) -> glib::ControlFlow + 'static>(self, _context: Option<&()>, mut func: F) {
        glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
            let mut latest = None;
            while let Ok(snap) = self.rx.try_recv() {
                latest = Some(snap);
            }
            if let Some(snap) = latest {
                func(snap)
            } else {
                glib::ControlFlow::Continue
            }
        });
    }
}

