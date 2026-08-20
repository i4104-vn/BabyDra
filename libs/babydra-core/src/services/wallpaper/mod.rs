//! Wallpaper and avatar management utilities.
//! Handles wallpaper persistence, background resolution, and avatar processing.

pub mod avatar;
pub mod wallpaper;

pub use avatar::{crop_circle, crop_square, get_avatar_bytes, set_avatar};
pub use wallpaper::{
    apply_greeter_wp, apply_wallpaper, get_greeter_wp_bytes, get_greeter_wp_css,
    get_local_wallpapers, get_wallpaper, get_wallpaper_dir, set_greeter_wp, set_wallpaper,
};
