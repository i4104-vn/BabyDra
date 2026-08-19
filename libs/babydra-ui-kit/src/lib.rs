//! BabyDra UI kit — reusable GTK4 components, UI helpers and shared styles.
//!
//! # Layout
//!
//! - [`components`] — widget builders (buttons, cards, modals, sliders, …)
//! - [`components::explore`] — file-manager feature components (dialogs,
//!   context menus, drag & drop, file items, rubberband selection)
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
        attach_hover_popover, build_hover_card, build_tray_gio_menu, clear_box, clear_list_box,
        close_tray_menu, create_accent_button, create_battery_icon, create_button, create_card,
        create_color_btn, create_colored_icon, create_css_card, create_danger_btn,
        create_danger_item, create_fab, create_footer_box, create_footer_btn, create_group_header,
        create_icon_badge, create_icon_btn, create_icon_button, create_list_row, create_menu_for,
        create_menu_full, create_menu_item, create_menu_popover, create_menu_sens, create_menu_sep,
        create_menu_shortcut, create_menu_text, create_placeholder, create_popover,
        create_popover_box, create_rssi_icon, create_scroll_list, create_sidebar_btn,
        create_sidebar_wbtn, create_square_tile, create_submenu_item, create_subtitle,
        create_switch, create_switch_card, create_sys_wifi_icon, create_title, create_toggle_tile,
        create_vpn_icon, create_wifi_icon, create_wifi_net_icon, create_wp_thumb, render_wifi_svg,
        set_tooltip, show_tray_menu, update_toggle_state, ContextMenuBuilder, CustomSlider,
        CustomSwitch, HoverPopoverRow, PasswordDialog, PlaceholderState, ToggleRow,
        VpnConfigDialog, VpnLogDialog, WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog,
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
        battery::{create_battery_area, draw_cairo_battery, get_battery_hex, get_battery_rgb},
        icon::resolver::get_resolved_icon,
        icon::{
            get_fallback_icon, get_icon, get_icon_colored, get_icon_from_svg, get_logo_png,
            set_fallback_icon, set_image_from_icon,
        },
        theme::{apply_theme_class, init_theme, is_dark_mode, set_dark_mode},
        window::{init_layer_window, setup_click_outside_dismiss},
    };
}
