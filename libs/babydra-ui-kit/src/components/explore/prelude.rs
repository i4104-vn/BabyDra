//! One-stop import for the most commonly used Explore API.
//!
//! Re-exports the public surface of every feature module: context menus,
//! dialogs, drag & drop, helpers, file items, rubberband selection and
//! widgets. Modules remain available for deeper access.
//!
//! Kept separate from the crate-level [`crate::prelude`] so names shared with
//! the generic component API (e.g. `create_list_row`) do not clash.

pub use crate::components::context_menu::{
    create_footer_box, create_footer_btn, create_menu_item, create_menu_sep, ContextMenuBuilder,
};
pub use crate::components::explore::context_menu::{
    clipboard::{
        execute_paste, execute_undo, paste_from_clipboard, set_clipboard_files, UndoOperation,
    },
    custom_items::append_custom_items,
    dimming::{apply_cut_dimming, apply_cut_everywhere},
    file_actions::{show_for_file_normal, show_for_file_trash},
    show_for_empty, show_for_file, CLIPBOARD,
};
pub use crate::components::explore::dialogs::{
    archive::show_compress_log,
    decompress::{show_decompress_log, show_password_dialog},
    decompress_async,
    properties::helpers::{count_dialog_height, count_dir_contents, get_perm_string},
    properties::permissions::{apply_permissions, build_perm_matrix, PermissionCheckboxes},
    show_alert_dialog, show_compress_dialog, show_conflict_dialog, show_delete_confirm,
    show_folder_dialog, show_new_file_dialog, show_properties, show_rename_dialog,
};
pub use crate::components::explore::drag::{
    create_bg_drop, create_drag_source, create_drop_nav, create_drop_target,
};
pub use crate::components::explore::helpers::{
    format_date, format_size, is_archive_file, is_in_trash, parse_target_dir, restore_from_trash,
    sanitize_path,
};
pub use crate::components::explore::items::{create_grid_file, create_list_row};
pub use crate::components::explore::selection::{wire_rubberband, wire_rubberband_grid};
pub use crate::components::explore::widgets::update_folder_btn;
