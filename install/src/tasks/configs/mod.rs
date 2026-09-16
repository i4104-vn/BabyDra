pub mod compositor;
pub mod desktop_entries;
pub mod dotfiles;
pub mod services;
pub mod themes;

use std::path::Path;

use crate::models::{BinaryItem, GenericOptionItem, LogLevel};
use crate::system::{InstallManifest, SudoSession};

pub use themes::{deploy_theme_packages, write_theme_selection};

pub fn execute_configs_task<F>(
    opt: &GenericOptionItem,
    workspace_root: &Path,
    binaries: &[BinaryItem],
    manifest: &InstallManifest,
    sudo: &SudoSession,
    log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let copied = match opt.id.as_str() {
        "labwc_configs" => compositor::sync_labwc_configs(workspace_root, log),
        "desktop_entries" => {
            desktop_entries::register_desktop_entries(workspace_root, binaries, sudo, log)
        }
        "dotfiles_gtk_terminal" => dotfiles::sync_dotfiles(workspace_root, log),
        "themes_icons_cursors" => themes::install_themes_icons_cursors(workspace_root, sudo, log),
        "gsettings_fontcache" => themes::apply_gsettings_fontcache(manifest, sudo, log),
        "restart_services" => services::restart_services(workspace_root, sudo, log),
        _ => 0,
    };

    (copied, 0)
}
