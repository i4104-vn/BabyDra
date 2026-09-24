pub mod fs;
pub mod git;
pub mod process;
pub mod sudo;
pub mod workspace;

pub use fs::{copy_recursive, format_size, safe_copy_binary};
pub use git::{branch_worktree_dir, checkout_and_pull, list_branches, refresh_branches};
pub use process::{is_root, stop_process};
pub use sudo::{spawn_and_stream, tail_lines, CmdOutput, SudoSession, MAX_PASSWORD_ATTEMPTS};
pub use workspace::{
    build_workspace, build_workspace_streaming, default_binary_source_dir, expand_path,
    find_workspace_root, get_user_home, get_user_local_bin,
};
