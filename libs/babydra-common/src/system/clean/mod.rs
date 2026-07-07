//! Cleaner subsystem module wrapper.

pub mod helper;
pub mod cache;
pub mod pacman;
pub mod logs;
pub mod trash;

pub use helper::{get_dir_size, get_dir_size_native, format_bytes, is_dir_writable};
pub use cache::{get_user_cache_size, remove_user_cache};
pub use pacman::{get_pacman_cache_size, remove_pacman_cache, get_orphans_size};
pub use logs::{get_journal_logs_size, remove_journal_logs};
pub use trash::{get_trash_size, remove_trash};

pub fn clean_all_native() -> u64 {
    let mut freed_bytes = 0;
    freed_bytes += remove_user_cache();
    freed_bytes += remove_pacman_cache();
    freed_bytes += remove_journal_logs();
    freed_bytes += remove_trash();
    freed_bytes
}
