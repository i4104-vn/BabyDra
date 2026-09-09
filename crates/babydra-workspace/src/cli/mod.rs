pub mod commands;

use babydra_core::services::workspace::{next_workspace_sync_only, prev_workspace_sync_only};
use commands::{
    handle_current, handle_list, handle_next, handle_prev, handle_reset, handle_set,
    handle_switch, print_help,
};

pub fn run_cli() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match cmd {
        "list" | "ls" => handle_list(),
        "current" => handle_current(),
        "switch" | "goto" => handle_switch(args.get(2).map(|s| s.as_str())),
        "set" | "sync" => handle_set(args.get(2).map(|s| s.as_str())),
        "reset" => handle_reset(),
        "next" => handle_next(),
        "prev" => handle_prev(),
        "next-sync" => {
            let id = next_workspace_sync_only();
            println!("Cached workspace set to {}", id);
        }
        "prev-sync" => {
            let id = prev_workspace_sync_only();
            println!("Cached workspace set to {}", id);
        }
        _ => print_help(),
    }
}
