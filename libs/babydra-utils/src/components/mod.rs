pub mod alerts;
pub mod badge;
pub mod buttons;
pub mod card;
pub mod close_button;
pub mod list_group;
pub mod modal;
pub mod navbar;
pub mod popovers;
pub mod progress;
pub mod spinners;
pub mod switch;
pub mod tooltips;

// Re-export all builders under components namespace to maintain compatibility
pub use alerts::create_placeholder_message;
pub use badge::create_status_badge;
pub use buttons::{create_button, create_accent_button, create_fab, create_icon_button, create_colored_icon_button, create_icon_label_button, create_toggle_tile, create_square_toggle_tile, create_sidebar_item_button};
pub use card::{create_card, create_card_with_class, create_title, create_subtitle, create_item_row, create_switch_card, create_scrollable_list, create_grid_file_item};
pub use close_button::{create_close_button, create_close_button_with_label};
pub use modal::create_dialog_box;
pub use navbar::create_sidebar_row;
pub use popovers::{create_popover, create_popover_with_content};
pub use progress::{create_progress_bar, create_disk_progress};
pub use spinners::{create_spinner, create_loading_box};
pub use tooltips::set_tooltip;
pub use list_group::create_list_row;
pub use switch::create_switch;
