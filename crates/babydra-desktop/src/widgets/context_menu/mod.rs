//! Context menu dispatcher for desktop background and file entries.

pub mod empty_menu;

pub use empty_menu::show_empty_menu;

use babydra_core::config::DesktopConfig;
use std::path::PathBuf;
use std::rc::Rc;

/// Adapts a plain refresh callback into the dialog-style `nav_callback` that simply refreshes.
pub(crate) fn refresh_nav_cb(refresh_cb: Rc<dyn Fn()>) -> Rc<dyn Fn(PathBuf)> {
    Rc::new(move |_| refresh_cb())
}

/// Loads the desktop config, applies `update`, and persists the result.
pub(crate) fn update_desktop_config(update: impl FnOnce(&mut DesktopConfig)) {
    let mut conf = babydra_core::config::load_desktop_config();
    update(&mut conf);
    babydra_core::config::save_desktop_config(&conf);
}
