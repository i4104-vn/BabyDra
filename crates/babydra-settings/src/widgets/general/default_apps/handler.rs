//! Default Applications event wiring.

use super::render::DefaultAppsWidgets;
use babydra_core::services::system::default_apps::{
    set_default_browser, set_default_file_manager, set_default_terminal, AppChoice,
};

/// Connects signals for Default Applications dropdowns.
pub fn wire_events(
    widgets: &DefaultAppsWidgets,
    browsers: Vec<AppChoice>,
    file_managers: Vec<AppChoice>,
    terminals: Vec<AppChoice>,
) {
    let b_list = browsers;
    widgets.browser_dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if let Some(c) = b_list.get(idx) {
            set_default_browser(&c.desktop_id);
        }
    });

    let fm_list = file_managers;
    widgets
        .file_manager_dropdown
        .connect_selected_notify(move |dd| {
            let idx = dd.selected() as usize;
            if let Some(c) = fm_list.get(idx) {
                set_default_file_manager(&c.desktop_id);
            }
        });

    let term_list = terminals;
    widgets
        .terminal_dropdown
        .connect_selected_notify(move |dd| {
            let idx = dd.selected() as usize;
            if let Some(c) = term_list.get(idx) {
                set_default_terminal(&c.desktop_id);
            }
        });
}
