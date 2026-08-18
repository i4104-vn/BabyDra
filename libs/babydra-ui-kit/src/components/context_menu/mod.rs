//! Reusable, unified Context Menu component for Desktop, Tray, and Explore.
//!
//! Provides a fluent [`ContextMenuBuilder`] along with standalone builders for popovers,
//! menu items, shortcuts, group headers, separators, footer action bars, and system tray integration.

pub mod builder;
pub mod items;
pub mod tray;

pub use builder::ContextMenuBuilder;
pub use items::{
    create_context_menu_for_widget, create_context_menu_popover, create_footer_container,
    create_footer_icon_button, create_menu_group_header, create_menu_item,
    create_menu_item_destructive, create_menu_item_destructive_sensitive, create_menu_item_full,
    create_menu_item_sensitive, create_menu_item_with_shortcut, create_menu_separator,
};
pub use tray::{build_tray_gio_menu, close_active_tray_menu, show_tray_context_menu};
