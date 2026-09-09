use std::process::Command;

pub fn dispatch_compositor_switch(id: u32) {
    let key_str = id.to_string();

    let home = std::env::var("HOME").unwrap_or_default();
    let local_wtype = format!("{}/.local/bin/wtype", home);
    let wtype_paths = [local_wtype.as_str(), "wtype"];
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

    let _ = Command::new("wlrctl")
        .args(&["keyboard", "type", &key_str, "SUPER"])
        .status();
}
