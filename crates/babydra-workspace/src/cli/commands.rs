use babydra_workspace::manager::{
    get_current_workspace, get_workspaces, next_workspace, prev_workspace, switch_workspace,
    DEFAULT_WORKSPACE_COUNT,
};

pub fn handle_list() {
    let active = get_current_workspace();
    for ws in get_workspaces() {
        let marker = if ws.id == active { "*" } else { " " };
        println!("{} {}: {}", marker, ws.id, ws.name);
    }
}

pub fn handle_current() {
    let active = get_current_workspace();
    println!("{}", active);
}

pub fn handle_switch(target_str: Option<&str>) {
    if let Some(target) = target_str.and_then(|s| s.parse::<u32>().ok()) {
        if switch_workspace(target) {
            println!("Switched to workspace {}", target);
        } else {
            eprintln!(
                "Invalid workspace ID: {}. Must be 1..={}",
                target, DEFAULT_WORKSPACE_COUNT
            );
            std::process::exit(1);
        }
    } else {
        eprintln!(
            "Usage: babydra-workspace switch <1..={}>",
            DEFAULT_WORKSPACE_COUNT
        );
        std::process::exit(1);
    }
}

pub fn handle_next() {
    let new_id = next_workspace();
    println!("Switched to workspace {}", new_id);
}

pub fn handle_prev() {
    let new_id = prev_workspace();
    println!("Switched to workspace {}", new_id);
}

pub fn print_help() {
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
    println!("  --daemon, -d      Run background daemon for instant overlay");
}
