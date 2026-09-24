use crate::actions::runner::CommandRunner;
use std::path::Path;

pub fn build_release(runner: &CommandRunner, repo_root: &Path) -> Result<(), String> {
    runner.step("Compiling workspace in release mode (cargo build --release)...");
    runner.run_cmd("cargo", &["build", "--release"], Some(repo_root))?;
    runner.success("Workspace compiled successfully.");
    Ok(())
}
