//! Main entry point for the BabyDra global keymap daemon.
//!
//! `babydra-keymap` runs in the background and handles global keyboard
//! shortcuts on behalf of the BabyDra shell, replacing labwc's `<keybind>`
//! configuration.  It reads raw keyboard events from `/dev/input` via evdev,
//! matches them against shortcuts declared in `~/.config/babydra/keymap.toml`
//! (shared with babydra-settings) and spawns the configured command.
//!
//! The config file is hot-reloaded whenever it changes on disk, so edits made
//! in babydra-settings take effect immediately.

mod daemon;
mod keyboard;
mod mapping;

fn main() {
    babydra_core::services::logger::init_logger("babydra-keymap", "babydra-keymap.log");

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
        .block_on(daemon::run());
}
