//! Explore-specific context menu widget aliases and adapters, backed by unified `babydra_ui_kit::components::context_menu`.

pub use crate::components::context_menu::{
    create_context_menu_for_widget, create_context_menu_popover as create_menu_popover,
    create_footer_container, create_footer_icon_button, create_menu_group_header,
    create_menu_item as create_menu_button, create_menu_item_destructive,
    create_menu_item_with_shortcut, create_menu_separator, ContextMenuBuilder,
};
