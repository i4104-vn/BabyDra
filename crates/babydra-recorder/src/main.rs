//! babydra-recorder — Wayland screen recording daemon with system tray, DBus interface, and GTK4 controls.

mod cli;
mod dbus;
mod tray;
mod widgets;

use cli::handle_cli_flag;
use dbus::{RecorderCommand, RecorderDbusService};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use tray::{register_with_watcher, StatusNotifierItemService};
use widgets::show_or_create_control_window;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_daemon = args.iter().any(|a| a == "--daemon" || a == "--background");

    if args.len() > 1 && !is_daemon {
        let flag = &args[1];
        if handle_cli_flag(flag) {
            return;
        }
    }

    // If launched as an interactive app (not daemon): check if daemon is already running.
    // If so, send ShowUi to the existing daemon to present its window, then exit.
    if !is_daemon {
        if cli::try_activate_running_instance() {
            return;
        }
    } else if cli::is_daemon_already_running() {
        eprintln!("babydra-recorder daemon is already running.");
        return;
    }

    let _lifecycle = babydra_core::services::app_lifecycle::init_app("babydra-recorder");

    let app = gtk4::Application::new(
        Some("org.babydra.RecorderApp"),
        gtk4::gio::ApplicationFlags::NON_UNIQUE,
    );

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<RecorderCommand>();
    let rx_holder = Rc::new(RefCell::new(Some(rx)));
    let window_holder: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));

    // Spawn DBus and StatusNotifierItem daemon thread
    let tx_clone = tx.clone();
    std::thread::Builder::new()
        .name("babydra-recorder-dbus".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();
            if let Ok(rt) = rt {
                rt.block_on(async move {
                    let dbus_svc = RecorderDbusService {
                        tx: tx_clone.clone(),
                    };
                    let tray_svc = StatusNotifierItemService { tx: tx_clone };

                    let conn_builder = zbus::connection::Builder::session();
                    if let Ok(builder) = conn_builder {
                        if let Ok(builder) = builder.name("org.babydra.Recorder") {
                            if let Ok(builder) = builder
                                .serve_at("/org/babydra/Recorder", dbus_svc)
                                .and_then(|b| b.serve_at("/StatusNotifierItem", tray_svc))
                            {
                                if let Ok(conn) = builder.build().await {
                                    // Register with host status notifier watcher (babydra-panel)
                                    let _ = register_with_watcher(&conn).await;

                                    let _ = std::future::pending::<()>().await;
                                    drop(conn);
                                }
                            }
                        }
                    }
                });
            }
        })
        .ok();

    // Connect GTK activate
    let win_holder = window_holder.clone();
    let rx_rc = rx_holder.clone();
    app.connect_activate(move |app| {
        babydra_ui_kit::ui::theme::init_theme();

        let app_c = app.clone();
        let win_holder_c = win_holder.clone();

        // Process incoming commands from DBus or Tray in GLib main loop
        if let Some(mut rx) = rx_rc.borrow_mut().take() {
            gtk4::glib::MainContext::default().spawn_local(async move {
                while let Some(cmd) = rx.recv().await {
                    match cmd {
                        RecorderCommand::ShowUI => {
                            show_or_create_control_window(&app_c, win_holder_c.clone());
                        }
                        RecorderCommand::Start => {
                            let _ = babydra_core::services::recording::start_recording(
                                &babydra_core::models::recording::RecordingConfig::default(),
                            );
                        }
                        RecorderCommand::Stop => {
                            let _ = babydra_core::services::recording::stop_recording();
                        }
                        RecorderCommand::Toggle => {
                            let _ = babydra_core::services::recording::toggle_recording(None);
                        }
                        RecorderCommand::Pause => {
                            let _ = babydra_core::services::recording::pause_recording();
                        }
                        RecorderCommand::Resume => {
                            let _ = babydra_core::services::recording::resume_recording();
                        }
                        RecorderCommand::TogglePause => {
                            let _ = babydra_core::services::recording::toggle_pause();
                        }
                        RecorderCommand::RecordOutput(output_name) => {
                            let config = babydra_core::models::recording::RecordingConfig {
                                mode: babydra_core::models::recording::RecordingMode::SingleOutput(
                                    output_name,
                                ),
                                ..Default::default()
                            };
                            let _ = babydra_core::services::recording::start_recording(&config);
                        }
                        RecorderCommand::RecordArea => {
                            if let Some(geom) =
                                babydra_core::services::recording::select_geometry_str_with_slurp()
                            {
                                let config = babydra_core::models::recording::RecordingConfig {
                                    mode: babydra_core::models::recording::RecordingMode::Window(
                                        geom,
                                    ),
                                    ..Default::default()
                                };
                                let _ = babydra_core::services::recording::start_recording(&config);
                            }
                        }
                        RecorderCommand::RecordWindow(geom) => {
                            let config = babydra_core::models::recording::RecordingConfig {
                                mode: babydra_core::models::recording::RecordingMode::Window(geom),
                                ..Default::default()
                            };
                            let _ = babydra_core::services::recording::start_recording(&config);
                        }
                    }
                }
            });
        }

        // Periodic watcher check: keep StatusNotifierItem registered
        gtk4::glib::timeout_add_seconds_local(30, move || {
            gtk4::glib::ControlFlow::Continue
        });

        if !is_daemon {
            show_or_create_control_window(app, win_holder.clone());
        }
    });

    app.run_with_args(&["babydra-recorder"]);
}
