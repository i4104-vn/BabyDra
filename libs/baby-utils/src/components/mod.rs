pub mod accordion;
pub mod alerts;
pub mod badge;
pub mod breadcrumb;
pub mod buttons;
pub mod button_group;
pub mod card;
pub mod carousel;
pub mod close_button;
pub mod collapse;
pub mod dropdowns;
pub mod list_group;
pub mod modal;
pub mod navbar;
pub mod navs_tabs;
pub mod offcanvas;
pub mod pagination;
pub mod placeholders;
pub mod popovers;
pub mod progress;
pub mod scrollspy;
pub mod spinners;
pub mod toasts;
pub mod tooltips;

// Re-export all builders under components namespace to maintain compatibility
pub use buttons::{create_button, create_accent_button, create_fab};
pub use card::{create_card, create_title, create_subtitle, create_item_row, create_switch_card};
