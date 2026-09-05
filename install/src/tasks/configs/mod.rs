pub mod compositor;
pub mod desktop_entries;
pub mod dotfiles;
pub mod services;
pub mod themes;

use std::path::Path;

use crate::models::{GenericOptionItem, LogLevel};
use crate::system::SudoSession;

pub use themes::{deploy_theme_packages, write_theme_selection};

pub fn execute_configs_task<F>(
    opt: &GenericOptionItem,
    workspace_root: &Path,
    sudo: &SudoSession,
    log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let copied = match opt.id.as_str() {
        "labwc_configs" => compositor::sync_labwc_configs(workspace_root, log),
        "desktop_entries" => desktop_entries::register_desktop_entries(sudo, log),
        "dotfiles_gtk_terminal" => dotfiles::sync_dotfiles(workspace_root, log),
        "themes_icons_cursors" => {
            themes::install_themes_icons_cursors(workspace_root, sudo, log)
        }
        "gsettings_fontcache" => themes::apply_gsettings_fontcache(sudo, log),
        "restart_services" => services::restart_services(sudo, log),
        _ => 0,
    };

    (copied, 0)
}
