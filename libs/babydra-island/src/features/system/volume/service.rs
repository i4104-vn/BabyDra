//! Background listener for system audio volume and mute state changes.

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeState {
    pub volume: f64,
    pub muted: bool,
}

/// Spawns an event listener thread that monitors `pactl subscribe` and dispatches
/// state changes to the GTK main thread via a channel.
pub fn spawn_volume_listener<F>(on_change: F)
where
    F: Fn(VolumeState) + 'static,
{
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<VolumeState>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(state) = receiver.recv().await {
            on_change(state);
        }
    });

    let running = Arc::new(AtomicBool::new(true));

    std::thread::Builder::new()
        .name("babydra-island-volume-listener".into())
        .spawn(move || {
            let (volume, muted) = babydra_core::services::system::volume::get_volume_state();
            let mut last_state = VolumeState { volume, muted };

            while running.load(Ordering::Relaxed) {
                let Ok(mut child) = Command::new("pactl")
                    .arg("subscribe")
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn()
                else {
                    // pactl is not available, fall back to sleep loop
                    std::thread::sleep(Duration::from_millis(200));
                    let (volume, muted) =
                        babydra_core::services::system::volume::get_volume_state();
                    let current = VolumeState { volume, muted };
                    if current != last_state {
                        last_state = current;
                        if sender.send(current).is_err() {
                            break;
                        }
                    }
                    continue;
                };

                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line_result in reader.lines() {
                        let Ok(line) = line_result else {
                            break;
                        };
                        // pactl emits lines like: Event 'change' on sink #...
                        if line.contains("sink") {
                            let (volume, muted) =
                                babydra_core::services::system::volume::get_volume_state();
                            let current = VolumeState { volume, muted };
                            if current != last_state {
                                last_state = current;
                                if sender.send(current).is_err() {
                                    let _ = child.kill();
                                    let _ = child.wait();
                                    return;
                                }
                            }
                        }
                    }
                }

                let _ = child.kill();
                let _ = child.wait();
                std::thread::sleep(Duration::from_millis(500));
            }
        })
        .ok();
}
