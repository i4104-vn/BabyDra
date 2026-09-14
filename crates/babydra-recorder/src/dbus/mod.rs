//! DBus interface exports for the recorder daemon.

pub mod service;

pub use service::{RecorderCommand, RecorderDbusService};
