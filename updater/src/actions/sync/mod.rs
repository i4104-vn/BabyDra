pub mod assets;
pub mod desktop;
pub mod gsettings;
pub mod themes;

use crate::actions::runner::CommandRunner;
use crate::config::UpdaterConfig;
use std::path::Path;

pub fn sync_all_configs(
    runner: &CommandRunner,
    repo_root: &Path,
    config: &UpdaterConfig,
) -> Result<(), String> {
    assets::sync_assets(runner, repo_root)?;
    desktop::sync_desktop_configs(runner, repo_root)?;
    themes::sync_themes(runner, repo_root)?;
    gsettings::apply_desktop_gsettings(runner, &config.gsettings)?;

    runner.success("All configurations and themes synchronized successfully.");
    Ok(())
}
