//! Cleaner subsystem module wrapper.

pub mod cache;
pub mod helper;
pub mod logs;
pub mod pacman;
pub mod trash;

pub use cache::{get_user_cache_size, remove_user_cache};
pub use helper::{format_bytes, get_dir_size, get_dir_size_native, is_dir_writable};
pub use logs::{get_journal_size, remove_journal_logs};
pub use pacman::{get_cache_size, get_orphans_size, remove_pacman_cache};
pub use trash::{get_trash_size, remove_trash};

/// Clean all native.
pub fn clean_all_native() -> u64 {
    let mut freed_bytes = 0;
    freed_bytes += remove_user_cache();
    freed_bytes += remove_pacman_cache();
    freed_bytes += remove_journal_logs();
    freed_bytes += remove_trash();
    freed_bytes
}
