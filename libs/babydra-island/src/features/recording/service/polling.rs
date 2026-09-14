//! Background poller monitoring screen recording state via DBus.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct IslandRecordingState {
    pub is_recording: bool,
    pub is_paused: bool,
    pub elapsed_secs: u64,
    pub mode_name: String,
    pub format: String,
    pub framerate: u32,
}

/// Spawns a background thread that polls `org.babydra.Recorder` via DBus.
pub fn spawn_recording_polling() -> Rc<RefCell<IslandRecordingState>> {
    let state_rc = Rc::new(RefCell::new(IslandRecordingState::default()));

    let (sender, mut receiver) =
        tokio::sync::mpsc::unbounded_channel::<IslandRecordingState>();

    std::thread::Builder::new()
        .name("island-recorder-poll".into())
        .spawn(move || {
            let mut conn_opt = zbus::blocking::Connection::session().ok();

            loop {
                let mut state = IslandRecordingState::default();

                if conn_opt.is_none() {
                    conn_opt = zbus::blocking::Connection::session().ok();
                }

                if let Some(ref conn) = conn_opt {
                    if let Ok(proxy) = zbus::blocking::Proxy::new(
                        conn,
                        "org.babydra.Recorder",
                        "/org/babydra/Recorder",
                        "org.babydra.Recorder",
                    ) {
                        let status_res: Result<String, _> = proxy.call("GetStatus", &());
                        if let Ok(status) = status_res {
                            if status == "recording" {
                                state.is_recording = true;
                                state.elapsed_secs =
                                    proxy.call("GetElapsed", &()).unwrap_or(0);
                                state.is_paused =
                                    proxy.call("IsPaused", &()).unwrap_or(false);
                                state.mode_name = proxy
                                    .call("GetMode", &())
                                    .unwrap_or_else(|_| "Fullscreen".to_string());
                                state.format = proxy
                                    .call("GetFormat", &())
                                    .unwrap_or_else(|_| "MP4".to_string());
                                state.framerate =
                                    proxy.call("GetFramerate", &()).unwrap_or(60);
                            }
                        }
                    }
                }

                // Fallback check: if wf-recorder is actively running in system
                if !state.is_recording {
                    if babydra_core::services::recording::is_recording() {
                        state.is_recording = true;
                        state.is_paused = babydra_core::services::recording::is_paused();
                        state.elapsed_secs =
                            babydra_core::services::recording::get_elapsed_secs();
                        if let Some(cfg) =
                            babydra_core::services::recording::get_active_config()
                        {
                            state.mode_name = match cfg.mode {
                                babydra_core::models::recording::RecordingMode::Fullscreen => {
                                    "Fullscreen".to_string()
                                }
                                babydra_core::models::recording::RecordingMode::SingleOutput(
                                    ref o,
                                ) => format!("Display ({o})"),
                                babydra_core::models::recording::RecordingMode::Area {
                                    ..
                                } => "Area".to_string(),
                                babydra_core::models::recording::RecordingMode::Window(
                                    _,
                                ) => "Window / Region".to_string(),
                            };
                            state.format = cfg.format.to_uppercase();
                            state.framerate = cfg.framerate;
                        }
                    } else if std::process::Command::new("pgrep")
                        .args(["-x", "wf-recorder"])
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false)
                    {
                        state.is_recording = true;
                        state.mode_name = "Screen".to_string();
                        state.format = "MP4".to_string();
                        state.framerate = 60;
                    }
                }

                if sender.send(state).is_err() {
                    break;
                }

                std::thread::sleep(Duration::from_millis(500));
            }
        })
        .ok();

    let cache = state_rc.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Some(st) = receiver.recv().await {
            *cache.borrow_mut() = st;
        }
    });

    state_rc
}
