use crate::actions::runner::CommandRunner;
use crate::config::WorkspaceBinary;
use crate::utils::fs::{mkdir_p, set_executable};
use crate::utils::system::get_local_bin_dir;
use std::fs;
use std::path::Path;

pub fn install_binaries(
    runner: &CommandRunner,
    repo_root: &Path,
    binaries: &[WorkspaceBinary],
) -> Result<(), String> {
    let local_bin = get_local_bin_dir();
    let release_dir = repo_root.join("target/release");

    runner.step("Installing updated binaries defined in workspace.toml...");
    mkdir_p(&local_bin)?;

    for bin in binaries {
        let src_bin = release_dir.join(&bin.name);
        ensure_release_binary(&src_bin, &bin.name)?;

        if bin.scope == "system" {
            let dest_path = format!("/usr/bin/{}", bin.name);
            let src_path = src_bin
                .to_str()
                .ok_or_else(|| format!("Invalid binary path: {}", src_bin.display()))?;

            runner.run_sudo("cp", &[src_path, &dest_path], None)?;
            runner.run_sudo("chmod", &["+x", &dest_path], None)?;
            runner.log(format!("Installed system binary: {}", dest_path));
        } else {
            let dest_bin = local_bin.join(&bin.name);
            fs::copy(&src_bin, &dest_bin)
                .map_err(|e| format!("Failed to install {} to {}: {}", bin.name, dest_bin.display(), e))?;
            set_executable(&dest_bin)
                .map_err(|e| format!("Failed to make {} executable: {}", dest_bin.display(), e))?;
            runner.log(format!("Installed: ~/.local/bin/{}", bin.name));

            if bin.name == "babydra-filedialog" {
                let xdg_open_symlink = local_bin.join("xdg-open");
                let _ = fs::remove_file(&xdg_open_symlink);
                let _ = std::os::unix::fs::symlink(&dest_bin, &xdg_open_symlink);
                runner.log("Linked ~/.local/bin/xdg-open -> babydra-filedialog");

                let _ = runner.run_sudo("ln", &["-sf", "/usr/local/bin/babydra-filedialog", "/usr/local/bin/xdg-open"], None);
                let _ = runner.run_sudo("sh", &["-c", "if [ ! -L /usr/bin/xdg-open ] && [ -f /usr/bin/xdg-open ]; then mv /usr/bin/xdg-open /usr/bin/xdg-open.orig; fi"], None);
                let _ = runner.run_sudo("ln", &["-sf", "/usr/bin/babydra-filedialog", "/usr/bin/xdg-open"], None);
            }
        }
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
