//! Explore feature kit: dialogs, context menus, drag & drop and file item
//! builders that were historically part of `babydra-ui-kit`.
//!
//! Splitting this out (planning.md Phase 3 T3.1) keeps `babydra-ui-kit` a
//! pure UI-kit and gives Explore a crate of its own — smaller rebuilds and
//! clearer ownership.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use babydra_explore_kit::prelude::*;
//!
//! let card = create_grid_file_item(&entry);
//! show_new_folder_dialog(current_path, nav_callback);
//! ```

pub mod explore;

pub use explore::*;

/// One-stop import for the most commonly used Explore API.
///
/// Re-exports the public surface of every feature module: context menus,
/// dialogs, drag & drop, helpers, file items, rubberband selection and
/// widgets. Modules remain available for deeper access.
pub mod prelude {
    pub use crate::explore::context_menu::{
        clipboard::{
            execute_paste, execute_paste_from_system_clipboard, execute_undo,
            set_system_clipboard_files, UndoOperation,
        },
        custom_items::append_custom_context_items,
        dimming::{apply_cut_dimming, apply_cut_dimming_global},
        file_actions::{show_for_file_normal, show_for_file_trash},
        show_for_empty, show_for_file,
        widgets::{
            create_footer_container, create_footer_icon_button, create_menu_button,
            create_menu_popover,
        },
        CLIPBOARD,
    };
    pub use crate::explore::dialogs::{
        archive::show_compress_log_dialog,
        decompress::{show_decompress_log_dialog, show_password_dialog},
        perform_decompress_async,
        properties::helpers::{
            count_dialog_height, count_dir_contents_recursive, get_permissions_string,
        },
        properties::permissions::{
            apply_permissions, build_permission_matrix, PermissionCheckboxes,
        },
        show_alert_dialog, show_compress_dialog, show_conflict_dialog, show_delete_confirm_dialog,
        show_new_file_dialog, show_new_folder_dialog, show_properties_dialog, show_rename_dialog,
    };
    pub use crate::explore::drag::{
        create_background_drop_target, create_dir_drop_target, create_dir_drop_target_with_nav,
        create_drag_source,
    };
    pub use crate::explore::helpers::{
        format_date, format_size, is_archive_file, is_in_trash, parse_target_dir,
        restore_from_trash, sanitize_path,
    };
    pub use crate::explore::items::{create_grid_file_item, create_list_row};
    pub use crate::explore::selection::{wire_rubberband_grid, wire_rubberband_listbox};
    pub use crate::explore::widgets::update_new_folder_button;
}
