use crate::actions::runner::CommandRunner;
use crate::utils::fs::set_executable;
use crate::utils::process::command_exists;
use crate::utils::system::get_local_bin_dir;
use std::fs;
use std::path::Path;

pub fn ensure_yay_installed(runner: &CommandRunner) -> Result<(), String> {
    if !command_exists("yay") {
        runner.step("Installing yay AUR helper...");
        runner.run_cmd("rm", &["-rf", "/tmp/yay-bin"], None)?;
        runner.run_cmd(
            "git",
            &[
                "clone",
                "https://aur.archlinux.org/yay-bin.git",
                "/tmp/yay-bin",
            ],
            None,
        )?;
        runner.run_cmd(
            "makepkg",
            &["-si", "--noconfirm"],
            Some(Path::new("/tmp/yay-bin")),
        )?;
    }
    Ok(())
}

pub fn ensure_wtype_installed(runner: &CommandRunner) -> Result<(), String> {
    let local_bin = get_local_bin_dir();
    let wtype_bin = local_bin.join("wtype");

    if !wtype_bin.exists() {
        runner.step("Compiling wtype from source...");
        runner.run_cmd("rm", &["-rf", "/tmp/wtype"], None)?;
        runner.run_cmd(
            "git",
            &["clone", "https://github.com/atx/wtype.git", "/tmp/wtype"],
            None,
        )?;

        let tmp_wtype = Path::new("/tmp/wtype");
        runner.run_cmd("meson", &["setup", "build"], Some(tmp_wtype))?;
        runner.run_cmd("ninja", &["-C", "build"], Some(tmp_wtype))?;
        fs::copy("/tmp/wtype/build/wtype", &wtype_bin)
            .map_err(|e| format!("Failed to install wtype: {}", e))?;
        set_executable(&wtype_bin)
            .map_err(|e| format!("Failed to make wtype executable: {}", e))?;
    }
    Ok(())
}
