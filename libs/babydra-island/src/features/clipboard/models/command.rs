//! Command models for the island clipboard D-Bus service.

/// IPC commands sent to the dynamic island over user session D-Bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IslandDbusCommand {
    ShowClipboard,
    ToggleClipboard,
    ShowPower,
    TogglePower,
    ShowRecording,
}
