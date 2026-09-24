use crate::actions::runner::CommandRunner;
use crate::utils::process::pkill;
use crate::utils::system::get_current_user;

pub fn stop_shell_processes(runner: &CommandRunner, is_dry_run: bool) {
    let user = get_current_user();
    runner.step("[Step 1/6] Stopping active BabyDra and compositor processes...");

    if !is_dry_run {
        pkill("babydra-", Some(&user));
        pkill("labwc", Some(&user));
        pkill("cage", None);
    } else {
        runner.log("[dry-run] Would kill babydra-*, labwc, cage");
    }
}
