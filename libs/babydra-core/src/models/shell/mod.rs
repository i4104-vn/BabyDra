//! Core Shell, Island, Theme & Desktop State Models (Re-exports from topic submodules).

pub use crate::models::cli::*;
pub use crate::models::desktop::*;
pub use crate::models::island::*;
pub use crate::models::network::*;
pub use crate::models::system::*;
pub use crate::models::theme::*;
pub use crate::models::tray::*;

// Module aliases for backward compatibility with `crate::models::shell::<module>::...`
pub mod app {
    pub use crate::models::desktop::app::*;
}
pub mod appearance {
    pub use crate::models::theme::appearance::*;
}
pub mod battery {
    pub use crate::models::system::battery::*;
}
pub mod cli {
    pub use crate::models::cli::*;
}
pub mod daemon {
    pub use crate::models::system::daemon::*;
}
pub mod dbus_menu {
    pub use crate::models::tray::dbus_menu::*;
}
pub mod desktop_state {
    pub use crate::models::desktop::desktop_state::*;
}
pub mod exif {
    pub use crate::models::desktop::exif::*;
}
pub mod island_state {
    pub use crate::models::island::island_state::*;
}
pub mod monitor {
    pub use crate::models::system::monitor::*;
}
pub mod network {
    pub use crate::models::network::*;
}
pub mod notification {
    pub use crate::models::island::notification::*;
}
pub mod power {
    pub use crate::models::system::power::*;
}
pub mod shell_config {
    pub use crate::models::theme::shell_config::*;
}
pub mod storage {
    pub use crate::models::system::storage::*;
}
pub mod theme_config {
    pub use crate::models::theme::theme_config::*;
}
pub mod tray_item {
    pub use crate::models::tray::tray_item::*;
}
pub mod tray_snapshot {
    pub use crate::models::tray::tray_snapshot::*;
}
pub mod volume {
    pub use crate::models::system::volume::*;
}
pub mod workspace {
    pub use crate::models::desktop::workspace::*;
}
