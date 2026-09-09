pub mod commands;
pub mod oneshot;

use babydra_workspace::daemon::{run_daemon, try_signal_daemon};
use commands::{handle_current, handle_list, handle_next, handle_prev, handle_switch, print_help};
use oneshot::run_oneshot_ui;

/// Main CLI entry point.
pub fn run_cli() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match cmd {
        "list" | "ls" => handle_list(),
        "current" => handle_current(),
        "switch" | "goto" => handle_switch(args.get(2).map(|s| s.as_str())),
        "next" => handle_next(),
        "prev" => handle_prev(),
        "show" | "toggle" => {
            if !try_signal_daemon(b"toggle") {
                run_oneshot_ui();
            }
        }
        "--daemon" | "-d" => {
            run_daemon();
        }
        _ => print_help(),
    }
}
