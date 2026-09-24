use crate::actions::runner::CommandRunner;
use crate::config::BinariesConfig;
use crate::utils::fs::{mkdir_p, set_executable};
use crate::utils::system::get_local_bin_dir;
use std::fs;
use std::path::Path;

pub fn install_binaries(
    runner: &CommandRunner,
    repo_root: &Path,
    config: &BinariesConfig,
) -> Result<(), String> {
    let local_bin = get_local_bin_dir();
    let release_dir = repo_root.join("target/release");

    runner.step("Installing updated binaries to ~/.local/bin and /usr/bin...");
    mkdir_p(&local_bin)?;

    for bin in &config.user_binaries {
        let src_bin = release_dir.join(bin);
        let dest_bin = local_bin.join(bin);

        ensure_release_binary(&src_bin, bin)?;
        fs::copy(&src_bin, &dest_bin)
            .map_err(|e| format!("Failed to install {} to {}: {}", bin, dest_bin.display(), e))?;
        set_executable(&dest_bin)
            .map_err(|e| format!("Failed to make {} executable: {}", dest_bin.display(), e))?;
        runner.log(format!("Installed: ~/.local/bin/{}", bin));
    }

    for sys_bin in &config.system_binaries {
        let src_bin = release_dir.join(sys_bin);
        let dest_path = format!("/usr/bin/{}", sys_bin);
        let src_path = src_bin
            .to_str()
            .ok_or_else(|| format!("Invalid binary path: {}", src_bin.display()))?;

        ensure_release_binary(&src_bin, sys_bin)?;
        runner.run_sudo("cp", &[src_path, &dest_path], None)?;
        runner.run_sudo("chmod", &["+x", &dest_path], None)?;
        runner.log(format!("Installed system binary: {}", dest_path));
    }

    Ok(())
}

fn ensure_release_binary(path: &std::path::Path, name: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!(
            "Release binary '{}' was not found at {}. Build the workspace first.",
            name,
            path.display()
        ))
    }
}
