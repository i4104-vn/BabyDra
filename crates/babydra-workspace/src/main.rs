use babydra_workspace::manager::{
    get_current_workspace, get_workspaces, next_workspace, prev_workspace, switch_workspace,
};
use babydra_workspace::switcher::{
    build_workspace_switcher_ui, try_signal_daemon, WORKSPACE_SOCKET_PATH,
};
use gtk4::prelude::*;
use std::io::Read;
use std::os::unix::net::UnixListener;
use std::sync::mpsc;
use std::thread;

fn main() {
    babydra_core::services::logger::init_logger("babydra-workspace", "babydra-workspace.log");

    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match cmd {
        "list" | "ls" => {
            let active = get_current_workspace();
            for ws in get_workspaces() {
                let marker = if ws.id == active { "*" } else { " " };
                println!("{} {}: {}", marker, ws.id, ws.name);
            }
        }
        "current" => {
            let active = get_current_workspace();
            println!("{}", active);
        }
        "switch" | "goto" => {
            if let Some(target) = args.get(2).and_then(|s| s.parse::<u32>().ok()) {
                if switch_workspace(target) {
                    println!("Switched to workspace {}", target);
                } else {
                    eprintln!("Invalid workspace ID: {}. Must be 1..=4", target);
                    std::process::exit(1);
                }
            } else {
                eprintln!("Usage: babydra-workspace switch <1..=4>");
                std::process::exit(1);
            }
        }
        "next" => {
            let new_id = next_workspace();
            println!("Switched to workspace {}", new_id);
        }
        "prev" => {
            let new_id = prev_workspace();
            println!("Switched to workspace {}", new_id);
        }
        "show" | "toggle" => {
            if !try_signal_daemon(b"toggle") {
                run_oneshot_ui();
            }
        }
        "--daemon" | "-d" => {
            run_daemon();
        }
        _ => {
            println!("BabyDra Workspace Manager");
            println!("Usage: babydra-workspace <command>");
            println!();
            println!("Commands:");
            println!("  list, ls          List all workspaces and active state");
            println!("  current           Print the active workspace ID");
            println!("  switch <id>       Switch to workspace by ID (1..=4)");
            println!("  next              Switch to the next workspace");
            println!("  prev              Switch to the previous workspace");
            println!("  show, toggle      Open/toggle the workspace switcher UI");
            println!("  --daemon          Run background daemon for instant overlay");
        }
    }
}

fn run_oneshot_ui() {
    let app = gtk4::Application::new(Some("org.babydra.workspace.oneshot"), Default::default());
    app.connect_activate(|app| {
        let controller = build_workspace_switcher_ui(app);
        (controller.show_fn)();
    });
    app.run_with_args(&["babydra-workspace"]);
}

fn run_daemon() {
    if std::os::unix::net::UnixStream::connect(WORKSPACE_SOCKET_PATH).is_ok() {
        eprintln!("[babydra-workspace] Daemon already running.");
        return;
    }

    let _ = std::fs::remove_file(WORKSPACE_SOCKET_PATH);

    let app = gtk4::Application::new(Some("org.babydra.workspace.daemon"), Default::default());
    app.connect_activate(|app| {
        let controller = build_workspace_switcher_ui(app);
        let show_fn = std::rc::Rc::new(controller.show_fn);
        let hide_fn = std::rc::Rc::new(controller.hide_fn);
        let window = controller.window;

        let (tx, rx) = mpsc::channel::<String>();

        // Socket listener thread
        thread::spawn(move || {
            let listener = match UnixListener::bind(WORKSPACE_SOCKET_PATH) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("[babydra-workspace daemon] Socket bind error: {e}");
                    return;
                }
            };

            for stream in listener.incoming() {
                if let Ok(mut s) = stream {
                    let mut buf = [0u8; 64];
                    if let Ok(n) = s.read(&mut buf) {
                        let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        let _ = tx.send(msg);
                    }
                }
            }
        });

        // Message pump in GTK main thread
        let show_c = show_fn.clone();
        let hide_c = hide_fn.clone();
        let win_c = window.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(20), move || {
            while let Ok(msg) = rx.try_recv() {
                match msg.as_str() {
                    "show" => {
                        show_c();
                    }
                    "hide" => {
                        hide_c();
                    }
                    "toggle" => {
                        if win_c.is_visible() {
                            hide_c();
                        } else {
                            show_c();
                        }
                    }
                    _ => {}
                }
            }
            glib::ControlFlow::Continue
        });
    });

    app.run_with_args(&["babydra-workspace"]);
}
