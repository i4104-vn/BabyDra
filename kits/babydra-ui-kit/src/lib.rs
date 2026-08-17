//! BabyDra UI kit — reusable GTK4 components, UI helpers and shared styles.
//!
//! # Layout
//!
//! - [`components`] — widget builders (buttons, cards, modals, sliders, …)
//! - [`ui`] — theme, icons, animations, window and battery helpers
//! - [`prelude`] — one-stop re-export of the most commonly used API
//!
//! # Quick start
//!
//! ```rust,ignore
//! use babydra_ui_kit::prelude::*;
//!
//! init_theme();
//! let fab = create_fab("plus");
//! let row = create_switch_card("Dark mode", true);
//! ```

pub mod components;
pub mod ui;

/// One-stop import for the most commonly used BabyDra UI API.
///
/// Re-exports every component builder plus the frequently used UI helpers
/// (theme init, icons, animations, battery, windows). Modules remain
/// available for deeper access — this prelude only flattens the surface.
pub mod prelude {
    pub use crate::components::{
        attach_hover_popover, build_hover_popover_card, clear_box, clear_list_box,
        create_accent_button, create_battery_percentage_icon, create_button, create_card,
        create_card_with_class, create_colored_icon_button, create_colored_icon_widget, create_fab,
        create_icon_badge, create_icon_button, create_icon_label_button, create_list_row,
        create_placeholder_row, create_popover, create_popover_with_content,
        create_scrollable_list, create_sidebar_item_button, create_sidebar_item_button_with_widget,
        create_square_toggle_tile, create_subtitle, create_switch, create_switch_card,
        create_system_wifi_signal_icon, create_title, create_toggle_tile, create_vpn_shield_icon,
        create_wallpaper_thumbnail_icon, create_wifi_signal_icon,
        create_wifi_signal_icon_for_network, create_wifi_signal_icon_from_strength,
        render_wifi_signal_svg, set_tooltip, update_toggle_tile_state, CustomSlider, CustomSwitch,
        HoverPopoverRow, PasswordDialog, PlaceholderState, ToggleRow, VpnConfigDialog,
        VpnLogDialog, WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog,
    };
    pub use crate::ui::{
        animation::{
            easing::{
                ease_in_cubic, ease_in_out_cubic, ease_out_back, ease_out_cubic, ease_out_quart,
                linear,
            },
            genie::{genie_in, genie_out},
            island::{island_animate_size, island_animate_width, island_zoom_in, island_zoom_out},
            slide::{slide_in, slide_out, slide_out_cb, SlideDirection},
        },
        battery::{
            create_battery_drawing_area, draw_cairo_battery, get_battery_color_hex,
            get_battery_color_rgb,
        },
        icon::resolver::get_resolved_icon_path,
        icon::{
            get_icon, get_icon_colored, get_icon_from_svg, get_logo_png, get_system_or_file_icon,
            set_image_from_icon, set_system_or_file_icon,
        },
        theme::{apply_theme_class, init_theme, is_dark_mode, set_dark_mode},
        window::{init_layer_window, setup_click_outside_dismiss},
    };
}
