//! evdev keyboard listener for `babydra-keymap`.
//!
//! Enumerates `/dev/input`, spawns one listener task per keyboard device and
//! matches key-down events against the shared shortcut list.  Devices are
//! re-scanned periodically so hot-plugged keyboards are picked up.
//!
//! Requires the user to be in the `input` group (or equivalent udev rule) to
//! read from `/dev/input/event*`.

use crate::daemon::SharedShortcuts;
use crate::mapping::{Mods, ResolvedShortcut};
use evdev::{enumerate, Device, EventType, InputEvent, Key};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const RESCAN_INTERVAL: Duration = Duration::from_secs(5);

/// Global modifier state tracked across all keyboards.
#[derive(Default)]
struct ModifierState {
    shift: bool,
    ctrl: bool,
    alt: bool,
    win: bool,
}

impl ModifierState {
    fn snapshot(&self) -> Mods {
        Mods {
            shift: self.shift,
            ctrl: self.ctrl,
            alt: self.alt,
            win: self.win,
        }
    }

    /// Updates state for a modifier key.  Returns `true` when handled.
    fn update(&mut self, key: Key, pressed: bool) -> bool {
        match key {
            Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => self.shift = pressed,
            Key::KEY_LEFTCTRL | Key::KEY_RIGHTCTRL => self.ctrl = pressed,
            Key::KEY_LEFTALT | Key::KEY_RIGHTALT => self.alt = pressed,
            Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => self.win = pressed,
            _ => return false,
        }
        true
    }
}

type KnownDevices = Arc<Mutex<HashSet<PathBuf>>>;

/// Main loop: keeps scanning for keyboard devices and listens on each.
pub async fn run(shortcuts: SharedShortcuts) {
    let state = Arc::new(Mutex::new(ModifierState::default()));
    let known: KnownDevices = Arc::new(Mutex::new(HashSet::new()));

    loop {
        scan_once(&state, &shortcuts, &known);
        tokio::time::sleep(RESCAN_INTERVAL).await;
    }
}

/// Scans `/dev/input` once and spawns listeners for new keyboard devices.
fn scan_once(state: &Arc<Mutex<ModifierState>>, shortcuts: &SharedShortcuts, known: &KnownDevices) {
    let devices = enumerate();
    for device_result in devices {
        let (path, device) = device_result;
        if !known.lock().unwrap().insert(path.clone()) {
            continue;
        }
        if !looks_like_keyboard(&device) {
            // Not a real keyboard (power button, joystick, ...) — forget it
            // so a future device may reuse the same node name.
            known.lock().unwrap().remove(&path);
            continue;
        }

        tracing::info!(
            "listening on {} ({})",
            path.display(),
            device.name().unwrap_or("unnamed")
        );
        tokio::spawn(listen_device(
            path.clone(),
            device,
            state.clone(),
            shortcuts.clone(),
            known.clone(),
        ));
    }
}

/// True when the device exposes a typical typing keyset (excludes power
/// buttons, joysticks and other event devices).
fn looks_like_keyboard(device: &Device) -> bool {
    device
        .supported_keys()
        .map(|keys| keys.contains(Key::KEY_A) && keys.contains(Key::KEY_SPACE))
        .unwrap_or(false)
}

/// Listens for events from a single keyboard until the device disappears.
async fn listen_device(
    path: PathBuf,
    device: Device,
    state: Arc<Mutex<ModifierState>>,
    shortcuts: SharedShortcuts,
    known: KnownDevices,
) {
    let mut stream = match device.into_event_stream() {
        Ok(stream) => stream,
        Err(e) => {
            tracing::warn!("cannot open event stream for {}: {}", path.display(), e);
            known.lock().unwrap().remove(&path);
            return;
        }
    };

    loop {
        match stream.next_event().await {
            Ok(event) => handle_event(event, &state, &shortcuts),
            Err(e) => {
                tracing::info!("keyboard {} disconnected: {}", path.display(), e);
                break;
            }
        }
    }

    known.lock().unwrap().remove(&path);
}

/// Processes one input event: updates modifier tracking and triggers
/// shortcuts on matching non-modifier key presses.
fn handle_event(event: InputEvent, state: &Arc<Mutex<ModifierState>>, shortcuts: &SharedShortcuts) {
    if event.event_type() != EventType::KEY {
        return;
    }

    let key = Key::new(event.code());
    let pressed = event.value() == 1;
    let repeated = event.value() == 2; // auto-repeat

    if state.lock().unwrap().update(key, pressed || repeated) {
        return;
    }

    if !pressed {
        return;
    }

    try_trigger(key, state, shortcuts);
}

/// Fires the command of the first shortcut whose modifiers exactly match the
/// currently held ones and whose key was just pressed.
fn try_trigger(key: Key, state: &Arc<Mutex<ModifierState>>, shortcuts: &SharedShortcuts) {
    // Global capture (e.g. the Settings key-capture dialog) suppresses all
    // shortcuts so recorded keys never fire their real commands.
    if babydra_core::services::system::keymap::is_paused() {
        return;
    }

    let current = state.lock().unwrap().snapshot();

    let matched = shortcuts
        .read()
        .unwrap()
        .iter()
        .filter_map(ResolvedShortcut::resolve)
        .find(|s| s.key.code() == key.code() && s.mods == current)
        .map(|s| s.command);

    if let Some(command) = matched {
        tracing::debug!("shortcut triggered: {:?}", command);
        tokio::spawn(run_command(command));
    }
}

/// Spawns a shortcut command via `sh -c` so shell syntax keeps working.
async fn run_command(command: String) {
    match tokio::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .status()
        .await
    {
        Ok(status) if status.success() => {}
        Ok(status) => tracing::warn!("command {:?} exited with {}", command, status),
        Err(e) => tracing::error!("failed to run {:?}: {}", command, e),
    }
}
