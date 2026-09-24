use crate::actions::runner::CommandRunner;
use crate::utils::process::command_exists;
use std::env;
use std::path::Path;

pub fn execute_check(runner: &CommandRunner, repo_root: &Path) -> Result<(), String> {
    runner.step("Running BabyDra safety-net checks...");

    // 1. Cargo check
    runner.step("Running compiler type check (cargo check --all-targets)...");
    runner.run_cmd("cargo", &["check", "--all-targets"], Some(repo_root))?;
    runner.success("Type check passed.");

    // 2. Clippy
    runner.step("Running linter (cargo clippy --all-targets -- -D warnings)...");
    runner.run_cmd(
        "cargo",
        &["clippy", "--all-targets", "--", "-D", "warnings"],
        Some(repo_root),
    )?;
    runner.success("Clippy checks passed with 0 warnings.");

    // 3. Tests
    runner.step("Running test suite (cargo test)...");
    let has_display = env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok();
    let has_xvfb = command_exists("xvfb-run");

    if !has_display && has_xvfb {
        runner.log("Notice: No display detected. Running tests under xvfb-run virtual display...");
        runner.run_cmd("xvfb-run", &["-a", "cargo", "test"], Some(repo_root))?;
    } else {
        runner.run_cmd("cargo", &["test"], Some(repo_root))?;
    }

    runner.success("All tests passed successfully!");
    Ok(())
}
