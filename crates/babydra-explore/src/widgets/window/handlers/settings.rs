use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::{SessionState, ActivePane};

/// Connects the clicked event of the settings button to display the settings dialog and sync visual states on change.
pub fn wire_settings_button(
    btn_settings: &gtk4::Button,
    window: &gtk4::ApplicationWindow,
    navigate_pane_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    active_pane: Rc<Cell<ActivePane>>,
    session: Rc<RefCell<SessionState>>,
    preview_visible: Rc<Cell<bool>>,
    toggle_preview: Rc<dyn Fn()>,
) {
    let win = window.clone();
    let nav = navigate_pane_ref;
    let active = active_pane;
    let session_c = session;
    let preview_v = preview_visible;
    let toggle_p = toggle_preview;
    
    btn_settings.connect_clicked(move |_| {
        let nav_c = nav.clone();
        let act_c = active.clone();
        let session_cc = session_c.clone();
        let preview_vc = preview_v.clone();
        let toggle_p_c = toggle_p.clone();
        let parent_win = win.clone().upcast::<gtk4::Window>();
        crate::widgets::settings_dialog::show_settings_dialog(&parent_win, move || {
            let settings = babydra_common::load_explore_settings();
            
            // 1. Sync hidden files visibility setting in tab state
            let _hidden_changed = {
                let mut s = session_cc.borrow_mut();
                let tab = s.active_tab_mut();
                let old_val = tab.show_hidden;
                tab.show_hidden = settings.show_hidden;
                old_val != settings.show_hidden
            };

            // 2. Sync preview panel visibility
            let preview_changed = preview_vc.get() != settings.preview_visible;
            if preview_changed {
                toggle_p_c();
            }

            // 3. Trigger navigate refresh
            let path = session_cc.borrow().active_tab().current_path.clone();
            if let Some(ref f) = *nav_c.borrow() {
                f(act_c.get(), path);
            }
        });
    });
}
