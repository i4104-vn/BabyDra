//! Reusable, unified Context Menu component for Desktop, Tray, and Explore.
//!
//! Provides a fluent [`ContextMenuBuilder`] along with standalone builders for popovers,
//! menu items, shortcuts, group headers, separators, footer action bars, and system tray integration.

pub mod builder;
pub mod items;
pub mod tray;

pub use builder::ContextMenuBuilder;
pub use items::{
    create_danger_btn, create_danger_item, create_footer_box, create_footer_btn,
    create_group_header, create_menu_for, create_menu_full, create_menu_item, create_menu_popover,
    create_menu_sens, create_menu_sep, create_menu_shortcut, create_menu_text, create_submenu_item,
};
pub use tray::{build_tray_gio_menu, close_tray_menu, show_tray_menu};
