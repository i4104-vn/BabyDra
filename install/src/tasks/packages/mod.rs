pub mod aur;
pub mod pacman;
pub mod system_perms;
pub mod wtype;

use crate::models::{GenericOptionItem, LogLevel};
use crate::system::SudoSession;

pub fn execute_packages_task<F>(
    opt: &GenericOptionItem,
    sudo: &SudoSession,
    log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    match opt.id.as_str() {
        "pacman_packages" => pacman::install_pacman_packages(sudo, log),
        "install_yay" => aur::ensure_yay_installed(sudo, log),
        "aur_packages" => aur::install_aur_packages(sudo, log),
        "build_wtype" => wtype::build_wtype_from_source(log),
        "kernel_permissions" => system_perms::configure_kernel_permissions(sudo, log),
        _ => (0, 0),
    }
}
