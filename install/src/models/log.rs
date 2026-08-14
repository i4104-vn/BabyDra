use chrono::Local;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Success,
    Warn,
    Error,
    Copy,
    Bundle,
    Config,
}

#[derive(Debug, Clone)]
pub struct LogMessage {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp: Local::now().format("%H:%M:%S").to_string(),
            level,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallState {
    Idle,
    Installing,
    Completed {
        success: bool,
        total_copied: usize,
        total_errors: usize,
    },
}
