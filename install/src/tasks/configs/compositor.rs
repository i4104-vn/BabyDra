use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::models::LogLevel;
use crate::system::{copy_recursive, get_user_home};

pub fn sync_labwc_configs<F>(workspace_root: &Path, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let labwc_src = workspace_root.join("configs/labwc");
    let labwc_dst = home.join(".config/labwc");

    if labwc_src.exists() {
        let _ = copy_recursive(&labwc_src, &labwc_dst);

        let autostart_file = labwc_dst.join("autostart");
        if autostart_file.exists() {
            let mut perms = fs::metadata(&autostart_file)
                .map(|m| m.permissions())
                .unwrap_or_else(|_| fs::Permissions::from_mode(0o755));
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&autostart_file, perms);
        }

        let scripts_dir = labwc_dst.join("scripts");
        if scripts_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&scripts_dir) {
                for e in entries.flatten() {
                    if let Ok(mut p) = fs::metadata(e.path()).map(|m| m.permissions()) {
                        p.set_mode(0o755);
                        let _ = fs::set_permissions(e.path(), p);
                    }
                }
            }
        }
        log(
            LogLevel::Success,
            "Synced labwc autostart, rc.xml, scripts to ~/.config/labwc".into(),
        );
        copied += 1;
    }

    copied
}
