use crate::actions::runner::CommandRunner;
use crate::utils::process::is_process_running;
use crate::utils::system::{gsettings_set, refresh_fc_cache};
use std::collections::HashMap;

pub fn apply_desktop_gsettings(
    runner: &CommandRunner,
    gsettings: &HashMap<String, String>,
) -> Result<(), String> {
    runner.step("Applying GNOME/GTK gsettings from workspace.toml...");
    for (full_key, val) in gsettings {
        if let Some(pos) = full_key.rfind('.') {
            let schema = &full_key[..pos];
            let key = &full_key[pos + 1..];
            gsettings_set(schema, key, val);
        }
    }

    runner.step("Refreshing font cache...");
    refresh_fc_cache();

    if is_process_running("labwc") {
        runner.step("Reconfiguring running labwc compositor...");
        runner.run_cmd("labwc", &["--reconfigure"], None)?;
        runner.success("labwc reconfigured successfully!");
    } else {
        runner.log("Notice: labwc is not currently running.");
    }

    Ok(())
}
