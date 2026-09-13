//! Background listener for system screen brightness changes.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Spawns a background thread checking brightness changes and dispatching
/// them to the GTK main thread via a channel.
pub fn spawn_brightness_listener<F>(on_change: F)
where
    F: Fn(f64) + 'static,
{
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<f64>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(val) = receiver.recv().await {
            on_change(val);
        }
    });

    let running = Arc::new(AtomicBool::new(true));

    std::thread::Builder::new()
        .name("babydra-island-brightness-listener".into())
        .spawn(move || {
            let mut last_brightness = babydra_core::services::system::backlight::get_brightness();

            while running.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(80));

                let current = babydra_core::services::system::backlight::get_brightness();
                if (current - last_brightness).abs() >= 0.5 {
                    last_brightness = current;
                    if sender.send(current).is_err() {
                        break;
                    }
                }
            }
        })
        .ok();
}
