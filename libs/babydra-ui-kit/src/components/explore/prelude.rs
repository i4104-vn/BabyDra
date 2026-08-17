//! One-stop import for the most commonly used Explore API.
//!
//! Re-exports the public surface of every feature module: context menus,
//! dialogs, drag & drop, helpers, file items, rubberband selection and
//! widgets. Modules remain available for deeper access.
//!
//! Kept separate from the crate-level [`crate::prelude`] so names shared with
//! the generic component API (e.g. `create_list_row`) do not clash.

pub use crate::components::explore::context_menu::{
    clipboard::{
        execute_paste, execute_paste_from_system_clipboard, execute_undo,
        set_system_clipboard_files, UndoOperation,
    },
    custom_items::append_custom_context_items,
    dimming::{apply_cut_dimming, apply_cut_dimming_global},
    file_actions::{show_for_file_normal, show_for_file_trash},
    show_for_empty, show_for_file,
    widgets::{
        create_footer_container, create_footer_icon_button, create_menu_button, create_menu_popover,
    },
    CLIPBOARD,
};
pub use crate::components::explore::dialogs::{
    archive::show_compress_log_dialog,
    decompress::{show_decompress_log_dialog, show_password_dialog},
    perform_decompress_async,
    properties::helpers::{
        count_dialog_height, count_dir_contents_recursive, get_permissions_string,
    },
    properties::permissions::{apply_permissions, build_permission_matrix, PermissionCheckboxes},
    show_alert_dialog, show_compress_dialog, show_conflict_dialog, show_delete_confirm_dialog,
    show_new_file_dialog, show_new_folder_dialog, show_properties_dialog, show_rename_dialog,
};
pub use crate::components::explore::drag::{
    create_background_drop_target, create_dir_drop_target, create_dir_drop_target_with_nav,
    create_drag_source,
};
pub use crate::components::explore::helpers::{
    format_date, format_size, is_archive_file, is_in_trash, parse_target_dir, restore_from_trash,
    sanitize_path,
};
pub use crate::components::explore::items::{create_grid_file_item, create_list_row};
pub use crate::components::explore::selection::{wire_rubberband_grid, wire_rubberband_listbox};
pub use crate::components::explore::widgets::update_new_folder_button;
