use crate::actions::runner::CommandRunner;
use crate::config::UpdaterConfig;
use crate::core::state::ComponentTarget;
use crate::utils::fs::remove_file_if_exists;
use crate::utils::process::kill_process;
use crate::utils::system::{get_local_bin_dir, gsettings_set, refresh_fc_cache};
use std::path::Path;
use std::process::Command;

pub fn execute_component_restart(
    runner: &CommandRunner,
    target: ComponentTarget,
    config: &UpdaterConfig,
) -> Result<(), String> {
    let local_bin = get_local_bin_dir();

    match target {
        ComponentTarget::Panel => {
            runner.step("Restarting babydra-panel...");
            restart_binary(
                runner,
                &local_bin,
                "babydra-panel",
                &[],
                "babydra-panel restarted successfully.",
                "~/.local/bin/babydra-panel not found. Please run Update & Reload first.",
            )?;
        }
        ComponentTarget::Desktop => {
            runner.step("Restarting babydra-desktop...");
            restart_binary(
                runner,
                &local_bin,
                "babydra-desktop",
                &[],
                "babydra-desktop restarted successfully.",
                "~/.local/bin/babydra-desktop not found.",
            )?;
        }
        ComponentTarget::Switcher => {
            runner.step("Restarting babydra-switcher daemon...");
            remove_file_if_exists(Path::new("/tmp/babydra-switcher.socket"));
            restart_binary(
                runner,
                &local_bin,
                "babydra-switcher",
                &["--daemon"],
                "babydra-switcher daemon restarted.",
                "~/.local/bin/babydra-switcher not found.",
            )?;
        }
        ComponentTarget::Keymap => {
            runner.step("Restarting babydra-keymap daemon...");
            restart_binary(
                runner,
                &local_bin,
                "babydra-keymap",
                &[],
                "babydra-keymap restarted successfully.",
                "~/.local/bin/babydra-keymap not found.",
            )?;
        }
        ComponentTarget::ReconfigureLabwc => {
            runner.step("Reloading labwc compositor configurations (labwc --reconfigure)...");
            runner.run_cmd("labwc", &["--reconfigure"], None)?;
            runner.success("labwc reloaded.");
        }
        ComponentTarget::RefreshGtkFonts => {
            runner.step("Re-applying GTK gsettings and font caches...");
            let g = &config.gsettings;
            gsettings_set("org.gnome.desktop.interface", "font-name", &g.font_name);
            gsettings_set("org.gnome.desktop.interface", "icon-theme", &g.icon_theme);
            gsettings_set(
                "org.gnome.desktop.interface",
                "cursor-theme",
                &g.cursor_theme,
            );
            let cursor_size = g.cursor_size.to_string();
            gsettings_set("org.gnome.desktop.interface", "cursor-size", &cursor_size);
            refresh_fc_cache();
            runner.success("GTK theme and font cache refreshed.");
        }
    }

    Ok(())
}

fn restart_binary(
    runner: &CommandRunner,
    local_bin: &Path,
    name: &str,
    args: &[&str],
    success_message: &str,
    missing_message: &str,
) -> Result<(), String> {
    kill_process(name);
    let binary = local_bin.join(name);
    if !binary.is_file() {
        runner.error(missing_message);
        return Err(missing_message.to_string());
    }

    Command::new(&binary)
        .args(args)
        .spawn()
        .map_err(|e| format!("Failed to spawn {}: {}", name, e))?;
    runner.success(success_message);
    Ok(())
}
