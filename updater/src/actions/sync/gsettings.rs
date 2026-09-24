use crate::actions::runner::CommandRunner;
use crate::config::GSettingsConfig;
use crate::utils::process::is_process_running;
use crate::utils::system::{gsettings_set, refresh_fc_cache};

pub fn apply_desktop_gsettings(
    runner: &CommandRunner,
    config: &GSettingsConfig,
) -> Result<(), String> {
    runner.step("Applying GNOME/GTK gsettings...");
    gsettings_set(
        "org.gnome.desktop.interface",
        "font-name",
        &config.font_name,
    );
    gsettings_set(
        "org.gnome.desktop.interface",
        "document-font-name",
        &config.document_font_name,
    );
    gsettings_set(
        "org.gnome.desktop.interface",
        "monospace-font-name",
        &config.monospace_font_name,
    );
    gsettings_set(
        "org.gnome.desktop.interface",
        "icon-theme",
        &config.icon_theme,
    );
    gsettings_set(
        "org.gnome.desktop.interface",
        "cursor-theme",
        &config.cursor_theme,
    );
    let cursor_size = config.cursor_size.to_string();
    gsettings_set("org.gnome.desktop.interface", "cursor-size", &cursor_size);

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
