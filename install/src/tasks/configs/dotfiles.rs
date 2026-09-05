use std::fs;
use std::path::Path;

use crate::models::LogLevel;
use crate::system::{copy_recursive, get_user_home};

pub fn sync_dotfiles<F>(workspace_root: &Path, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let labwc_src = workspace_root.join("configs/labwc");
    let settings_ini = labwc_src.join("settings.ini");
    let fonts_conf = labwc_src.join("fonts.conf");

    let gtk3 = home.join(".config/gtk-3.0");
    let gtk4 = home.join(".config/gtk-4.0");
    let fontconfig = home.join(".config/fontconfig");

    let _ = fs::create_dir_all(&gtk3);
    let _ = fs::create_dir_all(&gtk4);
    let _ = fs::create_dir_all(&fontconfig);

    if settings_ini.exists() {
        let _ = fs::copy(&settings_ini, gtk3.join("settings.ini"));
        let _ = fs::copy(&settings_ini, gtk4.join("settings.ini"));
    }
    if fonts_conf.exists() {
        let _ = fs::copy(&fonts_conf, fontconfig.join("fonts.conf"));
    }

    let kitty_src = workspace_root.join("configs/kitty");
    if kitty_src.exists() {
        let _ = copy_recursive(&kitty_src, &home.join(".config/kitty"));
    }
    let nvim_src = workspace_root.join("configs/nvim");
    if nvim_src.exists() {
        let _ = copy_recursive(&nvim_src, &home.join(".config/nvim"));
    }
    let ff_src = workspace_root.join("configs/fastfetch");
    if ff_src.exists() {
        let _ = copy_recursive(&ff_src, &home.join(".config/fastfetch"));
    }

    log(
        LogLevel::Success,
        "Synced GTK-3/4, Fontconfig, Kitty, Neovim, and Fastfetch dotfiles.".into(),
    );
    copied += 1;

    copied
}
