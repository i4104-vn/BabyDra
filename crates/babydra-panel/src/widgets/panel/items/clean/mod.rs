pub mod render;

pub use babydra_common::helper::clean::{clean_all_native, format_bytes};

pub mod cache {
    pub use babydra_common::helper::clean::{
        get_user_cache_size, get_pacman_cache_size,
    };
}

pub mod logs {
    pub use babydra_common::helper::clean::{
        get_journal_logs_size,
    };
}

pub mod temp {
    pub use babydra_common::helper::clean::{
        get_trash_size,
    };
}
