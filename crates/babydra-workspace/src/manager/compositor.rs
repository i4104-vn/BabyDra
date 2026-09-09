use std::process::Command;

/// Dispatches compositor desktop switch command to labwc.
pub fn dispatch_compositor_switch(id: u32) {
    let key_str = id.to_string();

    // 1. Try wtype: Super + <id>
    let wtype_paths = ["/home/i4104/.local/bin/wtype", "wtype"];
    for bin in &wtype_paths {
        if let Ok(status) = Command::new(bin)
            .args(&["-M", "logo", "-k", &key_str, "-m", "logo"])
            .status()
        {
            if status.success() {
                return;
            }
        }
    }

    // 2. Fallback to wlrctl keyboard type
    let _ = Command::new("wlrctl")
        .args(&["keyboard", "type", &key_str, "SUPER"])
        .status();
}
